use anyhow::Context;
use bluetooth_sys::*;
use joycon::{
    hidapi::HidDevice,
    joycon_sys::{
        input::Stick, output::SubcommandRequestEnum, spi::RightStickCalibration, InputReport, InputReportId::StandardFull,
        OutputReport,
    },
    JoyCon,
};
use socket2::{SockAddr, Socket};
use std::{
    convert::TryInto,
    ffi::CString,
    fs::{File, OpenOptions},
    intrinsics::transmute,
    io::Write,
    mem::{size_of_val, zeroed, MaybeUninit},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::opts::Relay;

const RIGHT_BUTTONS_BYTE: usize = 4;
const MIDDLE_BUTTONS_BYTE: usize = 5;
const LEFT_BUTTONS_BYTE: usize = 6;
const LEFT_STICK_BYTE: usize = 7;
const RIGHT_STICK_BYTE: usize = 10;

pub fn relay(device: HidDevice, opts: &Relay, mut joycon: JoyCon) -> anyhow::Result<()> {
    let mut output = opts
        .output
        .as_ref()
        .map(|path| {
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
                .context("opening the log file")
        })
        .transpose()?;
    let (mut _client_ctl, mut client_itr) = connect_switch(&opts.address)?;

    joycon.enable_imu()?;
    joycon.load_calibration()?;
    joycon.enable_ringcon()?;

    // Force input reports to be generated so that we don't have to manually click on a button.
    device.write(OutputReport::from(SubcommandRequestEnum::SetInputReportMode(StandardFull.into())).as_bytes())?;

    let mut right_gyro_file = File::create("right_gyro.dat")?;
    writeln!(right_gyro_file, "accel_x accel_y accel_z gyro_x gyro_y gyro_z flex")?;
    let mut button_a_count = 0;
    let mut loop_num: i64 = 0;

    let start = Instant::now();
    loop {
        {
            let mut buf = [0; 500];
            buf[0] = 0xa1;
            let len = device.read_timeout(&mut buf[1..], 0).context("joycon recv")?;
            if len > 0 {
                let mut report = InputReport::new();
                let raw_report = report.as_bytes_mut();
                raw_report.copy_from_slice(&buf[1..raw_report.len() + 1]);
                if report.try_validate() {
                    remap_right_joycon(
                        &mut joycon,
                        report,
                        &mut buf,
                        len,
                        loop_num,
                        &right_gyro_file,
                        &mut button_a_count,
                    );
                }

                let elapsed = start.elapsed().as_secs_f64();

                if let Some(subcmd) = report.subcmd_reply() {
                    println!("{:0>9.4} {:?}", elapsed, subcmd);
                } else if let Some(mcu) = report.mcu_report() {
                    println!("{:0>9.4} {:?}", elapsed, mcu);
                }

                if let Some(ref mut out) = output {
                    writeln!(out, "> {:0>9.4} {}", elapsed, hex::encode(&buf[1..len + 1]))?;
                }

                if let Err(e) = client_itr.send(&buf[..len + 1]) {
                    if e.raw_os_error() == Some(107) {
                        eprintln!("Reconnecting the switch");
                        let x = connect_switch(&opts.address)?;
                        _client_ctl = x.0;
                        client_itr = x.1;

                        // Force input reports to be generated so that we don't have to manually click on a button.
                        device
                            .write(OutputReport::from(SubcommandRequestEnum::SetInputReportMode(StandardFull.into())).as_bytes())?;
                    }
                }
            }
        }
        {
            let mut buf = [MaybeUninit::uninit(); 500];
            if let Ok(len) = client_itr.recv(&mut buf).context("switch recv") {
                if len > 0 {
                    let buf: [u8; 500] = unsafe { transmute(buf) };
                    let mut report = OutputReport::new();
                    let raw_report = report.as_bytes_mut();
                    raw_report.copy_from_slice(&buf[1..raw_report.len() + 1]);

                    let elapsed = start.elapsed().as_secs_f64();

                    if let Some(subcmd) = report.rumble_subcmd() {
                        println!("{:0>9.4} {:?}", elapsed, subcmd);
                    } else if let Some(mcu) = report.request_mcu_data() {
                        println!("{:0>9.4} {:?}", elapsed, mcu);
                    }

                    if let Some(ref mut out) = output {
                        writeln!(out, "< {:0>9.4} {}", elapsed, hex::encode(&buf[1..len + 1]))?;
                    }

                    device.write(&buf[1..len]).context("joycon send")?;
                }
            }
        }
        loop_num += 1;
        sleep(Duration::from_millis(1))
    }
}

fn remap_right_joycon(
    joycon: &mut JoyCon,
    report: InputReport,
    buf: &mut [u8],
    len: usize,
    loop_num: i64,
    mut gyro_file: &File,
    button_a_count: &mut i32,
) -> bool {
    let joycon_report = joycon.get_joycon_report(report).unwrap();
    let buttons_str = format!("{}", joycon_report.buttons);
    if buttons_str != "" || loop_num == 0 {
        let right_stick = Stick {
            data: [buf[RIGHT_STICK_BYTE], buf[RIGHT_STICK_BYTE + 1], buf[RIGHT_STICK_BYTE + 2]],
        };
        let stick_pos = joycon.right_stick_calib.value_from_raw(
            joycon.right_stick_calib.conv_x(right_stick.data),
            joycon.right_stick_calib.conv_y(right_stick.data),
        );
        eprintln!(
            "{} {:X} {:X} {:X} {:.2} {:.2}",
            joycon_report.buttons,
            buf[RIGHT_BUTTONS_BYTE],
            buf[MIDDLE_BUTTONS_BYTE],
            buf[LEFT_BUTTONS_BYTE],
            stick_pos.x,
            stick_pos.y
        );
    }
    // testing: convert R to ZR
    if buf[RIGHT_BUTTONS_BYTE] & 64 > 0 {
        eprintln!("R pressed");
        buf[RIGHT_BUTTONS_BYTE] |= 128;
        //buf[ReportByteIndexes::RightButtons as usize] &= !64;
        return true;
    }
    // testing: convert PLUS to stick x left?
    if buf[MIDDLE_BUTTONS_BYTE] & 2 > 0 {
        buf[MIDDLE_BUTTONS_BYTE] &= !2;
        let (min_x, min_y) = joycon.right_stick_calib.min();
        let min_x_bytes = min_x.to_be_bytes();
        buf[RIGHT_STICK_BYTE] = min_x_bytes[0];
        buf[RIGHT_STICK_BYTE + 1] = min_x_bytes[1];
        return true;
    }
    false
}

fn connect_switch(address: &str) -> anyhow::Result<(Socket, Socket)> {
    let client_ctl = Socket::new(
        (AF_BLUETOOTH as i32).into(),
        (__socket_type_SOCK_SEQPACKET as i32).into(),
        Some((BTPROTO_L2CAP as i32).into()),
    )?;
    let client_itr = Socket::new(
        (AF_BLUETOOTH as i32).into(),
        (__socket_type_SOCK_SEQPACKET as i32).into(),
        Some((BTPROTO_L2CAP as i32).into()),
    )?;

    unsafe {
        let ctl_addr = create_sockaddr(address, 17)?;
        client_ctl.connect(&ctl_addr).context("error connecting psm 17")?;
        client_ctl.set_nonblocking(true).context("non blocking error")?;

        let itr_addr = create_sockaddr(address, 19)?;
        client_itr.connect(&itr_addr).context("error connecting psm 17")?;
        client_itr.set_nonblocking(true).context("non blocking error")?;
    }

    Ok((client_ctl, client_itr))
}

unsafe fn create_sockaddr(address: &str, psm: u16) -> Result<SockAddr, anyhow::Error> {
    Ok(SockAddr::init(|storage, len| {
        let storage = &mut *storage.cast::<sockaddr_l2>();
        *len = size_of_val(storage) as u32;
        *storage = sockaddr_l2 {
            l2_family: AF_BLUETOOTH.try_into().unwrap(),
            // todo: watch out endian
            l2_psm: psm.to_le(),
            ..zeroed()
        };
        let sa = CString::new(address)?;
        str2ba(sa.as_ptr(), &mut storage.l2_bdaddr);
        Ok(())
    })?
    .1)
}
