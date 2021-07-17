(function() {var implementors = {};
implementors["dualshock_sys"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"dualshock_sys/input/struct.Dpad.html\" title=\"struct dualshock_sys::input::Dpad\">Dpad</a>","synthetic":false,"types":["dualshock_sys::input::Dpad"]},{"text":"impl&lt;Id:&nbsp;<a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/cast/trait.ToPrimitive.html\" title=\"trait num_traits::cast::ToPrimitive\">ToPrimitive</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Id&gt; for <a class=\"struct\" href=\"dualshock_sys/struct.RawId.html\" title=\"struct dualshock_sys::RawId\">RawId</a>&lt;Id&gt;","synthetic":false,"types":["dualshock_sys::RawId"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>&gt; for <a class=\"struct\" href=\"dualshock_sys/struct.I16LE.html\" title=\"struct dualshock_sys::I16LE\">I16LE</a>","synthetic":false,"types":["dualshock_sys::I16LE"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"dualshock_sys/struct.I16LE.html\" title=\"struct dualshock_sys::I16LE\">I16LE</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>","synthetic":false,"types":[]}];
implementors["gyromouse"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"hid_gamepad_sys/enum.JoyKey.html\" title=\"enum hid_gamepad_sys::JoyKey\">JoyKey</a>&gt; for <a class=\"enum\" href=\"gyromouse/mapping/enum.MapKey.html\" title=\"enum gyromouse::mapping::MapKey\">MapKey</a>","synthetic":false,"types":["gyromouse::mapping::MapKey"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"gyromouse/mapping/enum.VirtualKey.html\" title=\"enum gyromouse::mapping::VirtualKey\">VirtualKey</a>&gt; for <a class=\"enum\" href=\"gyromouse/mapping/enum.MapKey.html\" title=\"enum gyromouse::mapping::MapKey\">MapKey</a>","synthetic":false,"types":["gyromouse::mapping::MapKey"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"enum\" href=\"gyromouse/parse/enum.ActionType.html\" title=\"enum gyromouse::parse::ActionType\">ActionType</a>, <a class=\"enum\" href=\"gyromouse/enum.ClickType.html\" title=\"enum gyromouse::ClickType\">ClickType</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"enum\" href=\"gyromouse/mapping/enum.ExtAction.html\" title=\"enum gyromouse::mapping::ExtAction\">ExtAction</a>","synthetic":false,"types":["gyromouse::mapping::ExtAction"]}];
implementors["hid_gamepad"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;HidError&gt; for <a class=\"enum\" href=\"hid_gamepad/enum.GamepadError.html\" title=\"enum hid_gamepad::GamepadError\">GamepadError</a>","synthetic":false,"types":["hid_gamepad::error::GamepadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/anyhow/1.0.42/anyhow/struct.Error.html\" title=\"struct anyhow::Error\">Error</a>&gt; for <a class=\"enum\" href=\"hid_gamepad/enum.GamepadError.html\" title=\"enum hid_gamepad::GamepadError\">GamepadError</a>","synthetic":false,"types":["hid_gamepad::error::GamepadError"]}];
implementors["hid_gamepad_sys"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>&gt; for <a class=\"enum\" href=\"hid_gamepad_sys/enum.KeyStatus.html\" title=\"enum hid_gamepad_sys::KeyStatus\">KeyStatus</a>","synthetic":false,"types":["hid_gamepad_sys::KeyStatus"]}];
implementors["joycon"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon/struct.Report.html\" title=\"struct joycon::Report\">Report</a>&gt; for <a class=\"struct\" href=\"hid_gamepad_sys/struct.Report.html\" title=\"struct hid_gamepad_sys::Report\">Report</a>","synthetic":false,"types":["hid_gamepad_sys::Report"]}];
implementors["joycon_sys"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt; for <a class=\"struct\" href=\"joycon_sys/common/struct.U16LE.html\" title=\"struct joycon_sys::common::U16LE\">U16LE</a>","synthetic":false,"types":["joycon_sys::common::U16LE"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/common/struct.U16LE.html\" title=\"struct joycon_sys::common::U16LE\">U16LE</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>&gt; for <a class=\"struct\" href=\"joycon_sys/common/struct.I16LE.html\" title=\"struct joycon_sys::common::I16LE\">I16LE</a>","synthetic":false,"types":["joycon_sys::common::I16LE"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/common/struct.I16LE.html\" title=\"struct joycon_sys::common::I16LE\">I16LE</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; for <a class=\"struct\" href=\"joycon_sys/common/struct.U32LE.html\" title=\"struct joycon_sys::common::U32LE\">U32LE</a>","synthetic":false,"types":["joycon_sys::common::U32LE"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/common/struct.U32LE.html\" title=\"struct joycon_sys::common::U32LE\">U32LE</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>","synthetic":false,"types":[]},{"text":"impl&lt;Id:&nbsp;<a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/cast/trait.ToPrimitive.html\" title=\"trait num_traits::cast::ToPrimitive\">ToPrimitive</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Id&gt; for <a class=\"struct\" href=\"joycon_sys/common/struct.RawId.html\" title=\"struct joycon_sys::common::RawId\">RawId</a>&lt;Id&gt;","synthetic":false,"types":["joycon_sys::common::RawId"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>&gt; for <a class=\"enum\" href=\"joycon_sys/common/enum.Bool.html\" title=\"enum joycon_sys::common::Bool\">Bool</a>","synthetic":false,"types":["joycon_sys::common::Bool"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; for <a class=\"enum\" href=\"joycon_sys/input/enum.DeviceType.html\" title=\"enum joycon_sys::input::DeviceType\">DeviceType</a>","synthetic":false,"types":["joycon_sys::input::values::DeviceType"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; for <a class=\"enum\" href=\"joycon_sys/input/enum.BatteryLevel.html\" title=\"enum joycon_sys::input::BatteryLevel\">BatteryLevel</a>","synthetic":false,"types":["joycon_sys::input::values::BatteryLevel"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/input/enum.InputReportEnum.html\" title=\"enum joycon_sys::input::InputReportEnum\">InputReportEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/input/struct.InputReport.html\" title=\"struct joycon_sys::input::InputReport\">InputReport</a>","synthetic":false,"types":["joycon_sys::input::report::InputReport"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/input/enum.SubcommandReplyEnum.html\" title=\"enum joycon_sys::input::SubcommandReplyEnum\">SubcommandReplyEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/input/struct.SubcommandReply.html\" title=\"struct joycon_sys::input::SubcommandReply\">SubcommandReply</a>","synthetic":false,"types":["joycon_sys::input::report::SubcommandReply"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>&gt; for <a class=\"enum\" href=\"joycon_sys/light/enum.PlayerLight.html\" title=\"enum joycon_sys::light::PlayerLight\">PlayerLight</a>","synthetic":false,"types":["joycon_sys::light::PlayerLight"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/mcu/ir/enum.IRRequestEnum.html\" title=\"enum joycon_sys::mcu::ir::IRRequestEnum\">IRRequestEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/mcu/ir/struct.IRRequest.html\" title=\"struct joycon_sys::mcu::ir::IRRequest\">IRRequest</a>","synthetic":false,"types":["joycon_sys::mcu::ir::IRRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/mcu/ir/struct.IRAckRequestPacket.html\" title=\"struct joycon_sys::mcu::ir::IRAckRequestPacket\">IRAckRequestPacket</a>&gt; for <a class=\"struct\" href=\"joycon_sys/mcu/ir/struct.IRRequest.html\" title=\"struct joycon_sys::mcu::ir::IRRequest\">IRRequest</a>","synthetic":false,"types":["joycon_sys::mcu::ir::IRRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/mcu/ir/struct.IRReadRegisters.html\" title=\"struct joycon_sys::mcu::ir::IRReadRegisters\">IRReadRegisters</a>&gt; for <a class=\"struct\" href=\"joycon_sys/mcu/ir/struct.IRRequest.html\" title=\"struct joycon_sys::mcu::ir::IRRequest\">IRRequest</a>","synthetic":false,"types":["joycon_sys::mcu::ir::IRRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/mcu/enum.MCUReportEnum.html\" title=\"enum joycon_sys::mcu::MCUReportEnum\">MCUReportEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/mcu/struct.MCUReport.html\" title=\"struct joycon_sys::mcu::MCUReport\">MCUReport</a>","synthetic":false,"types":["joycon_sys::mcu::MCUReport"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/mcu/enum.MCURequestEnum.html\" title=\"enum joycon_sys::mcu::MCURequestEnum\">MCURequestEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/mcu/struct.MCURequest.html\" title=\"struct joycon_sys::mcu::MCURequest\">MCURequest</a>","synthetic":false,"types":["joycon_sys::mcu::MCURequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/mcu/ir/struct.IRRequest.html\" title=\"struct joycon_sys::mcu::ir::IRRequest\">IRRequest</a>&gt; for <a class=\"struct\" href=\"joycon_sys/mcu/struct.MCURequest.html\" title=\"struct joycon_sys::mcu::MCURequest\">MCURequest</a>","synthetic":false,"types":["joycon_sys::mcu::MCURequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/mcu/ir/enum.IRRequestEnum.html\" title=\"enum joycon_sys::mcu::ir::IRRequestEnum\">IRRequestEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/mcu/struct.MCURequest.html\" title=\"struct joycon_sys::mcu::MCURequest\">MCURequest</a>","synthetic":false,"types":["joycon_sys::mcu::MCURequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/output/enum.OutputReportEnum.html\" title=\"enum joycon_sys::output::OutputReportEnum\">OutputReportEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.OutputReport.html\" title=\"struct joycon_sys::output::OutputReport\">OutputReport</a>","synthetic":false,"types":["joycon_sys::output::report::OutputReport"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.OutputReport.html\" title=\"struct joycon_sys::output::OutputReport\">OutputReport</a>","synthetic":false,"types":["joycon_sys::output::report::OutputReport"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/output/enum.SubcommandRequestEnum.html\" title=\"enum joycon_sys::output::SubcommandRequestEnum\">SubcommandRequestEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.OutputReport.html\" title=\"struct joycon_sys::output::OutputReport\">OutputReport</a>","synthetic":false,"types":["joycon_sys::output::report::OutputReport"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/mcu/struct.MCURequest.html\" title=\"struct joycon_sys::mcu::MCURequest\">MCURequest</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.OutputReport.html\" title=\"struct joycon_sys::output::OutputReport\">OutputReport</a>","synthetic":false,"types":["joycon_sys::output::report::OutputReport"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/output/enum.SubcommandRequestEnum.html\" title=\"enum joycon_sys::output::SubcommandRequestEnum\">SubcommandRequestEnum</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>","synthetic":false,"types":["joycon_sys::output::report::SubcommandRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/mcu/struct.MCUCommand.html\" title=\"struct joycon_sys::mcu::MCUCommand\">MCUCommand</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>","synthetic":false,"types":["joycon_sys::output::report::SubcommandRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/accessory/struct.AccessoryCommand.html\" title=\"struct joycon_sys::accessory::AccessoryCommand\">AccessoryCommand</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>","synthetic":false,"types":["joycon_sys::output::report::SubcommandRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/imu/struct.Sensitivity.html\" title=\"struct joycon_sys::imu::Sensitivity\">Sensitivity</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>","synthetic":false,"types":["joycon_sys::output::report::SubcommandRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/spi/struct.SPIReadRequest.html\" title=\"struct joycon_sys::spi::SPIReadRequest\">SPIReadRequest</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>","synthetic":false,"types":["joycon_sys::output::report::SubcommandRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/spi/struct.SPIWriteRequest.html\" title=\"struct joycon_sys::spi::SPIWriteRequest\">SPIWriteRequest</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>","synthetic":false,"types":["joycon_sys::output::report::SubcommandRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/light/struct.PlayerLights.html\" title=\"struct joycon_sys::light::PlayerLights\">PlayerLights</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>","synthetic":false,"types":["joycon_sys::output::report::SubcommandRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/light/struct.HomeLight.html\" title=\"struct joycon_sys::light::HomeLight\">HomeLight</a>&gt; for <a class=\"struct\" href=\"joycon_sys/output/struct.SubcommandRequest.html\" title=\"struct joycon_sys::output::SubcommandRequest\">SubcommandRequest</a>","synthetic":false,"types":["joycon_sys::output::report::SubcommandRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/spi/struct.ControllerColor.html\" title=\"struct joycon_sys::spi::ControllerColor\">ControllerColor</a>&gt; for <a class=\"struct\" href=\"joycon_sys/spi/struct.SPIWriteRequest.html\" title=\"struct joycon_sys::spi::SPIWriteRequest\">SPIWriteRequest</a>","synthetic":false,"types":["joycon_sys::spi::SPIWriteRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"joycon_sys/input/enum.UseSPIColors.html\" title=\"enum joycon_sys::input::UseSPIColors\">UseSPIColors</a>&gt; for <a class=\"struct\" href=\"joycon_sys/spi/struct.SPIWriteRequest.html\" title=\"struct joycon_sys::spi::SPIWriteRequest\">SPIWriteRequest</a>","synthetic":false,"types":["joycon_sys::spi::SPIWriteRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/spi/struct.SensorCalibration.html\" title=\"struct joycon_sys::spi::SensorCalibration\">SensorCalibration</a>&gt; for <a class=\"struct\" href=\"joycon_sys/spi/struct.UserSensorCalibration.html\" title=\"struct joycon_sys::spi::UserSensorCalibration\">UserSensorCalibration</a>","synthetic":false,"types":["joycon_sys::spi::UserSensorCalibration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"joycon_sys/spi/struct.UserSensorCalibration.html\" title=\"struct joycon_sys::spi::UserSensorCalibration\">UserSensorCalibration</a>&gt; for <a class=\"struct\" href=\"joycon_sys/spi/struct.SPIWriteRequest.html\" title=\"struct joycon_sys::spi::SPIWriteRequest\">SPIWriteRequest</a>","synthetic":false,"types":["joycon_sys::spi::SPIWriteRequest"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()