(function() {var implementors = {};
implementors["dualshock"] = [{"text":"impl Sync for DS4Driver","synthetic":true,"types":[]},{"text":"impl !Sync for DS4","synthetic":true,"types":[]}];
implementors["dualshock_sys"] = [{"text":"impl Sync for InputReport","synthetic":true,"types":[]},{"text":"impl Sync for InputReportId","synthetic":true,"types":[]},{"text":"impl Sync for USBReport","synthetic":true,"types":[]},{"text":"impl Sync for BTSimpleReport","synthetic":true,"types":[]},{"text":"impl Sync for BTFullReport","synthetic":true,"types":[]},{"text":"impl Sync for FullReport","synthetic":true,"types":[]},{"text":"impl Sync for SimpleReport","synthetic":true,"types":[]},{"text":"impl Sync for Type","synthetic":true,"types":[]},{"text":"impl Sync for Trigger","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Buttons&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for Dpad","synthetic":true,"types":[]},{"text":"impl Sync for Gyro","synthetic":true,"types":[]},{"text":"impl Sync for Accel","synthetic":true,"types":[]},{"text":"impl Sync for Stick","synthetic":true,"types":[]},{"text":"impl Sync for BTTrackpad","synthetic":true,"types":[]},{"text":"impl Sync for USBTrackpad","synthetic":true,"types":[]},{"text":"impl Sync for TrackpadPacket","synthetic":true,"types":[]},{"text":"impl Sync for Finger","synthetic":true,"types":[]},{"text":"impl Sync for FingerCoord","synthetic":true,"types":[]},{"text":"impl Sync for ConnectionType","synthetic":true,"types":[]},{"text":"impl&lt;Id&gt; Sync for RawId&lt;Id&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Id: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for I16LE","synthetic":true,"types":[]}];
implementors["gyromouse"] = [{"text":"impl Sync for GyroMouse","synthetic":true,"types":[]},{"text":"impl Sync for CameraStick","synthetic":true,"types":[]},{"text":"impl Sync for FlickStickState","synthetic":true,"types":[]},{"text":"impl Sync for FlickStick","synthetic":true,"types":[]},{"text":"impl Sync for ButtonStick","synthetic":true,"types":[]},{"text":"impl Sync for Action","synthetic":true,"types":[]},{"text":"impl Sync for ExtAction","synthetic":true,"types":[]},{"text":"impl Sync for KeyStatus","synthetic":true,"types":[]},{"text":"impl Sync for Layer","synthetic":true,"types":[]},{"text":"impl Sync for KeyState","synthetic":true,"types":[]},{"text":"impl Sync for VirtualKey","synthetic":true,"types":[]},{"text":"impl Sync for MapKey","synthetic":true,"types":[]},{"text":"impl Sync for Buttons","synthetic":true,"types":[]},{"text":"impl !Sync for Mouse","synthetic":true,"types":[]},{"text":"impl Sync for ActionModifier","synthetic":true,"types":[]},{"text":"impl Sync for EventModifier","synthetic":true,"types":[]},{"text":"impl Sync for JSMAction","synthetic":true,"types":[]},{"text":"impl Sync for ActionType","synthetic":true,"types":[]},{"text":"impl Sync for Key","synthetic":true,"types":[]},{"text":"impl Sync for SpecialKey","synthetic":true,"types":[]},{"text":"impl Sync for Cmd","synthetic":true,"types":[]},{"text":"impl Sync for ClickType","synthetic":true,"types":[]}];
implementors["hid_gamepad"] = [{"text":"impl Sync for GamepadError","synthetic":true,"types":[]}];
implementors["hid_gamepad_sys"] = [{"text":"impl Sync for JoyKey","synthetic":true,"types":[]},{"text":"impl Sync for KeyStatus","synthetic":true,"types":[]},{"text":"impl Sync for Report","synthetic":true,"types":[]},{"text":"impl Sync for Motion","synthetic":true,"types":[]}];
implementors["joycon"] = [{"text":"impl Sync for Calibration","synthetic":true,"types":[]},{"text":"impl Sync for Report","synthetic":true,"types":[]},{"text":"impl !Sync for JoyCon","synthetic":true,"types":[]},{"text":"impl Sync for Image","synthetic":true,"types":[]},{"text":"impl Sync for IMU","synthetic":true,"types":[]},{"text":"impl Sync for JoyconDriver","synthetic":true,"types":[]}];
implementors["joycon_sys"] = [{"text":"impl Sync for AccessoryCommandId","synthetic":true,"types":[]},{"text":"impl Sync for AccessoryType","synthetic":true,"types":[]},{"text":"impl Sync for RingconItemId","synthetic":true,"types":[]},{"text":"impl Sync for AccessoryCommand","synthetic":true,"types":[]},{"text":"impl Sync for AccessoryResponse","synthetic":true,"types":[]},{"text":"impl Sync for OfflineSteps","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl Sync for InputReportId","synthetic":true,"types":[]},{"text":"impl Sync for SubcommandId","synthetic":true,"types":[]},{"text":"impl Sync for U16LE","synthetic":true,"types":[]},{"text":"impl Sync for I16LE","synthetic":true,"types":[]},{"text":"impl Sync for U32LE","synthetic":true,"types":[]},{"text":"impl&lt;Id&gt; Sync for RawId&lt;Id&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Id: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for Bool","synthetic":true,"types":[]},{"text":"impl Sync for IMUMode","synthetic":true,"types":[]},{"text":"impl Sync for Frame","synthetic":true,"types":[]},{"text":"impl Sync for Sensitivity","synthetic":true,"types":[]},{"text":"impl Sync for GyroSens","synthetic":true,"types":[]},{"text":"impl Sync for AccSens","synthetic":true,"types":[]},{"text":"impl Sync for GyroPerfRate","synthetic":true,"types":[]},{"text":"impl Sync for AccAntiAliasing","synthetic":true,"types":[]},{"text":"impl Sync for DeviceStatus","synthetic":true,"types":[]},{"text":"impl Sync for DeviceType","synthetic":true,"types":[]},{"text":"impl Sync for BatteryLevel","synthetic":true,"types":[]},{"text":"impl Sync for ButtonsStatus","synthetic":true,"types":[]},{"text":"impl Sync for RightButtons","synthetic":true,"types":[]},{"text":"impl Sync for MiddleButtons","synthetic":true,"types":[]},{"text":"impl Sync for LeftButtons","synthetic":true,"types":[]},{"text":"impl Sync for Button","synthetic":true,"types":[]},{"text":"impl Sync for Stick","synthetic":true,"types":[]},{"text":"impl Sync for InputReport","synthetic":true,"types":[]},{"text":"impl Sync for InputReportEnum","synthetic":true,"types":[]},{"text":"impl Sync for NormalInputReport","synthetic":true,"types":[]},{"text":"impl Sync for StandardInputReport","synthetic":true,"types":[]},{"text":"impl Sync for SubcommandReply","synthetic":true,"types":[]},{"text":"impl Sync for SubcommandReplyEnum","synthetic":true,"types":[]},{"text":"impl Sync for Ack","synthetic":true,"types":[]},{"text":"impl Sync for DeviceInfo","synthetic":true,"types":[]},{"text":"impl Sync for FirmwareVersion","synthetic":true,"types":[]},{"text":"impl Sync for MACAddress","synthetic":true,"types":[]},{"text":"impl Sync for WhichController","synthetic":true,"types":[]},{"text":"impl Sync for UseSPIColors","synthetic":true,"types":[]},{"text":"impl Sync for PlayerLights","synthetic":true,"types":[]},{"text":"impl Sync for PlayerLight","synthetic":true,"types":[]},{"text":"impl Sync for HomeLight","synthetic":true,"types":[]},{"text":"impl Sync for Register","synthetic":true,"types":[]},{"text":"impl Sync for Resolution","synthetic":true,"types":[]},{"text":"impl Sync for ExposureMode","synthetic":true,"types":[]},{"text":"impl Sync for ExternalLightFilter","synthetic":true,"types":[]},{"text":"impl Sync for Flip","synthetic":true,"types":[]},{"text":"impl Sync for Leds","synthetic":true,"types":[]},{"text":"impl Sync for IRRequestId","synthetic":true,"types":[]},{"text":"impl Sync for IRRequest","synthetic":true,"types":[]},{"text":"impl Sync for IRRequestEnum","synthetic":true,"types":[]},{"text":"impl Sync for IRAckRequestPacket","synthetic":true,"types":[]},{"text":"impl Sync for IRReadRegisters","synthetic":true,"types":[]},{"text":"impl Sync for MCUIRMode","synthetic":true,"types":[]},{"text":"impl Sync for MCUIRModeData","synthetic":true,"types":[]},{"text":"impl Sync for IRStatus","synthetic":true,"types":[]},{"text":"impl Sync for IRRegistersSlice","synthetic":true,"types":[]},{"text":"impl Sync for IRData","synthetic":true,"types":[]},{"text":"impl Sync for MCURegisters","synthetic":true,"types":[]},{"text":"impl Sync for MCUSetReg","synthetic":true,"types":[]},{"text":"impl Sync for MCUReportId","synthetic":true,"types":[]},{"text":"impl Sync for MCUReport","synthetic":true,"types":[]},{"text":"impl Sync for MCUReportEnum","synthetic":true,"types":[]},{"text":"impl Sync for MCUStatus","synthetic":true,"types":[]},{"text":"impl Sync for MCUCommandId","synthetic":true,"types":[]},{"text":"impl Sync for MCUSubCommandId","synthetic":true,"types":[]},{"text":"impl Sync for MCUCommand","synthetic":true,"types":[]},{"text":"impl Sync for MCUCommandCRC","synthetic":true,"types":[]},{"text":"impl Sync for MCUMode","synthetic":true,"types":[]},{"text":"impl Sync for MCURequestId","synthetic":true,"types":[]},{"text":"impl Sync for MCURequest","synthetic":true,"types":[]},{"text":"impl Sync for MCURequestEnum","synthetic":true,"types":[]},{"text":"impl Sync for MCURequestCRC","synthetic":true,"types":[]},{"text":"impl Sync for OutputReportId","synthetic":true,"types":[]},{"text":"impl Sync for OutputReport","synthetic":true,"types":[]},{"text":"impl Sync for OutputReportEnum","synthetic":true,"types":[]},{"text":"impl Sync for Rumble","synthetic":true,"types":[]},{"text":"impl Sync for SubcommandRequest","synthetic":true,"types":[]},{"text":"impl Sync for SubcommandRequestEnum","synthetic":true,"types":[]},{"text":"impl Sync for RumbleData","synthetic":true,"types":[]},{"text":"impl Sync for RumbleSide","synthetic":true,"types":[]},{"text":"impl Sync for SPIRange","synthetic":true,"types":[]},{"text":"impl Sync for WrongRangeError","synthetic":true,"types":[]},{"text":"impl Sync for SPIReadRequest","synthetic":true,"types":[]},{"text":"impl Sync for SPIWriteRequest","synthetic":true,"types":[]},{"text":"impl Sync for SPIReadResult","synthetic":true,"types":[]},{"text":"impl Sync for SPIWriteResult","synthetic":true,"types":[]},{"text":"impl Sync for SticksCalibration","synthetic":true,"types":[]},{"text":"impl Sync for UserSticksCalibration","synthetic":true,"types":[]},{"text":"impl Sync for LeftStickCalibration","synthetic":true,"types":[]},{"text":"impl Sync for RightStickCalibration","synthetic":true,"types":[]},{"text":"impl Sync for UserStickCalibration","synthetic":true,"types":[]},{"text":"impl Sync for SensorCalibration","synthetic":true,"types":[]},{"text":"impl Sync for UserSensorCalibration","synthetic":true,"types":[]},{"text":"impl Sync for Color","synthetic":true,"types":[]},{"text":"impl Sync for ControllerColor","synthetic":true,"types":[]}];
implementors["joytk"] = [{"text":"impl Sync for Opts","synthetic":true,"types":[]},{"text":"impl Sync for SubCommand","synthetic":true,"types":[]},{"text":"impl Sync for Calibrate","synthetic":true,"types":[]},{"text":"impl Sync for CalibrateE","synthetic":true,"types":[]},{"text":"impl Sync for Set","synthetic":true,"types":[]},{"text":"impl Sync for SetE","synthetic":true,"types":[]},{"text":"impl Sync for SetColor","synthetic":true,"types":[]},{"text":"impl Sync for Ringcon","synthetic":true,"types":[]},{"text":"impl Sync for RingconE","synthetic":true,"types":[]},{"text":"impl Sync for Relay","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()