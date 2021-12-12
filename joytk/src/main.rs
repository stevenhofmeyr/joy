use anyhow::{Context, Result};
use cgmath::{Vector2, Vector3};
use clap::Clap;
use colored::Colorize;
use joycon::{
    hidapi::{DeviceInfo, HidApi},
    joycon_sys::{
        accessory::AccessoryCommand,
        input::{BatteryLevel, InputReportEnum, Stick, UseSPIColors, WhichController},
        light::{self, PlayerLight},
        mcu::ir::Resolution,
        output::OutputReportEnum,
        spi::{ControllerColor, SPIRange, SensorCalibration, SticksCalibration, UserSensorCalibration, UserSticksCalibration},
        InputReport, OutputReport,
    },
    JoyCon, Report,
};
use std::{
    convert::TryFrom,
    fs::{File, OpenOptions},
    io::{BufRead, Write},
    time::Duration,
};
use std::{thread::sleep, time::Instant};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

mod camera;
#[cfg(feature = "interface")]
mod interface;
mod opts;
#[cfg(target_os = "linux")]
mod relay;

use opts::*;

fn main() -> Result<()> {
    let formatter = tracing_subscriber::fmt()
        .with_span_events(if std::env::var("LOG_TIMING").is_ok() {
            FmtSpan::CLOSE
        } else {
            FmtSpan::NONE
        })
        .with_env_filter(EnvFilter::from_default_env());
    if std::env::var("LOG_PRETTY").is_ok() {
        formatter.pretty().init();
    } else {
        formatter.init();
    }

    let opts = Opts::parse();

    if let SubCommand::Decode = opts.subcmd {
        return decode();
    }

    #[cfg(feature = "interface")]
    if let SubCommand::Tui = opts.subcmd {
        return interface::run();
    }

    let mut api = HidApi::new()?;

    let mut left_dev_info_opt = None;
    let mut right_dev_info_opt = None;
    let mut i = 0;

    loop {
        for device_info in api.device_list() {
            if device_info.product_string() == Some("Joy-Con (L)") && left_dev_info_opt.is_none() {
                eprintln!("\nFound {:?}", device_info.product_string().unwrap());
                left_dev_info_opt = Some(device_info.clone());
            } else if device_info.product_string() == Some("Joy-Con (R)") && right_dev_info_opt.is_none() {
                eprintln!("\nFound {:?}", device_info.product_string().unwrap());
                right_dev_info_opt = Some(device_info.clone());
            }
        }
        if !right_dev_info_opt.is_none() && !left_dev_info_opt.is_none() {
            break;
        } else {
            if i == 0 {
                eprintln!(
                    "Error: didn't find both joycons: left {} right {}",
                    !left_dev_info_opt.is_none(),
                    !right_dev_info_opt.is_none()
                );
            }
            if !opts.wait {
                std::process::exit(1);
            } else {
                if i == 0 {
                    eprint!("Waiting for 2s for joycons to be connected");
                } else {
                    eprint!(".");
                }
                api.refresh_devices()?;
                sleep(Duration::from_millis(2000));
            }
        }
        i += 1;
    }

    if let SubCommand::Relay(ref r) = opts.subcmd {
        eprintln!("Relay to switch...");
        let right_dev_info = right_dev_info_opt.unwrap();
        let right_device = right_dev_info
            .open_device(&api)
            .with_context(|| format!("error opening the HID device {:?}", right_dev_info))?;
        relay::relay(right_device, r).context("error during the relay")?;
    } else {
        let left_joycon = get_joycon(left_dev_info_opt.unwrap(), &api)?;
        let right_joycon = get_joycon(right_dev_info_opt.unwrap(), &api)?;
        hid_main(left_joycon, right_joycon, &opts).context("error running the command")?;
    }
    Ok(())
}

fn get_joycon(device_info: DeviceInfo, api: &HidApi) -> Result<JoyCon> {
    let device = device_info
        .open_device(&api)
        .with_context(|| format!("error opening the HID device {:?}", device_info))?;
    let joycon = JoyCon::new(device, device_info.clone())?;
    Ok(joycon)
}

