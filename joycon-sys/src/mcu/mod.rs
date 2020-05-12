/// Cf https://github.com/CTCaer/Nintendo_Switch_Reverse_Engineering/blob/ir-nfc/mcu_ir_nfc_notes.md
use crate::common::*;
use crate::input::*;
use std::fmt;

pub mod ir_register;

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct MCUReport {
    pub id: RawId<MCUReportId>,
    u: MCUReportUnion,
}

impl MCUReport {
    pub fn validate(&self) {
        /*
        assert!(
            self.id.try_into().is_some(),
            "invalid MCU report id {:?}",
            self.id
        );*/
        if self.id.try_into().is_none() {
            let slice = unsafe { (&self.u as *const _ as *const [u8; 20]).as_ref() };
            println!("{:?}", slice);
        }
    }
    pub fn as_status(&self) -> Option<&MCUStatus> {
        if self.id == MCUReportId::StateReport {
            Some(unsafe { &self.u.status })
        } else {
            None
        }
    }

    pub fn is_busy_init(&self) -> bool {
        self.id == MCUReportId::BusyInitializing
    }

    pub fn as_ir_status(&self) -> Option<&IRStatus> {
        if self.id == MCUReportId::IRStatus {
            Some(unsafe { &self.u.ir_status })
        } else {
            None
        }
    }

    pub fn as_ir_data(&self) -> Option<&IRData> {
        if self.id == MCUReportId::IRData {
            Some(unsafe { &self.u.ir_data })
        } else {
            None
        }
    }

    pub fn as_ir_registers(&self) -> Option<&IRRegistersSlice> {
        if self.id == MCUReportId::IRRegisters {
            Some(unsafe { &self.u.ir_registers })
        } else {
            None
        }
    }
}

