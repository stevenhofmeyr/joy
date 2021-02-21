(function() {var implementors = {};
implementors["dualshock"] = [{"text":"impl Freeze for DS4Driver","synthetic":true,"types":[]},{"text":"impl Freeze for DS4","synthetic":true,"types":[]}];
implementors["dualshock_sys"] = [{"text":"impl&lt;Id&gt; Freeze for RawId&lt;Id&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for I16LE","synthetic":true,"types":[]},{"text":"impl Freeze for ConnectionType","synthetic":true,"types":[]},{"text":"impl Freeze for InputReport","synthetic":true,"types":[]},{"text":"impl Freeze for USBReport","synthetic":true,"types":[]},{"text":"impl Freeze for BTSimpleReport","synthetic":true,"types":[]},{"text":"impl Freeze for BTFullReport","synthetic":true,"types":[]},{"text":"impl Freeze for FullReport","synthetic":true,"types":[]},{"text":"impl Freeze for SimpleReport","synthetic":true,"types":[]},{"text":"impl Freeze for Type","synthetic":true,"types":[]},{"text":"impl Freeze for Trigger","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for Buttons&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Freeze for Dpad","synthetic":true,"types":[]},{"text":"impl Freeze for Gyro","synthetic":true,"types":[]},{"text":"impl Freeze for Accel","synthetic":true,"types":[]},{"text":"impl Freeze for Stick","synthetic":true,"types":[]},{"text":"impl Freeze for BTTrackpad","synthetic":true,"types":[]},{"text":"impl Freeze for USBTrackpad","synthetic":true,"types":[]},{"text":"impl Freeze for TrackpadPacket","synthetic":true,"types":[]},{"text":"impl Freeze for Finger","synthetic":true,"types":[]},{"text":"impl Freeze for FingerCoord","synthetic":true,"types":[]},{"text":"impl Freeze for InputReportId","synthetic":true,"types":[]}];
implementors["gyromouse"] = [{"text":"impl Freeze for ClickType","synthetic":true,"types":[]},{"text":"impl Freeze for GyroMouse","synthetic":true,"types":[]},{"text":"impl Freeze for CameraStick","synthetic":true,"types":[]},{"text":"impl Freeze for FlickStick","synthetic":true,"types":[]},{"text":"impl Freeze for ButtonStick","synthetic":true,"types":[]},{"text":"impl Freeze for FlickStickState","synthetic":true,"types":[]},{"text":"impl Freeze for Layer","synthetic":true,"types":[]},{"text":"impl Freeze for KeyState","synthetic":true,"types":[]},{"text":"impl Freeze for Buttons","synthetic":true,"types":[]},{"text":"impl Freeze for Action","synthetic":true,"types":[]},{"text":"impl Freeze for ExtAction","synthetic":true,"types":[]},{"text":"impl Freeze for KeyStatus","synthetic":true,"types":[]},{"text":"impl Freeze for VirtualKey","synthetic":true,"types":[]},{"text":"impl Freeze for MapKey","synthetic":true,"types":[]},{"text":"impl Freeze for Mouse","synthetic":true,"types":[]},{"text":"impl Freeze for JSMAction","synthetic":true,"types":[]},{"text":"impl Freeze for ActionModifier","synthetic":true,"types":[]},{"text":"impl Freeze for EventModifier","synthetic":true,"types":[]},{"text":"impl Freeze for ActionType","synthetic":true,"types":[]},{"text":"impl Freeze for Key","synthetic":true,"types":[]},{"text":"impl Freeze for SpecialKey","synthetic":true,"types":[]},{"text":"impl Freeze for Cmd","synthetic":true,"types":[]}];
implementors["hid_gamepad"] = [{"text":"impl Freeze for GamepadError","synthetic":true,"types":[]}];
implementors["hid_gamepad_sys"] = [{"text":"impl Freeze for Report","synthetic":true,"types":[]},{"text":"impl Freeze for Motion","synthetic":true,"types":[]},{"text":"impl Freeze for JoyKey","synthetic":true,"types":[]},{"text":"impl Freeze for KeyStatus","synthetic":true,"types":[]}];
implementors["joy_infrared"] = [{"text":"impl Freeze for Mouse","synthetic":true,"types":[]},{"text":"impl !Freeze for GUI","synthetic":true,"types":[]},{"text":"impl Freeze for UserEvent","synthetic":true,"types":[]},{"text":"impl Freeze for JoyconCmd","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !Freeze for BoundBuffer&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for Camera","synthetic":true,"types":[]},{"text":"impl Freeze for CameraController","synthetic":true,"types":[]},{"text":"impl !Freeze for Controls","synthetic":true,"types":[]},{"text":"impl Freeze for StyleSheet","synthetic":true,"types":[]},{"text":"impl Freeze for Sliderf32","synthetic":true,"types":[]},{"text":"impl Freeze for Message","synthetic":true,"types":[]},{"text":"impl Freeze for Vertex2D","synthetic":true,"types":[]},{"text":"impl !Freeze for D2","synthetic":true,"types":[]},{"text":"impl Freeze for D3","synthetic":true,"types":[]},{"text":"impl !Freeze for IRCompute","synthetic":true,"types":[]},{"text":"impl Freeze for Texture","synthetic":true,"types":[]},{"text":"impl Freeze for Uniforms","synthetic":true,"types":[]},{"text":"impl Freeze for Lights","synthetic":true,"types":[]},{"text":"impl Freeze for Light","synthetic":true,"types":[]}];
implementors["joycon"] = [{"text":"impl Freeze for Image","synthetic":true,"types":[]},{"text":"impl Freeze for Calibration","synthetic":true,"types":[]},{"text":"impl Freeze for Report","synthetic":true,"types":[]},{"text":"impl Freeze for JoyCon","synthetic":true,"types":[]},{"text":"impl Freeze for IMU","synthetic":true,"types":[]},{"text":"impl Freeze for JoyconDriver","synthetic":true,"types":[]}];
implementors["joycon_sys"] = [{"text":"impl Freeze for AccessoryCommand","synthetic":true,"types":[]},{"text":"impl Freeze for AccessoryResponse","synthetic":true,"types":[]},{"text":"impl Freeze for OfflineSteps","synthetic":true,"types":[]},{"text":"impl Freeze for AccessoryCommandId","synthetic":true,"types":[]},{"text":"impl Freeze for AccessoryType","synthetic":true,"types":[]},{"text":"impl Freeze for RingconItemId","synthetic":true,"types":[]},{"text":"impl Freeze for Error","synthetic":true,"types":[]},{"text":"impl Freeze for U16LE","synthetic":true,"types":[]},{"text":"impl Freeze for I16LE","synthetic":true,"types":[]},{"text":"impl Freeze for U32LE","synthetic":true,"types":[]},{"text":"impl&lt;Id&gt; Freeze for RawId&lt;Id&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for InputReportId","synthetic":true,"types":[]},{"text":"impl Freeze for SubcommandId","synthetic":true,"types":[]},{"text":"impl Freeze for Bool","synthetic":true,"types":[]},{"text":"impl Freeze for Frame","synthetic":true,"types":[]},{"text":"impl Freeze for Sensitivity","synthetic":true,"types":[]},{"text":"impl Freeze for IMUMode","synthetic":true,"types":[]},{"text":"impl Freeze for GyroSens","synthetic":true,"types":[]},{"text":"impl Freeze for AccSens","synthetic":true,"types":[]},{"text":"impl Freeze for GyroPerfRate","synthetic":true,"types":[]},{"text":"impl Freeze for AccAntiAliasing","synthetic":true,"types":[]},{"text":"impl Freeze for InputReport","synthetic":true,"types":[]},{"text":"impl Freeze for NormalInputReport","synthetic":true,"types":[]},{"text":"impl Freeze for StandardInputReport","synthetic":true,"types":[]},{"text":"impl Freeze for DeviceStatus","synthetic":true,"types":[]},{"text":"impl Freeze for ButtonsStatus","synthetic":true,"types":[]},{"text":"impl Freeze for RightButtons","synthetic":true,"types":[]},{"text":"impl Freeze for MiddleButtons","synthetic":true,"types":[]},{"text":"impl Freeze for LeftButtons","synthetic":true,"types":[]},{"text":"impl Freeze for Stick","synthetic":true,"types":[]},{"text":"impl Freeze for SubcommandReply","synthetic":true,"types":[]},{"text":"impl Freeze for Ack","synthetic":true,"types":[]},{"text":"impl Freeze for DeviceInfo","synthetic":true,"types":[]},{"text":"impl Freeze for FirmwareVersion","synthetic":true,"types":[]},{"text":"impl Freeze for MACAddress","synthetic":true,"types":[]},{"text":"impl Freeze for IMUMCU","synthetic":true,"types":[]},{"text":"impl Freeze for DeviceType","synthetic":true,"types":[]},{"text":"impl Freeze for BatteryLevel","synthetic":true,"types":[]},{"text":"impl Freeze for Button","synthetic":true,"types":[]},{"text":"impl Freeze for WhichController","synthetic":true,"types":[]},{"text":"impl Freeze for UseSPIColors","synthetic":true,"types":[]},{"text":"impl Freeze for PlayerLights","synthetic":true,"types":[]},{"text":"impl Freeze for HomeLight","synthetic":true,"types":[]},{"text":"impl Freeze for PlayerLight","synthetic":true,"types":[]},{"text":"impl Freeze for MCUReport","synthetic":true,"types":[]},{"text":"impl Freeze for MCUStatus","synthetic":true,"types":[]},{"text":"impl Freeze for MCUCommand","synthetic":true,"types":[]},{"text":"impl Freeze for MCUCommandCRC","synthetic":true,"types":[]},{"text":"impl Freeze for MCURequest","synthetic":true,"types":[]},{"text":"impl Freeze for MCURequestCRC","synthetic":true,"types":[]},{"text":"impl Freeze for MCUReportId","synthetic":true,"types":[]},{"text":"impl Freeze for MCUCommandId","synthetic":true,"types":[]},{"text":"impl Freeze for MCUSubCommandId","synthetic":true,"types":[]},{"text":"impl Freeze for MCUMode","synthetic":true,"types":[]},{"text":"impl Freeze for MCURequestId","synthetic":true,"types":[]},{"text":"impl Freeze for MCURequestEnum","synthetic":true,"types":[]},{"text":"impl Freeze for Register","synthetic":true,"types":[]},{"text":"impl Freeze for Leds","synthetic":true,"types":[]},{"text":"impl Freeze for IRRequest","synthetic":true,"types":[]},{"text":"impl Freeze for IRAckRequestPacket","synthetic":true,"types":[]},{"text":"impl Freeze for IRReadRegisters","synthetic":true,"types":[]},{"text":"impl Freeze for MCUIRModeData","synthetic":true,"types":[]},{"text":"impl Freeze for IRStatus","synthetic":true,"types":[]},{"text":"impl Freeze for IRRegistersSlice","synthetic":true,"types":[]},{"text":"impl Freeze for IRData","synthetic":true,"types":[]},{"text":"impl Freeze for MCURegisters","synthetic":true,"types":[]},{"text":"impl Freeze for MCUSetReg","synthetic":true,"types":[]},{"text":"impl Freeze for Resolution","synthetic":true,"types":[]},{"text":"impl Freeze for ExposureMode","synthetic":true,"types":[]},{"text":"impl Freeze for ExternalLightFilter","synthetic":true,"types":[]},{"text":"impl Freeze for Flip","synthetic":true,"types":[]},{"text":"impl Freeze for IRRequestId","synthetic":true,"types":[]},{"text":"impl Freeze for IRRequestEnum","synthetic":true,"types":[]},{"text":"impl Freeze for MCUIRMode","synthetic":true,"types":[]},{"text":"impl Freeze for OutputReport","synthetic":true,"types":[]},{"text":"impl Freeze for SubcommandRequest","synthetic":true,"types":[]},{"text":"impl Freeze for RumbleData","synthetic":true,"types":[]},{"text":"impl Freeze for RumbleSide","synthetic":true,"types":[]},{"text":"impl Freeze for OutputReportId","synthetic":true,"types":[]},{"text":"impl Freeze for SPIRange","synthetic":true,"types":[]},{"text":"impl Freeze for WrongRangeError","synthetic":true,"types":[]},{"text":"impl Freeze for SPIReadRequest","synthetic":true,"types":[]},{"text":"impl Freeze for SPIWriteRequest","synthetic":true,"types":[]},{"text":"impl Freeze for SPIReadResult","synthetic":true,"types":[]},{"text":"impl Freeze for SPIWriteResult","synthetic":true,"types":[]},{"text":"impl Freeze for SticksCalibration","synthetic":true,"types":[]},{"text":"impl Freeze for UserSticksCalibration","synthetic":true,"types":[]},{"text":"impl Freeze for LeftStickCalibration","synthetic":true,"types":[]},{"text":"impl Freeze for RightStickCalibration","synthetic":true,"types":[]},{"text":"impl Freeze for UserStickCalibration","synthetic":true,"types":[]},{"text":"impl Freeze for SensorCalibration","synthetic":true,"types":[]},{"text":"impl Freeze for UserSensorCalibration","synthetic":true,"types":[]},{"text":"impl Freeze for Color","synthetic":true,"types":[]},{"text":"impl Freeze for ControllerColor","synthetic":true,"types":[]}];
implementors["joytk"] = [{"text":"impl Freeze for Opts","synthetic":true,"types":[]},{"text":"impl Freeze for Calibrate","synthetic":true,"types":[]},{"text":"impl Freeze for Set","synthetic":true,"types":[]},{"text":"impl Freeze for SetColor","synthetic":true,"types":[]},{"text":"impl Freeze for Ringcon","synthetic":true,"types":[]},{"text":"impl Freeze for Relay","synthetic":true,"types":[]},{"text":"impl Freeze for SubCommand","synthetic":true,"types":[]},{"text":"impl Freeze for CalibrateE","synthetic":true,"types":[]},{"text":"impl Freeze for SetE","synthetic":true,"types":[]},{"text":"impl Freeze for RingconE","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()