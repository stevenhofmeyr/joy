#![allow(dead_code)]

use crate::calibration::Calibration;
use crate::image::Image;
use anyhow::{bail, ensure, Context, Result};
use joycon_sys::input::*;
use joycon_sys::light;
use joycon_sys::mcu::ir::*;
use joycon_sys::mcu::*;
use joycon_sys::output::*;
use joycon_sys::spi::*;
use joycon_sys::*;

/// 200 samples per second with 3 sample per InputReport.
pub const IMU_SAMPLES_PER_SECOND: u32 = 200;
const WAIT_TIMEOUT: u32 = 60;

pub struct JoyCon {
    device: hidapi::HidDevice,
    info: hidapi::DeviceInfo,
    counter: u8,
    calib_gyro: Calibration,
    gyro_sens: imu::GyroSens,
    calib_accel: Calibration,
    accel_sens: imu::AccSens,
    pub max_raw_gyro: i16,
    pub max_raw_accel: i16,
    left_stick_calib: StickCalibration,
    right_stick_calib: StickCalibration,
    image: Image,
    pub enable_ir_loop: bool,
}

impl JoyCon {
    pub fn new(
        device: hidapi::HidDevice,
        info: hidapi::DeviceInfo,
        resolution: Resolution,
    ) -> Result<JoyCon> {
        assert!([
            JOYCON_L_BT,
            JOYCON_R_BT,
            PRO_CONTROLLER,
            JOYCON_CHARGING_GRIP,
        ]
        .contains(&info.product_id()));
        let mut joycon = JoyCon {
            device,
            info,
            counter: 0,
            // 10s with 3 reports at 60Hz
            calib_gyro: Calibration::new(10 * IMU_SAMPLES_PER_SECOND as usize),
            gyro_sens: imu::GyroSens::DPS2000,
            calib_accel: Calibration::new(10 * IMU_SAMPLES_PER_SECOND as usize),
            accel_sens: imu::AccSens::G8,
            max_raw_gyro: 0,
            max_raw_accel: 0,
            left_stick_calib: StickCalibration::default(),
            right_stick_calib: StickCalibration::default(),
            image: Image::new(resolution),
            enable_ir_loop: false,
        };

        joycon.set_report_mode_standard()?;
        Ok(joycon)
    }

    pub fn set_ir_callback(&mut self, cb: Box<dyn FnMut(Box<[u8]>, u32, u32)>) {
        self.image.set_cb(cb);
    }

    pub fn send(&mut self, report: &mut OutputReport) -> Result<()> {
        report.packet_counter = self.counter;
        self.counter = (self.counter + 1) & 0xf;
        let buffer = report.as_bytes();
        let nb_written = self.device.write(buffer)?;
        assert_eq!(nb_written, buffer.len());
        Ok(())
    }

