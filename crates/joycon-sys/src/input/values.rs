use num::FromPrimitive;
use std::fmt;

bitfield::bitfield! {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct DeviceStatus(u8);
    impl Debug;

    pub connected, _: 0;
    pub u8, into DeviceType, device_type, _: 2, 1;
    pub charging, _: 4;
    pub u8, into BatteryLevel, battery_level, _: 7, 5;
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum DeviceType {
    ProController = 0,
    // Used when the ringcon is plugged, maybe also for the pokeball?
    MaybeAccessory = 1,
    // Used in one InputReport when the ringcon is plugged, then switch to value 1.
    MaybeInitializingAccessory = 2,
    Joycon = 3,
}

impl From<u8> for DeviceType {
    fn from(v: u8) -> Self {
        match DeviceType::from_u8(v) {
            Some(t) => t,
            None => panic!("unknown device type 0x{:x}", v),
        }
    }
}

#[derive(Debug, Copy, Clone, FromPrimitive, Eq, PartialEq, Ord, PartialOrd)]
pub enum BatteryLevel {
    Empty = 0,
    Critical = 1,
    Low = 2,
    Medium = 3,
    Full = 4,
}

impl From<u8> for BatteryLevel {
    fn from(v: u8) -> Self {
        BatteryLevel::from_u8(v).expect("unexpected battery level")
    }
}

#[repr(packed)]
#[derive(Copy, Clone, Default)]
pub struct ButtonsStatus {
    pub right: RightButtons,
    pub middle: MiddleButtons,
    pub left: LeftButtons,
}

impl fmt::Debug for ButtonsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ButtonsStatus").field(&format_args!("{}", self)).finish()
    }
}

impl fmt::Display for ButtonsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buttons = Vec::new();
        if self.right.a() {
            buttons.push("A");
        }
        if self.right.b() {
            buttons.push("B");
        }
        if self.right.x() {
            buttons.push("X");
        }
        if self.right.y() {
            buttons.push("Y");
        }
        if self.left.up() {
            buttons.push("DPAD_UP");
        }
        if self.left.down() {
            buttons.push("DPAD_DOWN");
        }
        if self.left.left() {
            buttons.push("DPAD_LEFT");
        }
        if self.left.right() {
            buttons.push("DPAD_RIGHT");
        }
        if self.left.l() {
            buttons.push("L");
        }
        if self.left.zl() {
            buttons.push("ZL");
        }
        if self.right.r() {
            buttons.push("R");
        }
        if self.right.zr() {
            buttons.push("ZR");
        }
        if self.left.sl() || self.right.sl() {
            buttons.push("JCL_SL");
        }
        if self.left.sr() || self.right.sr() {
            buttons.push("JCL_SR");
        }
        if self.middle.lstick() {
            buttons.push("L_STICK_PRESS");
        }
        if self.middle.rstick() {
            buttons.push("R_STICK_PRESS");
        }
        if self.middle.minus() {
            buttons.push("MINUS");
        }
        if self.middle.plus() {
            buttons.push("PLUS");
        }
        if self.middle.capture() {
            buttons.push("CAPTURE");
        }
        if self.middle.home() {
            buttons.push("HOME");
        }
        if !buttons.is_empty() {
            write!(f, "BUTTONS,{} ", buttons.join(","))?;
        }
        Ok(())
    }
}

bitfield::bitfield! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default)]
    pub struct RightButtons(u8);
    impl Debug;
    pub y, _: 0;
    pub x, _: 1;
    pub b, _: 2;
    pub a, _: 3;
    pub sr, _: 4;
    pub sl, _: 5;
    pub r, _: 6;
    pub zr, _: 7;
}
bitfield::bitfield! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default)]
    pub struct MiddleButtons(u8);
    impl Debug;
    pub minus, _: 0;
    pub plus, _: 1;
    pub rstick, _: 2;
    pub lstick, _: 3;
    pub home, _: 4;
    pub capture, _: 5;
    pub _unused, _: 6;
    pub charging_grip, _: 7;
}

bitfield::bitfield! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default)]
    pub struct LeftButtons(u8);
    impl Debug;
    pub down, _: 0;
    pub up, _: 1;
    pub right, _: 2;
    pub left, _: 3;
    pub sr, _: 4;
    pub sl, _: 5;
    pub l, _: 6;
    pub zl, _: 7;
}

pub enum Button {
    N,
    S,
    E,
    W,
    L,
    R,
    ZL,
    ZR,
    L3,
    R3,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Stick {
    data: [u8; 3],
}

impl Stick {
    pub fn x(self) -> u16 {
        u16::from(self.data[0]) | u16::from(self.data[1] & 0xf) << 8
    }

    pub fn y(self) -> u16 {
        u16::from(self.data[1]) >> 4 | u16::from(self.data[2]) << 4
    }
}

impl fmt::Debug for Stick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Stick").field(&self.x()).field(&self.y()).finish()
    }
}