fn hid_init_joycon(mut joycon: JoyCon, lbl: &str) -> Result<JoyCon> {
    joycon.set_home_light(light::HomeLight::new(0x8, 0x2, 0x0, &[(0xf, 0xf, 0), (0x2, 0xf, 0)]))?;
    let battery_level = joycon.tick()?.info.battery_level();
    eprintln!("{} Battery level is {:?}", lbl, battery_level);
    joycon.set_player_light(light::PlayerLights::new(
        (battery_level >= BatteryLevel::Full).into(),
        (battery_level >= BatteryLevel::Medium).into(),
        (battery_level >= BatteryLevel::Low).into(),
        if battery_level >= BatteryLevel::Low {
            PlayerLight::On
        } else {
            PlayerLight::Blinking
        },
    ))?;
    Ok(joycon)
}

fn hid_main(mut left_joycon: JoyCon, mut right_joycon: JoyCon, opts: &Opts) -> Result<()> {
    left_joycon = hid_init_joycon(left_joycon, "Left joycon")?;
    right_joycon = hid_init_joycon(right_joycon, "Right joycon")?;
    match opts.subcmd {
        SubCommand::Calibrate(ref calib) => match calib.subcmd {
            CalibrateE::Sticks => {
                calibrate_sticks(&mut left_joycon)?;
                calibrate_sticks(&mut right_joycon)?;
            }
            CalibrateE::Gyroscope => {
                calibrate_gyro(&mut left_joycon, "left")?;
                calibrate_gyro(&mut right_joycon, "right")?;
            }
            CalibrateE::Reset => {
                reset_calibration(&mut left_joycon)?;
                reset_calibration(&mut right_joycon)?;
            }
        },
        SubCommand::Get => {
            get(&mut left_joycon)?;
            get(&mut right_joycon)?;
        }
        SubCommand::Monitor => monitor(&mut left_joycon, &mut right_joycon)?,

        SubCommand::Ringcon(ref cmd) => ringcon(&mut right_joycon, cmd)?,
        SubCommand::PulseRate => pulse_rate(&mut right_joycon)?,
        SubCommand::Set(ref set) => match set.subcmd {
            SetE::Color(ref arg) => set_color(&mut right_joycon, arg)?,
        },
        SubCommand::Dump => dump(&mut right_joycon)?,
        SubCommand::Restore => restore(&mut right_joycon)?,
        SubCommand::Decode | SubCommand::Relay(_) => unreachable!(),
        #[cfg(feature = "interface")]
        SubCommand::Tui => unreachable!(),
        SubCommand::Camera => camera::run(right_joycon)?,
    }
    Ok(())
}

const WIN_SIZE: usize = 30;

fn monitor(left_joycon: &mut JoyCon, right_joycon: &mut JoyCon) -> Result<()> {
    //calibrate_gyro(left_joycon, "left")?;
    left_joycon.enable_imu()?;
    left_joycon.load_calibration()?;
    //calibrate_gyro(right_joycon, "right")?;
    right_joycon.enable_imu()?;
    right_joycon.load_calibration()?;

    right_joycon.enable_ringcon()?;

    let mut left_gyro_file = File::create("left_gyro.dat")?;
    let mut right_gyro_file = File::create("right_gyro.dat")?;

    writeln!(
        left_gyro_file,
        "accel_x accel_y accel_z maxa_x maxa_y maxa_z gyro_x gyro_y gyro_z maxg_x maxg_y maxg_z"
    )?;

    writeln!(right_gyro_file, "accel_x accel_y accel_z gyro_x gyro_y gyro_z flex")?;

    let mut accel_window: Vector3<[f64; WIN_SIZE]> = Vector3 {
        x: [0.0; WIN_SIZE],
        y: [0.0; WIN_SIZE],
        z: [0.0; WIN_SIZE],
    };
    let mut gyro_window: Vector3<[f64; WIN_SIZE]> = Vector3 {
        x: [0.0; WIN_SIZE],
        y: [0.0; WIN_SIZE],
        z: [0.0; WIN_SIZE],
    };
    let mut squat_steps = 0;
    let mut squatting = false;
    let mut now = Instant::now();

    loop {
        let left_report = left_joycon.tick()?;
        let right_report = right_joycon.tick()?;
        // NOTE: the default update interval for the joycons is 15ms (66Hz) and the pro controller is 8ms (120Hz)
        if now.elapsed() > Duration::from_millis(1000 / 66) {
            now = Instant::now();
            monitor_left_joycon(
                left_report,
                &left_gyro_file,
                &mut accel_window,
                &mut gyro_window,
                &mut squat_steps,
                &mut squatting,
            )?;
            monitor_right_joycon(right_report, &right_gyro_file)?;
            // the reader on the other side of the pipe reads by line, so this ensures it gets the latest update all as one
            println!("");
            std::io::stdout().flush()?;
        }
    }
}