    pub fn recv(&mut self) -> Result<InputReport> {
        // Larger buffer to detect unhandled received data
        let mut reports = [InputReport::new(); 2];
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(
                &mut reports as *mut _ as *mut u8,
                std::mem::size_of::<InputReport>(),
            )
        };
        let nb_read = self.device.read(buffer)?;
        let report = reports[0];
        report.validate();
        assert_eq!(nb_read, std::mem::size_of_val(&report));
        if let Some(mcu_report) = report.mcu_report() {
            if self.enable_ir_loop {
                for packet in &mut self.image.handle(&mcu_report) {
                    if let Some(mut packet) = packet {
                        self.send(&mut packet)?;
                    }
                }
            }
        }
        Ok(report)
    }

    pub fn load_calibration(&mut self) -> Result<()> {
        let factory_result = self.read_spi(RANGE_FACTORY_CALIBRATION_SENSORS)?;
        let factory_settings = factory_result.imu_factory_calib().unwrap();
        self.calib_accel.factory_offset = factory_settings.acc_offset();
        self.calib_gyro.factory_offset = factory_settings.gyro_offset();

        let user_result = self.read_spi(RANGE_USER_CALIBRATION_SENSORS)?;
        let user_settings = user_result.imu_user_calib().unwrap();
        self.calib_accel.user_offset = user_settings.acc_offset();
        self.calib_gyro.user_offset = user_settings.gyro_offset();

        let factory_result = self.read_spi(RANGE_FACTORY_CALIBRATION_STICKS)?;
        let factory_settings = factory_result.sticks_factory_calib().unwrap();
        let user_result = self.read_spi(RANGE_USER_CALIBRATION_STICKS)?;
        let user_settings = user_result.sticks_user_calib().unwrap();
        self.left_stick_calib = user_settings.left.calib().unwrap_or(factory_settings.left);
        self.right_stick_calib = user_settings
            .right
            .calib()
            .unwrap_or(factory_settings.right);

        Ok(())
    }

    pub fn set_imu_sens(&mut self) -> Result<()> {
        let gyro_sens = imu::GyroSens::DPS2000;
        let accel_sens = imu::AccSens::G8;
        self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::SetIMUSens,
            u: SubcommandRequestUnion {
                imu_sensitivity: imu::Sensitivity {
                    gyro_sens,
                    acc_sens: accel_sens,
                    ..imu::Sensitivity::default()
                },
            },
        })?;
        self.gyro_sens = gyro_sens;
        self.accel_sens = accel_sens;
        Ok(())
    }

    pub fn get_dev_info(&mut self) -> Result<DeviceInfo> {
        let reply = self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::RequestDeviceInfo,
            u: SubcommandRequestUnion { nothing: () },
        })?;
        Ok(*reply.device_info().unwrap())
    }

    pub fn set_home_light(&mut self, home_light: light::HomeLight) -> Result<()> {
        self.send_subcmd_wait(home_light)?;
        Ok(())
    }

    pub fn enable_imu(&mut self) -> Result<()> {
        self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::EnableIMU,
            u: SubcommandRequestUnion { imu_enabled: true },
        })?;
        Ok(())
    }

    pub fn set_report_mode_standard(&mut self) -> Result<()> {
        self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::SetInputReportMode,
            u: SubcommandRequestUnion {
                input_report_mode: InputReportId::StandardFull,
            },
        })?;
        Ok(())
    }

    pub fn set_report_mode_mcu(&mut self) -> Result<()> {
        self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::SetInputReportMode,
            u: SubcommandRequestUnion {
                input_report_mode: InputReportId::StandardFullMCU,
            },
        })?;
        Ok(())
    }

    pub fn enable_mcu(&mut self) -> Result<()> {
        self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::SetMCUState,
            u: SubcommandRequestUnion {
                mcu_mode: MCUMode::Standby,
            },
        })?;
        self.wait_mcu_status(MCUMode::Standby)
            .context("enable_mcu")?;
        Ok(())
    }

    fn wait_mcu_status(&mut self, mode: MCUMode) -> Result<MCUReport> {
        self.wait_mcu_cond(MCURequest::get_mcu_status(), |report| {
            report
                .as_status()
                .map(|status| status.state == mode)
                .unwrap_or(false)
        })
    }
    fn wait_mcu_cond<R: Into<MCURequest>>(
        &mut self,
        mcu_subcmd: R,
        mut f: impl FnMut(&MCUReport) -> bool,
    ) -> Result<MCUReport> {
        let mcu_subcmd = mcu_subcmd.into();
        // The MCU takes some time to warm up so we retry until we get an answer
        for _ in 0..WAIT_TIMEOUT {
            self.send_mcu_subcmd(mcu_subcmd)?;
            for _ in 0..WAIT_TIMEOUT {
                let in_report = self.recv()?;
                if let Some(mcu_report) = in_report.mcu_report() {
                    if f(mcu_report) {
                        return Ok(*mcu_report);
                    }
                }
            }
        }
        bail!("error getting the MCU status: timeout");
    }

    pub fn disable_mcu(&mut self) -> Result<()> {
        self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::SetMCUState,
            u: SubcommandRequestUnion {
                mcu_mode: MCUMode::Suspend,
            },
        })?;
        Ok(())
    }

    pub fn set_mcu_mode_ir(&mut self) -> Result<()> {
        let mcu_cmd = MCUCommand::set_mcu_mode(MCUMode::IR);
        self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::SetMCUConf,
            u: SubcommandRequestUnion { mcu_cmd },
        })?;
        self.wait_mcu_status(MCUMode::IR)
            .context("set_mcu_mode_ir")?;
        Ok(())
    }

    pub fn set_ir_image_mode(&mut self, frags: u8) -> Result<()> {
        let mut mcu_fw_version = Default::default();
        self.wait_mcu_cond(MCURequest::get_mcu_status(), |r| {
            if let Some(status) = r.as_status() {
                mcu_fw_version = (status.fw_major_version, status.fw_minor_version);
                true
            } else {
                false
            }
        })?;
        let mcu_cmd = MCUCommand::configure_ir(MCUIRModeData {
            ir_mode: MCUIRMode::ImageTransfer,
            no_of_frags: frags,
            mcu_fw_version,
        });
        let reply = self.send_subcmd_wait(mcu_cmd)?;
        ensure!(
            unsafe { reply.ir_status().0 } == MCUReportId::BusyInitializing,
            "mcu not busy"
        );

        let request = IRRequest {
            id: IRRequestId::GetState,
            u: IRRequestUnion { nothing: () },
        };
        self.wait_mcu_cond(request, |r| {
            r.as_ir_status()
                .map(|status| status.ir_mode == MCUIRMode::ImageTransfer)
                .unwrap_or(false)
        })
        .context("check sensor state")?;
        Ok(())
    }

    pub fn set_ir_registers(&mut self, regs: &[ir::Register]) -> Result<()> {
        let mut regs_mut = regs;
        while !regs_mut.is_empty() {
            let (mut report, remaining_regs) = OutputReport::set_registers(regs_mut);
            self.send(&mut report)?;
            regs_mut = remaining_regs;
            if !remaining_regs.is_empty() {
                // For packet drop purpose
                // TODO: not clean at all
                std::thread::sleep(std::time::Duration::from_millis(15));
            }
        }
        // TODO reg value doesn't change until next frame
        Ok(())
    }
    pub fn get_ir_registers(&mut self) -> Result<Vec<Register>> {
        let mut registers = vec![];
        for page in 0..=4 {
            let offset = 0;
            let nb_registers = 0x6f;
            let id = IRRequestId::ReadRegister;
            let request = IRRequest {
                id,
                u: IRRequestUnion {
                    read_registers: IRReadRegisters {
                        unknown_0x01: 0x01,
                        page,
                        offset,
                        nb_registers,
                    },
                },
            };
            let mcu_report = self
                .wait_mcu_cond(request, |mcu_report| {
                    if let Some(reg_slice) = mcu_report.as_ir_registers() {
                        reg_slice.page == page
                            && reg_slice.offset == offset
                            && reg_slice.nb_registers == nb_registers
                    } else {
                        false
                    }
                })
                .context("get IR registers slice")?;
            let reg_slice = mcu_report
                .as_ir_registers()
                .expect("already validated above");
            registers.extend(Register::decode_raw(
                page,
                offset,
                &reg_slice.values[..reg_slice.nb_registers as usize],
            ));
        }
        Ok(registers)
    }

    pub fn set_player_light(&mut self, player_lights: light::PlayerLights) -> Result<()> {
        self.send_subcmd_wait(player_lights)?;
        Ok(())
    }

    fn send_subcmd_wait<S: Into<SubcommandRequest>>(
        &mut self,
        subcmd: S,
    ) -> Result<SubcommandReply> {
        let subcmd = subcmd.into();
        let mut out_report = subcmd.into();

        self.send(&mut out_report)?;
        for _ in 0..WAIT_TIMEOUT {
            let in_report = self.recv()?;
            if let Some(reply) = in_report.subcmd_reply() {
                if reply.id() == Some(subcmd.subcommand_id) {
                    ensure!(reply.ack.is_ok(), "subcmd reply is nack");
                    return Ok(*reply);
                }
            }
        }

        bail!("Timeout while waiting for subcommand");
    }

    fn send_mcu_subcmd(&mut self, mcu_subcmd: MCURequest) -> Result<()> {
        let mut out_report = mcu_subcmd.into();
        self.send(&mut out_report)?;
        Ok(())
    }

    fn read_spi(&mut self, range: SPIRange) -> Result<SPIReadResult> {
        let reply = self.send_subcmd_wait(SubcommandRequest {
            subcommand_id: SubcommandId::SPIRead,
            u: SubcommandRequestUnion {
                spi_read: SPIReadRequest::new(range),
            },
        })?;
        let result = reply.spi_result().unwrap();
        ensure!(
            range == result.range(),
            "invalid range {:?}",
            result.range()
        );
        Ok(*result)
    }

    pub fn get_sticks(&mut self) -> Result<((f32, f32), (f32, f32))> {
        let report = self.recv()?;
        let inputs = report.standard().expect("should be standard");
        Ok((
            self.left_stick_calib
                .value_from_raw(inputs.left_stick.x(), inputs.left_stick.y()),
            self.right_stick_calib
                .value_from_raw(inputs.right_stick.x(), inputs.right_stick.y()),
        ))
    }

    pub fn get_gyro_rot_delta(&mut self, apply_calibration: bool) -> Result<[Vector3; 3]> {
        let report = self.recv()?;
        let gyro_frames = report.imu_frames().expect("no imu frame received");
        let offset = self
            .calib_gyro
            .user_offset
            .unwrap_or(self.calib_gyro.factory_offset);
        let mut out = [Vector3::default(); 3];
        // frames are from newest to oldest so we iter backward
        for (frame, out) in gyro_frames.iter().rev().zip(out.iter_mut()) {
            let max = [
                frame.raw_gyro().0.abs() as i16,
                frame.raw_gyro().1.abs() as i16,
                frame.raw_gyro().2.abs() as i16,
            ]
            .iter()
            .cloned()
            .max()
            .unwrap();
            self.max_raw_gyro = self.max_raw_gyro.max(max);
            if max > i16::MAX - 1000 {
                println!("saturation");
            }

            let gyro_rps = frame.gyro_rps(offset, self.gyro_sens) / IMU_SAMPLES_PER_SECOND as f32;
            *out = if apply_calibration {
                gyro_rps - self.calib_gyro.get_average()
            } else {
                gyro_rps
            }
        }
        Ok(out)
    }

    pub fn get_accel_delta_g(&mut self, apply_calibration: bool) -> Result<[Vector3; 3]> {
        let report = self.recv()?;
        let frames = report.imu_frames().expect("no imu frame received");
        let offset = self
            .calib_accel
            .user_offset
            .unwrap_or(self.calib_accel.factory_offset);
        let mut out = [Vector3::default(); 3];
        // frames are from newest to oldest so we iter backward
        for (frame, out) in frames.iter().rev().zip(out.iter_mut()) {
            let max = [
                frame.raw_accel().0.abs() as i16,
                frame.raw_accel().1.abs() as i16,
                frame.raw_accel().2.abs() as i16,
            ]
            .iter()
            .cloned()
            .max()
            .unwrap();
            self.max_raw_accel = self.max_raw_accel.max(max);
            if max > i16::MAX - 1000 {
                println!("saturation");
            }

            let accel_g = frame.accel_g(offset, self.accel_sens);
            *out = if apply_calibration {
                accel_g - self.calib_accel.get_average()
            } else {
                accel_g
            }
        }
        Ok(out)
    }

    pub fn reset_calibration(&mut self) -> Result<()> {
        // seems needed
        self.get_gyro_rot_delta(false)?;
        self.calib_gyro.reset();
        for _ in 0..60 {
            for frame in &self.get_gyro_rot_delta(false)? {
                self.calib_gyro.push(*frame);
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for JoyCon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("JoyCon")
            .field("manufacturer", &self.info.manufacturer_string())
            .field("product", &self.info.product_string())
            .field("serial", &self.info.serial_number())
            .finish()
    }
}