impl fmt::Debug for MCUReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = f.debug_struct("MCUReport");
        match self.id.try_into() {
            Some(MCUReportId::StateReport) => out.field("status", unsafe { &self.u.status }),
            x @ Some(MCUReportId::BusyInitializing) => out.field("type", &x),
            x @ Some(MCUReportId::Empty) => out.field("type", &x),
            x @ Some(MCUReportId::EmptyAwaitingCmd) => out.field("type", &x),
            id => out.field("unknown_id", &id),
        }
        .finish()
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MCUReportId {
    Empty = 0x00,
    StateReport = 0x01,
    IRData = 0x03,
    BusyInitializing = 0x0b,
    IRStatus = 0x13,
    IRRegisters = 0x1b,
    NFCState = 0x2a,
    NFCReadData = 0x3a,
    EmptyAwaitingCmd = 0xff,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub union MCUReportUnion {
    // add to validate when adding variant
    _raw: [u8; 312],
    status: MCUStatus,
    ir_status: IRStatus,
    ir_data: IRData,
    ir_registers: IRRegistersSlice,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IRRegistersSlice {
    _unknown_0x00: u8,
    pub page: u8,
    pub offset: u8,
    pub nb_registers: u8,
    pub values: [u8; 0x7f],
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IRData {
    _unknown: [u8; 2],
    pub frag_number: u8,
    pub average_intensity: u8,
    // Only when EXFilter enabled
    _unknown3: u8,
    pub white_pixel_count: U16LE,
    pub ambient_noise_count: U16LE,
    pub img_fragment: [u8; 300],
}

#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct MCUStatus {
    _unknown: [u8; 2],
    pub fw_major_version: U16LE,
    pub fw_minor_version: U16LE,
    pub state: RawId<MCUMode>,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IRStatus {
    _unknown_0x00: u8,
    pub ir_mode: MCUIRMode,
    pub required_fw_major_version: U16LE,
    pub required_fw_minor_version: U16LE,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct MCUCmd {
    pub cmd_id: MCUCmdId,
    pub subcmd_id: MCUSubCmdId,
    pub u: MCUCmdData,
}

impl MCUCmd {
    pub fn compute_crc(&mut self) {
        unsafe { self.u.crc.compute_crc8(self.subcmd_id) }
    }
}

impl fmt::Debug for MCUCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = f.debug_struct("MCUCmd");
        out.field("crc", unsafe { &self.u.crc });
        match (self.cmd_id, self.subcmd_id) {
            (MCUCmdId::ConfigureIR, MCUSubCmdId::WriteIRRegisters) => {
                out.field("cmd", unsafe { &self.u.regs })
            }
            ids => out.field("subcommand", &ids),
        };
        out.finish()
    }
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub union MCUCmdData {
    pub mcu_mode: MCUMode,
    pub regs: MCURegisters,
    pub crc: CRC8A,
    pub ir_mode: MCUIRModeData,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct CRC8A {
    bytes: [u8; 34],
    crc: u8,
}

impl fmt::Debug for CRC8A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("CRC8A").field(&self.crc).finish()
    }
}

impl CRC8A {
    pub fn compute_crc8(&mut self, cmd_id: MCUCmdId, subcmd_id: MCUSubCmdId) {
        // To simplify the data layout, subcmd_id is outside the byte buffer.
        self.crc = compute_crc8(cmd_id as u8, subcmd_id as u8, &self.bytes);
    }
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct CRC8B {
    bytes: [u8; 36],
    crc: u8,
    _padding_0xff: u8,
}

impl CRC8B {
    pub fn compute_crc8(&mut self, id: IRDataRequestId) {
        // To simplify the data layout, subcmd_id is outside the byte buffer.
        self.crc = compute_crc8(0, 0, &self.bytes);
        self._padding_0xff = match id {
            IRDataRequestId::GetSensorData | IRDataRequestId::GetState => 0xff,
            IRDataRequestId::ReadRegister => 0x00,
        };
    }
}

fn compute_crc8(id1: u8, id2: u8, bytes: &[u8]) -> u8 {
    use std::iter::once;
    // To simplify the data layout, subcmd_id is outside the byte buffer.
    let mut crc = 9;
    for byte in once(id1).chain(once(id2)).chain(bytes.iter().cloned()) {
        crc = MCU_CRC8_TABLE[(crc ^ byte) as usize];
    }
    crc
}

#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct MCURegisters {
    pub len: u8,
    pub regs: [ir_register::Register; 9],
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct MCUSetReg {
    pub cmd_id: MCUCmdId,
    pub subcmd_id: MCUSubCmdId,
    pub mode: MCUMode,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MCUMode {
    Suspend = 0,
    Standby = 1,
    NFC = 4,
    IR = 5,
    MaybeFWUpdate = 6,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MCUCmdId {
    ConfigureMCU = 0x21,
    ConfigureIR = 0x23,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MCUSubCmdId {
    SetMCUMode = 0,
    SetIRMode = 1,
    WriteIRRegisters = 4,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MCUSubCmdId2 {
    GetMCUStatus = 1,
    GetNFCData = 2,
    GetIRData = 3,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MCUIRMode {
    IRSensorReset = 0,
    IRSensorSleep = 1,
    Unknown2 = 2,
    Moment = 3,
    /// Wii-style pointing
    Dpd = 4,
    Unknown5 = 5,
    Clustering = 6,
    ImageTransfer = 7,
    HandAnalysisSilhouette = 8,
    HandAnalysisImage = 9,
    HandAnalysisSilhouetteImage = 10,
    Unknown11 = 11,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct MCUIRModeData {
    pub ir_mode: MCUIRMode,
    /// Set number of packets to output per buffer
    pub no_of_frags: u8,
    /// Get it from MCUStatus
    pub mcu_fw_version: (U16LE, U16LE),
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct MCUSubcommand {
    pub subcmd_id: MCUSubCmdId2,
    pub u: MCUSubcommandUnion,
}

impl MCUSubcommand {
    pub fn compute_crc(&mut self, id: IRDataRequestId) {
        unsafe { self.u.crc.compute_crc8(id) }
    }
}

impl fmt::Debug for MCUSubcommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MCUSubcommand").finish()
    }
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub union MCUSubcommandUnion {
    pub nothing: (),
    pub ir_cmd: IRDataRequest,
    pub crc: CRC8B,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IRDataRequest {
    pub id: IRDataRequestId,
    pub u: IRDataRequestUnion,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum IRDataRequestId {
    GetSensorData = 0,
    GetState = 2,
    ReadRegister = 3,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub union IRDataRequestUnion {
    pub nothing: (),
    pub ack_request_packet: IRAckRequestPacket,
    pub read_registers: IRReadRegisters,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IRAckRequestPacket {
    pub packet_missing: bool,
    pub missed_packet_id: u8,
    pub ack_packet_id: u8,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IRReadRegisters {
    pub unknown_0x01: u8,
    pub page: u8,
    pub offset: u8,
    pub nb_registers: u8,
}

// crc-8-ccitt / polynomial 0x07 look up table
// From jc_toolkit
const MCU_CRC8_TABLE: [u8; 256] = [
    0x00, 0x07, 0x0E, 0x09, 0x1C, 0x1B, 0x12, 0x15, 0x38, 0x3F, 0x36, 0x31, 0x24, 0x23, 0x2A, 0x2D,
    0x70, 0x77, 0x7E, 0x79, 0x6C, 0x6B, 0x62, 0x65, 0x48, 0x4F, 0x46, 0x41, 0x54, 0x53, 0x5A, 0x5D,
    0xE0, 0xE7, 0xEE, 0xE9, 0xFC, 0xFB, 0xF2, 0xF5, 0xD8, 0xDF, 0xD6, 0xD1, 0xC4, 0xC3, 0xCA, 0xCD,
    0x90, 0x97, 0x9E, 0x99, 0x8C, 0x8B, 0x82, 0x85, 0xA8, 0xAF, 0xA6, 0xA1, 0xB4, 0xB3, 0xBA, 0xBD,
    0xC7, 0xC0, 0xC9, 0xCE, 0xDB, 0xDC, 0xD5, 0xD2, 0xFF, 0xF8, 0xF1, 0xF6, 0xE3, 0xE4, 0xED, 0xEA,
    0xB7, 0xB0, 0xB9, 0xBE, 0xAB, 0xAC, 0xA5, 0xA2, 0x8F, 0x88, 0x81, 0x86, 0x93, 0x94, 0x9D, 0x9A,
    0x27, 0x20, 0x29, 0x2E, 0x3B, 0x3C, 0x35, 0x32, 0x1F, 0x18, 0x11, 0x16, 0x03, 0x04, 0x0D, 0x0A,
    0x57, 0x50, 0x59, 0x5E, 0x4B, 0x4C, 0x45, 0x42, 0x6F, 0x68, 0x61, 0x66, 0x73, 0x74, 0x7D, 0x7A,
    0x89, 0x8E, 0x87, 0x80, 0x95, 0x92, 0x9B, 0x9C, 0xB1, 0xB6, 0xBF, 0xB8, 0xAD, 0xAA, 0xA3, 0xA4,
    0xF9, 0xFE, 0xF7, 0xF0, 0xE5, 0xE2, 0xEB, 0xEC, 0xC1, 0xC6, 0xCF, 0xC8, 0xDD, 0xDA, 0xD3, 0xD4,
    0x69, 0x6E, 0x67, 0x60, 0x75, 0x72, 0x7B, 0x7C, 0x51, 0x56, 0x5F, 0x58, 0x4D, 0x4A, 0x43, 0x44,
    0x19, 0x1E, 0x17, 0x10, 0x05, 0x02, 0x0B, 0x0C, 0x21, 0x26, 0x2F, 0x28, 0x3D, 0x3A, 0x33, 0x34,
    0x4E, 0x49, 0x40, 0x47, 0x52, 0x55, 0x5C, 0x5B, 0x76, 0x71, 0x78, 0x7F, 0x6A, 0x6D, 0x64, 0x63,
    0x3E, 0x39, 0x30, 0x37, 0x22, 0x25, 0x2C, 0x2B, 0x06, 0x01, 0x08, 0x0F, 0x1A, 0x1D, 0x14, 0x13,
    0xAE, 0xA9, 0xA0, 0xA7, 0xB2, 0xB5, 0xBC, 0xBB, 0x96, 0x91, 0x98, 0x9F, 0x8A, 0x8D, 0x84, 0x83,
    0xDE, 0xD9, 0xD0, 0xD7, 0xC2, 0xC5, 0xCC, 0xCB, 0xE6, 0xE1, 0xE8, 0xEF, 0xFA, 0xFD, 0xF4, 0xF3,
];

#[cfg(test)]
#[test]
fn check_input_layout() {
    unsafe {
        let report = InputReport::new();
        let mcu_report = report.u_mcu_report();
        assert_eq!(49, offset_of(&report, mcu_report));
        assert_eq!(56, offset_of(&report, &mcu_report.u.status.state));
        assert_eq!(52, offset_of(&report, &mcu_report.u.ir_data.frag_number));
        assert_eq!(
            53,
            offset_of(&report, &mcu_report.u.ir_data.average_intensity)
        );
        assert_eq!(
            55,
            offset_of(&report, &mcu_report.u.ir_data.white_pixel_count)
        );
        assert_eq!(59, offset_of(&report, &mcu_report.u.ir_data.img_fragment));

        assert_eq!(54, offset_of(&report, &mcu_report.u.ir_registers.values));
    }
}

#[cfg(test)]
#[test]
fn check_output_layout() {
    unsafe {
        let report = crate::output::OutputReport::new();
        let cmd = report.as_mcu_subcmd();
        // Same as normal output report
        assert_eq!(10, offset_of(&report, &cmd.subcmd_id));
        assert_eq!(11, offset_of(&report, &cmd.u.ir_cmd.id));
        assert_eq!(11, offset_of(&report, &cmd.u.crc));
        assert_eq!(47, offset_of(&report, &cmd.u.crc.crc));
        assert_eq!(48, offset_of(&report, &cmd.u.crc._padding_0xff));

        assert_eq!(
            15,
            offset_of(&report, &cmd.u.ir_cmd.u.read_registers.nb_registers)
        );
        assert_eq!(12, offset_of(&report, &report.as_mcu_cmd().cmd_id));
        assert_eq!(14, offset_of(&report, &report.as_mcu_cmd().u.crc.bytes));
        assert_eq!(48, offset_of(&report, &report.as_mcu_cmd().u.crc.crc));
    }
}