fn monitor_left_joycon(
    report: Report,
    mut f: &File,
    accel_window: &mut Vector3<[f64; WIN_SIZE]>,
    gyro_window: &mut Vector3<[f64; WIN_SIZE]>,
    squat_steps: &mut i32,
    squatting: &mut bool,
) -> Result<()> {
    print!("{}", report.buttons);

    let frame = &report.imu.unwrap()[2];
    let accel = frame.accel;
    let gyro = frame.gyro;
    let max_accel_x = max_magnitude(&mut (*accel_window).x, accel.x);
    let max_accel_y = max_magnitude(&mut (*accel_window).y, accel.y);
    let max_accel_z = max_magnitude(&mut (*accel_window).z, accel.z);
    let max_gyro_x = max_magnitude(&mut (*gyro_window).x, gyro.x);
    let max_gyro_y = max_magnitude(&mut (*gyro_window).y, gyro.y);
    let max_gyro_z = max_magnitude(&mut (*gyro_window).z, gyro.z);
    write!(
        f,
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} ",
        accel.x, accel.y, accel.z, max_accel_x, max_accel_y, max_accel_z
    )?;
    writeln!(
        f,
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}",
        gyro.x, gyro.y, gyro.z, max_gyro_x, max_gyro_y, max_gyro_z
    )?;

    let mut stick: Vector2<f64> = Vector2 { x: 0.0, y: 0.0 };
    let mut squat_track = false;
    if accel.x > 1.1 {
        eprintln!("jump");
        print!("BUTTON,X ");
    } else if max_accel_x > 2.5 {
        // if you run fast enough you come out of sneak
        if *squatting {
            // press B to unsneak
            print!("BUTTON,B ");
            eprintln!("unsneak");
            *squatting = false;
        }
        // running
        stick.y = 1.0;
        if max_accel_x > 3.0 {
            // sprinting
            eprintln!("sprint");
            print!("BUTTON,B ");
        }
    } else if max_accel_x > 1.5 {
        // walking
        stick.y = max_accel_x - 1.0;
    } else {
        if accel.x > -0.6 && !*squatting {
            if *squat_steps > 5 {
                // only press once to start sneaking, and then remain sneaking
                print!("BUTTON,L_STICK_PRESS ");
                eprintln!("sneak");
                *squatting = true;
            } else {
                *squat_steps += 1;
                squat_track = true;
            }
        }
        // can we leave this enabled? It could get bumped or shaken?
        stick = report.left_stick;
    }
    if !squat_track {
        *squat_steps = 0;
    }
    print!("STICK,LEFT,{:.2},{:.2} ", stick.x, stick.y);
    Ok(())
}

fn monitor_right_joycon(report: Report, mut f: &File) -> Result<()> {
    print!("{}", report.buttons);

    let frame = &report.imu.unwrap()[2];
    let accel = frame.accel;
    let gyro = frame.gyro;

    let mut bow_pull = false;

    let flex = report.raw.imu_frames().unwrap()[2].raw_ringcon();
    if flex > 300 && flex < 2000 {
        //eprintln!("pulling bow");
        print!("BUTTON,ZR ");
        bow_pull = true;
    } else if flex > 3000 {
        // pushing attacks
        //eprintln!("attack!");
        print!("BUTTON,Y ");
    }
    writeln!(
        f,
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} {}",
        accel.x, accel.y, accel.z, gyro.x, gyro.y, gyro.z, flex
    )?;
    let mut stick: Vector2<f64> = Vector2 { x: 0.0, y: 0.0 };
    if bow_pull {
        // point bow (rotate ringcon for up/down, tilt for left/right)
        if accel.z < -0.8 {
            stick.x = 1.0;
        } else if accel.z > 0.8 {
            stick.x = -1.0;
        } else if accel.y < 0.65 {
            if accel.x < -0.4 {
                stick.y = 1.0;
            } else if accel.x > 0.4 {
                stick.y = -1.0;
            }
        }
    } else {
        // adjust viewpoint (rotate ringcon for left/right, tilt for up/down)
        if accel.z < -0.8 {
            stick.y = 1.0;
        } else if accel.z > 0.8 {
            stick.y = -1.0;
        } else if accel.y < 0.65 {
            if accel.x < -0.4 {
                stick.x = -1.0;
            } else if accel.x > 0.4 {
                stick.x = 1.0;
            }
        } else {
            stick = report.right_stick;
        }
    }
    print!("STICK,RIGHT,{:.2},{:.2} ", stick.x, stick.y);
    Ok(())
}

fn max_magnitude(window_vals: &mut [f64; WIN_SIZE], new_val: f64) -> f64 {
    (*window_vals).rotate_left(1);
    (*window_vals)[WIN_SIZE - 1] = new_val.abs();
    (*window_vals).iter().cloned().fold((*window_vals)[0], f64::max)
}

fn decode() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut image = joycon::Image::new();
    image.change_resolution(Resolution::R320x240);
    for line in stdin.lock().lines() {
        let line = line?;
        let fragments: Vec<&str> = line.split(" ").collect();
        let side = fragments[0];
        let time = fragments[1];
        let hex = hex::decode(&fragments[2][2..])?;
        if side == ">" {
            let mut report = InputReport::new();
            let raw_report = report.as_bytes_mut();
            let len = raw_report.len().min(hex.len());
            raw_report[..len].copy_from_slice(&hex[..len]);
            match InputReportEnum::try_from(report) {
                Ok(InputReportEnum::StandardAndSubcmd((_, subcmd))) => {
                    println!("{} {}", time.blue(), format!("{:?}", subcmd).green());
                }
                Ok(InputReportEnum::StandardFullMCU((_, _, mcu))) => {
                    println!("{} {:?}", time.blue(), mcu);
                    image.handle(&mcu);
                    if let Some(img) = image.last_image.take() {
                        img.save("/tmp/out.png")?;
                        dbg!("new image");
                    }
                }
                _ => {}
            }
        } else {
            let mut report = OutputReport::new();
            let raw_report = report.as_bytes_mut();
            let len = raw_report.len().min(hex.len());
            raw_report[..len].copy_from_slice(&hex[..len]);
            match OutputReportEnum::try_from(report) {
                Ok(OutputReportEnum::RumbleAndSubcmd(subcmd)) => {
                    println!("{} {}", time.blue(), format!("{:?}", subcmd).red());
                }
                Ok(OutputReportEnum::RequestMCUData(mcu)) => {
                    println!("{} {}", time.blue(), format!("{:?}", mcu).yellow());
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn ringcon(joycon: &mut JoyCon, cmd: &Ringcon) -> anyhow::Result<()> {
    println!("Ringcon initialisation...");
    joycon.enable_ringcon()?;
    match cmd.subcmd {
        RingconE::StoredFlex => {
            println!(
                "Stored steps: {}",
                joycon
                    .call_subcmd_wait(AccessoryCommand::get_offline_steps())?
                    .maybe_accessory()
                    .unwrap()
                    .offline_steps()?
                    .steps
            );
        }
        RingconE::Monitor => loop {
            let report = joycon.recv()?;
            let frames = report.imu_frames().unwrap();
            println!("Flex value: {}", frames[2].raw_ringcon());
        },
        RingconE::Exp => {}
    }
    Ok(())
}

fn pulse_rate(joycon: &mut JoyCon) -> Result<()> {
    joycon.enable_pulserate()?;
    eprintln!("pote");
    let mut i = 0;
    loop {
        let report = joycon.recv()?;
        if let Some(mcu) = report.mcu_report() {
            if let Some(frame) = mcu.ir_data() {
                if let Some(img) = image::GrayImage::from_raw(11, 27, frame.img_fragment[0..11 * 27].to_vec()) {
                    img.save(format!("/tmp/pulse{:05}.png", i))?;
                    i += 1;
                }
            }
        }
    }
}

fn reset_calibration(joycon: &mut JoyCon) -> Result<()> {
    joycon.write_spi(UserSensorCalibration::reset())?;
    // TODO: reset sticks
    Ok(())
}

fn dump(joycon: &mut JoyCon) -> Result<()> {
    let mut out = OpenOptions::new().write(true).truncate(true).open("/tmp/out.raw")?;
    let mut offset = 0;
    let size = 0x1D;
    // TODO: why value
    let mut last_percent = 0;
    while offset + (size as u32) < 0x80000 {
        let percent = offset * 100 / 0x80000;
        if last_percent != percent {
            println!("{}%", percent);
            last_percent = percent;
        }
        let slice = joycon.read_spi_raw(unsafe { SPIRange::new(offset, size) })?;
        out.write(&slice)?;
        offset += size as u32;
    }
    let last_size = (0x80000 - offset) as u8;
    let slice = joycon.read_spi_raw(unsafe { SPIRange::new(offset, last_size) })?;
    out.write(&slice)?;
    Ok(())
}

fn restore(_joycon: &mut JoyCon) -> Result<()> {
    unimplemented!()
}

fn calibrate_gyro(joycon: &mut JoyCon, side: &str) -> Result<()> {
    joycon.enable_imu()?;
    eprintln!("Calibrating {}: don't move the controller...", side);
    sleep(Duration::from_secs(1));

    let mut gyro_reports = Vec::new();
    let mut acc_reports = Vec::new();
    for i in (0..10).rev() {
        eprint!("{}, ", i);
        std::io::stdout().flush()?;
        let now = Instant::now();
        while now.elapsed() < Duration::from_secs(1) {
            let report = joycon.tick()?;
            gyro_reports.extend(report.raw.imu_frames().unwrap().iter().map(|x| x.raw_gyro()));
            acc_reports.extend(report.raw.imu_frames().unwrap().iter().map(|x| x.raw_accel()));
        }
    }
    eprintln!();
    let gyro_avg = gyro_reports.iter().sum::<Vector3<f64>>() / gyro_reports.len() as f64;

    let factory: SensorCalibration = joycon.read_spi()?;
    let user: UserSensorCalibration = joycon.read_spi()?;
    let mut calib = user.calib().unwrap_or(factory);
    calib.set_gyro_offset(gyro_avg);

    eprintln!("Writing calibration data {:x?}...", calib);
    joycon.write_spi(UserSensorCalibration::from(calib))?;
    eprintln!("...done");

    Ok(())
}

fn calibrate_sticks(joycon: &mut JoyCon) -> Result<()> {
    eprintln!("Don't move the sticks...");
    sleep(Duration::from_secs(1));
    let (left_neutral, right_neutral) = raw_sticks(joycon)?;

    eprintln!("Move the sticks then press A...");
    let mut l_x_min = left_neutral.x();
    let mut l_x_max = left_neutral.x();
    let mut l_y_min = left_neutral.y();
    let mut l_y_max = left_neutral.y();
    let mut r_x_min = right_neutral.x();
    let mut r_x_max = right_neutral.x();
    let mut r_y_min = right_neutral.y();
    let mut r_y_max = right_neutral.y();

    loop {
        let report = joycon.tick()?;
        let (left_stick, right_stick) = raw_sticks(joycon)?;

        if report.buttons.right.a() {
            break;
        }

        l_x_min = l_x_min.min(left_stick.x());
        l_x_max = l_x_max.max(left_stick.x());
        l_y_min = l_y_min.min(left_stick.y());
        l_y_max = l_y_max.max(left_stick.y());
        r_x_min = r_x_min.min(right_stick.x());
        r_x_max = r_x_max.max(right_stick.x());
        r_y_min = r_y_min.min(right_stick.y());
        r_y_max = r_y_max.max(right_stick.y());
    }

    dbg!((l_x_min, left_neutral.x(), l_x_max));
    dbg!((l_y_min, left_neutral.y(), l_y_max));
    // TODO: write calibration

    Ok(())
}

fn raw_sticks(joycon: &mut JoyCon) -> Result<(Stick, Stick)> {
    let report = joycon.recv()?;
    let std_report = report.standard().expect("should be standard");
    Ok((std_report.left_stick, std_report.right_stick))
}

fn get(joycon: &mut JoyCon) -> Result<()> {
    let dev_info = joycon.get_dev_info()?;
    println!(
        "{}, MAC {}, firmware version {}",
        dev_info.which_controller, dev_info.mac_address, dev_info.firmware_version
    );
    println!();

    println!("Controller color:");
    let color: ControllerColor = joycon.read_spi()?;
    println!("  body: {}", color.body);
    println!("  buttons: {}", color.buttons);
    if dev_info.use_spi_colors == UseSPIColors::IncludingGrip {
        println!("  left grip: {}", color.left_grip);
        println!("  right grip: {}", color.right_grip);
    }
    println!();

    let imu_factory_settings: SensorCalibration = joycon.read_spi()?;
    let imu_user_settings: UserSensorCalibration = joycon.read_spi()?;

    println!("Gyroscope calibration data:");
    println!(
        "  factory: offset ({:?}), factor ({:?})",
        imu_factory_settings.gyro_offset().cast::<i16>().unwrap(),
        imu_factory_settings.gyro_factor().cast::<u16>().unwrap(),
    );
    if let Some(settings) = imu_user_settings.calib() {
        println!(
            "  user: offset ({:?}), factor ({:?})",
            settings.gyro_offset().cast::<i16>().unwrap(),
            settings.gyro_factor().cast::<u16>().unwrap(),
        );
    } else {
        println!("  no user");
    }
    println!("");
    println!("Accelerometer calibration data:");
    println!(
        "  factory: offset ({:?}), factor ({:?})",
        imu_factory_settings.acc_offset().cast::<i16>().unwrap(),
        imu_factory_settings.acc_factor().cast::<u16>().unwrap(),
    );
    if let Some(settings) = imu_user_settings.calib() {
        println!(
            "  user: offset ({:?}), factor ({:?})",
            settings.acc_offset().cast::<i16>().unwrap(),
            settings.acc_factor().cast::<u16>().unwrap(),
        );
    } else {
        println!("  no user");
    }
    println!("");

    let sticks_factory_settings: SticksCalibration = joycon.read_spi()?;
    let sticks_user_settings: UserSticksCalibration = joycon.read_spi()?;
    println!("Left stick calibration data");
    println!(
        "  factory: min {:x?}, center {:x?}, max {:x?}",
        sticks_factory_settings.left.min(),
        sticks_factory_settings.left.center(),
        sticks_factory_settings.left.max()
    );
    if let Some(left) = sticks_user_settings.left.calib() {
        println!(
            "  user: min {:x?}, center {:x?}, max {:x?}",
            left.min(),
            left.center(),
            left.max()
        );
    } else {
        println!("  no user");
    }
    println!("");
    println!("Right stick calibration data");
    println!(
        "  factory: min {:x?}, center {:x?}, max {:x?}",
        sticks_factory_settings.right.min(),
        sticks_factory_settings.right.center(),
        sticks_factory_settings.right.max()
    );
    if let Some(right) = sticks_user_settings.right.calib() {
        println!(
            "  user: min {:x?}, center {:x?}, max {:x?}",
            right.min(),
            right.center(),
            right.max()
        );
    } else {
        println!("  no user");
    }
    println!("");

    Ok(())
}

fn set_color(joycon: &mut JoyCon, arg: &SetColor) -> Result<()> {
    let dev_info = joycon.get_dev_info()?;
    let is_pro_controller = dev_info.which_controller == WhichController::ProController;

    let mut colors = ControllerColor {
        body: arg.body.parse()?,
        buttons: arg.buttons.parse()?,
        ..Default::default()
    };
    if let (Some(left), Some(right)) = (arg.left_grip.as_ref(), arg.right_grip.as_ref()) {
        if is_pro_controller {
            colors.left_grip = left.parse()?;
            colors.right_grip = right.parse()?;
            if dev_info.use_spi_colors != UseSPIColors::IncludingGrip {
                joycon.write_spi(UseSPIColors::IncludingGrip)?;
            }
        } else {
            panic!("grips can only be set on pro controller");
        }
    }
    println!("Setting controller colors to {:x?}", colors);
    joycon.write_spi(colors)?;
    println!("Reconnect your controller");
    Ok(())
}
