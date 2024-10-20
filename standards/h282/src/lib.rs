#![doc = include_str!("../README.md")]
#![no_std]

use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "SEQUENCE")]
pub struct AnonymousAudioInputsCapabilityAvailableDevices {
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceIdentifier")]
    pub device_identifier: DeviceID,
}

impl AnonymousAudioInputsCapabilityAvailableDevices {
    pub fn new(device_class: DeviceClass, device_identifier: DeviceID) -> Self {
        Self {
            device_class,
            device_identifier,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2..=64"))]
pub struct AudioInputsCapabilityAvailableDevices(
    pub SetOf<AnonymousAudioInputsCapabilityAvailableDevices>,
);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct AudioInputsCapability {
    #[rasn(value("2..=64"), identifier = "numberOfDeviceInputs")]
    pub number_of_device_inputs: u8,
    #[rasn(size("2..=64"), identifier = "availableDevices")]
    pub available_devices: Option<SetOf<AudioInputsCapabilityAvailableDevices>>,
}

impl AudioInputsCapability {
    pub fn new(
        number_of_device_inputs: u8,
        available_devices: Option<SetOf<AudioInputsCapabilityAvailableDevices>>,
    ) -> Self {
        Self {
            number_of_device_inputs,
            available_devices,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct AutoSlideDisplayTime(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum AutoSlideShowControl {
    Start,
    Stop,
    Pause,
}

#[doc = " 100ths of a degree/sec"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct BackLight(pub u8);

#[doc = " Anonymous SET OF member "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "SEQUENCE")]
pub struct AnonymousCameraFilterCapabilityFilterTextLabel {
    #[rasn(value("1..=255"), identifier = "filterNumber")]
    pub filter_number: u8,
    #[rasn(identifier = "filterTextLabel")]
    pub filter_text_label: DeviceText,
}

impl AnonymousCameraFilterCapabilityFilterTextLabel {
    pub fn new(filter_number: u8, filter_text_label: DeviceText) -> Self {
        Self {
            filter_number,
            filter_text_label,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=255"))]
pub struct CameraFilterCapabilityFilterTextLabel(
    pub SetOf<AnonymousCameraFilterCapabilityFilterTextLabel>,
);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CameraFilterCapability {
    #[rasn(value("2..=255"), identifier = "maxNumberOfFilters")]
    pub max_number_of_filters: u8,
    #[rasn(size("0..=255"), identifier = "filterTextLabel")]
    pub filter_text_label: Option<SetOf<CameraFilterCapabilityFilterTextLabel>>,
}

impl CameraFilterCapability {
    pub fn new(
        max_number_of_filters: u8,
        filter_text_label: Option<SetOf<CameraFilterCapabilityFilterTextLabel>>,
    ) -> Self {
        Self {
            max_number_of_filters,
            filter_text_label,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct CameraFilterNumber(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CameraFocusedToLimit {
    Near,
    Far,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "SEQUENCE")]
pub struct AnonymousCameraLensCapabilityAccessoryTextLabel {
    #[rasn(value("1..=255"), identifier = "lensNumber")]
    pub lens_number: u8,
    #[rasn(identifier = "lensTextLabel")]
    pub lens_text_label: DeviceText,
}

impl AnonymousCameraLensCapabilityAccessoryTextLabel {
    pub fn new(lens_number: u8, lens_text_label: DeviceText) -> Self {
        Self {
            lens_number,
            lens_text_label,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=255"))]
pub struct CameraLensCapabilityAccessoryTextLabel(
    pub SetOf<AnonymousCameraLensCapabilityAccessoryTextLabel>,
);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CameraLensCapability {
    #[rasn(value("2..=255"), identifier = "maxNumberOfLens")]
    pub max_number_of_lens: u8,
    #[rasn(size("0..=255"), identifier = "accessoryTextLabel")]
    pub accessory_text_label: Option<SetOf<CameraLensCapabilityAccessoryTextLabel>>,
}

impl CameraLensCapability {
    pub fn new(
        max_number_of_lens: u8,
        accessory_text_label: Option<SetOf<CameraLensCapabilityAccessoryTextLabel>>,
    ) -> Self {
        Self {
            max_number_of_lens,
            accessory_text_label,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct CameraLensNumber(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=18000"))]
pub struct CameraPanSpeed(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CameraPanSpeedCapability {
    #[rasn(identifier = "maxSpeed")]
    pub max_speed: CameraPanSpeed,
    #[rasn(identifier = "minSpeed")]
    pub min_speed: CameraPanSpeed,
    #[rasn(identifier = "speedStepSize")]
    pub speed_step_size: CameraPanSpeed,
}
impl CameraPanSpeedCapability {
    pub fn new(
        max_speed: CameraPanSpeed,
        min_speed: CameraPanSpeed,
        speed_step_size: CameraPanSpeed,
    ) -> Self {
        Self {
            max_speed,
            min_speed,
            speed_step_size,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CameraPannedToLimit {
    Left,
    Right,
}

/// 100ths of a degree/sec
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=18000"))]
pub struct CameraTiltSpeed(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CameraTiltSpeedCapability {
    #[rasn(identifier = "maxSpeed")]
    pub max_speed: CameraTiltSpeed,
    #[rasn(identifier = "minSpeed")]
    pub min_speed: CameraTiltSpeed,
    #[rasn(identifier = "speedStepSize")]
    pub speed_step_size: CameraTiltSpeed,
}
impl CameraTiltSpeedCapability {
    pub fn new(
        max_speed: CameraTiltSpeed,
        min_speed: CameraTiltSpeed,
        speed_step_size: CameraTiltSpeed,
    ) -> Self {
        Self {
            max_speed,
            min_speed,
            speed_step_size,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CameraTiltedToLimit {
    Up,
    Down,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CameraZoomedToLimit {
    Telescopic,
    Wide,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CapabilityID {
    #[rasn(value("0..=65535"))]
    Standard(u16),
    NonStandard(Key),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ConfigureDeviceEventsRequest {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
    #[rasn(identifier = "deviceEventIdentifierList")]
    pub device_event_identifier_list: SetOf<DeviceEventIdentifier>,
}
impl ConfigureDeviceEventsRequest {
    pub fn new(
        request_handle: Handle,
        device_class: DeviceClass,
        device_id: DeviceID,
        device_event_identifier_list: SetOf<DeviceEventIdentifier>,
    ) -> Self {
        Self {
            request_handle,
            device_class,
            device_id,
            device_event_identifier_list,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ConfigureDeviceEventsResponseResult {
    Successful,
    RequestDenied,
    UnknownDevice,
    DeviceUnavailable,
    DeviceAttributeError,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ConfigureDeviceEventsResponse {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    pub result: ConfigureDeviceEventsResponseResult,
}
impl ConfigureDeviceEventsResponse {
    pub fn new(request_handle: Handle, result: ConfigureDeviceEventsResponseResult) -> Self {
        Self {
            request_handle,
            result,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ControlAttribute {
    SetDeviceState(DeviceState),
    SetDeviceDate(DeviceDate),
    SetDeviceTime(DeviceTime),
    SetDevicePreset(DevicePreset),
    SetIrisMode(Mode),
    SetFocusMode(Mode),
    SetBackLightMode(Mode),
    SetPointingMode(PointingToggle),
    SelectCameraLens(CameraLensNumber),
    SelectCameraFilter(CameraFilterNumber),
    GotoHomePosition,
    SelectExternalLight(SelectExternalLight),
    ClearCameraLens,
    SetCameraPanSpeed(CameraPanSpeed),
    SetCameraTiltSpeed(CameraTiltSpeed),
    SetBackLight(BackLight),
    SetWhiteBalance(WhiteBalance),
    SetWhiteBalanceMode(Mode),
    CalibrateWhiteBalance,
    FocusImage,
    CaptureImage,
    PanContinuous(PanContinuous),
    TiltContinuous(TiltContinuous),
    ZoomContinuous(ZoomContinuous),
    FocusContinuous(FocusContinuous),
    SetZoomPosition(SetZoomPosition),
    SetFocusPosition(SetFocusPosition),
    SetIrisPosition(SetIrisPosition),
    SetPanPosition(SetPanPosition),
    SetTiltPosition(SetTiltPosition),
    SetZoomMagnification(ZoomMagnification),
    SetPanView(PanView),
    SetTiltView(TiltView),
    SelectSlide(SlideNumber),
    SelectNextSlide(SelectDirection),
    PlayAutoSlideShow(AutoSlideShowControl),
    SetAutoSlideDisplayTime(AutoSlideDisplayTime),
    ContinuousRewindControl(bool),
    ContinuousFastForwardControl(bool),
    SearchBackwardsControl(bool),
    SearchForwardsControl(bool),
    Pause(bool),
    SelectProgram(ProgramNumber),
    NextProgramSelect(SelectDirection),
    GotoNormalPlayTimePoint(ProgramDuration),
    ContinuousPlayBackMode(bool),
    SetPlaybackSpeed(PlaybackSpeed),
    Play(bool),
    SetAudioOutputMute(bool),
    PlayToNormalPlayTimePoint(ProgramDuration),
    Record(bool),
    RecordForDuration(RecordForDuration),
    ConfigureVideoInputs(DeviceInputs),
    ConfigureAudioInputs(DeviceInputs),
    NonStandardControl(NonStandardParameter),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentAudioOutputMute {
    Mute(bool),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentAutoSlideDisplayTime {
    Time(AutoSlideDisplayTime),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentBackLight {
    BackLight(BackLight),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentCameraFilterNumber {
    LensNumber(CameraFilterNumber),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentCameraLensNumber {
    LensNumber(CameraLensNumber),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentCameraPanSpeed {
    Speed(CameraPanSpeed),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentCameraTiltSpeed {
    Speed(CameraTiltSpeed),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentDeviceDateCurrentDay {
    Day(Day),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentDeviceDateCurrentMonth {
    Month(Month),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentDeviceDateCurrentYear {
    Year(Year),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CurrentDeviceDate {
    #[rasn(identifier = "currentDay")]
    pub current_day: CurrentDeviceDateCurrentDay,
    #[rasn(identifier = "currentMonth")]
    pub current_month: CurrentDeviceDateCurrentMonth,
    #[rasn(identifier = "currentYear")]
    pub current_year: CurrentDeviceDateCurrentYear,
}
impl CurrentDeviceDate {
    pub fn new(
        current_day: CurrentDeviceDateCurrentDay,
        current_month: CurrentDeviceDateCurrentMonth,
        current_year: CurrentDeviceDateCurrentYear,
    ) -> Self {
        Self {
            current_day,
            current_month,
            current_year,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentDevicePreset {
    Preset(PresetNumber),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentDeviceState {
    DeviceState(DeviceState),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentDeviceTimeCurrentHour {
    Hour(Hour),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentDeviceTimeCurrentMinute {
    Minute(Minute),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CurrentDeviceTime {
    #[rasn(identifier = "currentHour")]
    pub current_hour: CurrentDeviceTimeCurrentHour,
    #[rasn(identifier = "currentMinute")]
    pub current_minute: CurrentDeviceTimeCurrentMinute,
}
impl CurrentDeviceTime {
    pub fn new(
        current_hour: CurrentDeviceTimeCurrentHour,
        current_minute: CurrentDeviceTimeCurrentMinute,
    ) -> Self {
        Self {
            current_hour,
            current_minute,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentExternalLight {
    #[rasn(value("1..=10"))]
    LightNumber(u8),
    None,
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentFocusPosition {
    FocusPosition(FocusPosition),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentIrisPosition {
    IrisPosition(IrisPosition),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentMode {
    Mode(Mode),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentPanPosition {
    PanPosition(PanPosition),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentPlaybackSpeed {
    Speed(PlaybackSpeed),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentPointingMode {
    Automatic,
    Manual,
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentSelectedProgram {
    Program(ProgramNumber),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentSlide {
    Slide(SlideNumber),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentStreamPlayerState {
    State(StreamPlayerState),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentTiltPosition {
    TiltPosition(TiltPosition),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentWhiteBalance {
    WhiteBalance(WhiteBalance),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum CurrentZoomPosition {
    ZoomPosition(ZoomPosition),
    Unknown,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=31"))]
pub struct Day(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum DeviceAttribute {
    DeviceStateSupported,
    DeviceDateSupported,
    DeviceTimeSupported,
    DevicePresetSupported(DevicePresetCapability),
    IrisModeSupported,
    FocusModeSupported,
    PointingModeSupported,
    CameraLensSupported(CameraLensCapability),
    CameraFilterSupported(CameraFilterCapability),
    HomePositionSupported,
    ExternalCameraLightSupported(ExternalCameraLightCapability),
    ClearCameraLensSupported,
    CameraPanSpeedSupported(CameraPanSpeedCapability),
    CameraTiltSpeedSupported(CameraTiltSpeedCapability),
    BackLightModeSupported,
    BackLightSettingSupported(MaxBacklight),
    WhiteBalanceSettingSupported(MaxWhiteBalance),
    WhiteBalanceModeSupported,
    CalibrateWhiteBalanceSupported,
    FocusImageSupported,
    CaptureImageSupported,
    PanContinuousSupported,
    TiltContinuousSupported,
    ZoomContinuousSupported,
    FocusContinuousSupported,
    IrisContinuousSupported,
    ZoomPositionSupported(MinZoomPositionSetSize),
    FocusPositionSupported(MinFocusPositionStepSize),
    IrisPositionSupported(MinIrisPositionStepSize),
    PanPositionSupported(PanPositionCapability),
    TiltPositionSupported(TiltPositionCapability),
    ZoomMagnificationSupported(MinZoomMagnificationStepSize),
    PanViewSupported,
    TiltViewSupported,
    SelectSlideSupported(MaxNumberOfSlides),
    SelectNextSlideSupported,
    SlideShowModeSupported,
    PlaySlideShowSupported,
    SetSlideDisplayTimeSupported(MaxSlideDisplayTime),
    ContinuousRewindSupported,
    ContinuousFastForwardSupported,
    SearchBackwardsSupported,
    SearchForwardsSupported,
    PauseSupported,
    SelectProgramSupported(MaxNumberOfPrograms),
    NextProgramSupported,
    GotoNormalPlayTimePointSupported,
    ReadStreamPlayerStateSupported,
    ReadProgramDurationSupported,
    ContinuousPlayBackModeSupported,
    PlaybackSpeedSupported(PlayBackSpeedCapability),
    PlaySupported,
    SetAudioOutputStateSupported,
    PlayToNormalPlayTimePointSupported,
    RecordSupported,
    RecordForDurationSupported,
    ConfigurableVideoInputsSupported(VideoInputsCapability),
    VideoInputsSupported(VideoInputsCapability),
    ConfigurableAudioInputsSupported(AudioInputsCapability),
    AudioInputsSupported(AudioInputsCapability),
    DeviceLockStateChangedSupported,
    DeviceAvailabilityChangedSupported,
    CameraPannedToLimitSupported,
    CameraTiltedToLimitSupported,
    CameraZoomedToLimitSupported,
    CameraFocusedToLimitSupported,
    AutoSlideShowFinishedSupported,
    StreamPlayerStateChangeSupported,
    StreamPlayerProgramChangeSupported,
    NonStandardAttributeSupported(NonStandardParameter),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceAttributeRequest {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
}
impl DeviceAttributeRequest {
    pub fn new(request_handle: Handle, device_class: DeviceClass, device_id: DeviceID) -> Self {
        Self {
            request_handle,
            device_class,
            device_id,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum DeviceAttributeResponseResult {
    Successful,
    RequestDenied,
    UnknownDevice,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceAttributeResponse {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceAttributeList")]
    pub device_attribute_list: Option<SetOf<DeviceAttribute>>,
    pub result: DeviceAttributeResponseResult,
}
impl DeviceAttributeResponse {
    pub fn new(
        request_handle: Handle,
        device_attribute_list: Option<SetOf<DeviceAttribute>>,
        result: DeviceAttributeResponseResult,
    ) -> Self {
        Self {
            request_handle,
            device_attribute_list,
            result,
        }
    }
}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum DeviceClass {
    Camera,
    Microphone,
    StreamPlayerRecorder,
    SlideProjector,
    LightSource,
    SourceCombiner,
    NonStandardDevice(NonStandardIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceControlRequest {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
    #[rasn(size("1..=8"), identifier = "controlAttributeList")]
    pub control_attribute_list: SetOf<ControlAttribute>,
}
impl DeviceControlRequest {
    pub fn new(
        request_handle: Handle,
        device_class: DeviceClass,
        device_id: DeviceID,
        control_attribute_list: SetOf<ControlAttribute>,
    ) -> Self {
        Self {
            request_handle,
            device_class,
            device_id,
            control_attribute_list,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct DeviceDate {
    pub day: Day,
    pub month: Month,
    pub year: Year,
}
impl DeviceDate {
    pub fn new(day: Day, month: Month, year: Year) -> Self {
        Self { day, month, year }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum DeviceEvent {
    DeviceLockChanged(bool),
    DeviceAvailabilityChanged(bool),
    CameraPannedToLimit(CameraPannedToLimit),
    CameraTiltedToLimit(CameraTiltedToLimit),
    CameraZoomedToLimit(CameraZoomedToLimit),
    CameraFocusedToLimit(CameraFocusedToLimit),
    AutoSlideShowFinished,
    StreamPlayerStateChange(StreamPlayerState),
    StreamPlayerProgramChange(ProgramNumber),
    NonStandardEvent(NonStandardParameter),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum DeviceEventIdentifier {
    RequestDeviceLockChanged,
    RequestDeviceAvailabilityChanged,
    RequestCameraPannedToLimit,
    RequestCameraTiltedToLimit,
    RequestCameraZoomedToLimit,
    RequestCameraFocusedToLimit,
    RequestAutoSlideShowFinished,
    RequestStreamPlayerStateChange,
    RequestStreamPlayerProgramChange,
    RequestNonStandardEvent(NonStandardIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceEventNotifyIndication {
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
    #[rasn(size("1..=8"), identifier = "deviceEventList")]
    pub device_event_list: SetOf<DeviceEvent>,
}
impl DeviceEventNotifyIndication {
    pub fn new(
        device_class: DeviceClass,
        device_id: DeviceID,
        device_event_list: SetOf<DeviceEvent>,
    ) -> Self {
        Self {
            device_class,
            device_id,
            device_event_list,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=127"))]
pub struct DeviceID(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "SEQUENCE")]
pub struct AnonymousDeviceInputsInputDevices {
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceIdentifier")]
    pub device_identifier: DeviceID,
}
impl AnonymousDeviceInputsInputDevices {
    pub fn new(device_class: DeviceClass, device_identifier: DeviceID) -> Self {
        Self {
            device_class,
            device_identifier,
        }
    }
}
#[doc = " Inner type "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2..=64"))]
pub struct DeviceInputsInputDevices(pub SetOf<AnonymousDeviceInputsInputDevices>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct DeviceInputs {
    #[rasn(size("2..=64"), identifier = "inputDevices")]
    pub input_devices: SetOf<DeviceInputsInputDevices>,
}

impl DeviceInputs {
    pub fn new(input_devices: SetOf<DeviceInputsInputDevices>) -> Self {
        Self { input_devices }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceLockEnquireRequest {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
}
impl DeviceLockEnquireRequest {
    pub fn new(request_handle: Handle, device_class: DeviceClass, device_id: DeviceID) -> Self {
        Self {
            request_handle,
            device_class,
            device_id,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum DeviceLockEnquireResponseResult {
    LockRequired,
    LockNotRequired,
    UnknownDevice,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceLockEnquireResponse {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    pub result: DeviceLockEnquireResponseResult,
}
impl DeviceLockEnquireResponse {
    pub fn new(request_handle: Handle, result: DeviceLockEnquireResponseResult) -> Self {
        Self {
            request_handle,
            result,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceLockRequest {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
    #[rasn(identifier = "lockFlag")]
    pub lock_flag: bool,
}
impl DeviceLockRequest {
    pub fn new(
        request_handle: Handle,
        device_class: DeviceClass,
        device_id: DeviceID,
        lock_flag: bool,
    ) -> Self {
        Self {
            request_handle,
            device_class,
            device_id,
            lock_flag,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum DeviceLockResponseResult {
    Successful,
    RequestDenied,
    UnknownDevice,
    LockingNotSupported,
    DeviceAlreadyLocked,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceLockResponse {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    pub result: DeviceLockResponseResult,
}
impl DeviceLockResponse {
    pub fn new(request_handle: Handle, result: DeviceLockResponseResult) -> Self {
        Self {
            request_handle,
            result,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceLockTerminatedIndication {
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
}
impl DeviceLockTerminatedIndication {
    pub fn new(device_class: DeviceClass, device_id: DeviceID) -> Self {
        Self {
            device_class,
            device_id,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum DevicePresetMode {
    Store,
    Activate,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct DevicePreset {
    #[rasn(identifier = "presetNumber")]
    pub preset_number: PresetNumber,
    pub mode: DevicePresetMode,
}
impl DevicePreset {
    pub fn new(preset_number: PresetNumber, mode: DevicePresetMode) -> Self {
        Self {
            preset_number,
            mode,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "SEQUENCE")]
pub struct AnonymousDevicePresetCapabilityPresetCapability {
    #[rasn(identifier = "presetNumber")]
    pub preset_number: PresetNumber,
    #[rasn(identifier = "storeModeSupported")]
    pub store_mode_supported: bool,
    #[rasn(identifier = "presetTextLabel")]
    pub preset_text_label: DeviceText,
}
impl AnonymousDevicePresetCapabilityPresetCapability {
    pub fn new(
        preset_number: PresetNumber,
        store_mode_supported: bool,
        preset_text_label: DeviceText,
    ) -> Self {
        Self {
            preset_number,
            store_mode_supported,
            preset_text_label,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=255"))]
pub struct DevicePresetCapabilityPresetCapability(
    pub SetOf<AnonymousDevicePresetCapabilityPresetCapability>,
);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct DevicePresetCapability {
    #[rasn(identifier = "maxNumber")]
    pub max_number: PresetNumber,
    #[rasn(size("0..=255"), identifier = "presetCapability")]
    pub preset_capability: Option<SetOf<DevicePresetCapabilityPresetCapability>>,
}
impl DevicePresetCapability {
    pub fn new(
        max_number: PresetNumber,
        preset_capability: Option<SetOf<DevicePresetCapabilityPresetCapability>>,
    ) -> Self {
        Self {
            max_number,
            preset_capability,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceProfile {
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
    #[rasn(identifier = "audioSourceFlag")]
    pub audio_source_flag: bool,
    #[rasn(identifier = "audioSinkFlag")]
    pub audio_sink_flag: bool,
    #[rasn(identifier = "videoSourceFlag")]
    pub video_source_flag: bool,
    #[rasn(identifier = "videoSinkFlag")]
    pub video_sink_flag: bool,
    #[rasn(identifier = "remoteControlFlag")]
    pub remote_control_flag: bool,
    #[rasn(value("0..=255"), identifier = "instanceNumber")]
    pub instance_number: u8,
    #[rasn(identifier = "deviceName")]
    pub device_name: Option<TextString>,
}
impl DeviceProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device_id: DeviceID,
        audio_source_flag: bool,
        audio_sink_flag: bool,
        video_source_flag: bool,
        video_sink_flag: bool,
        remote_control_flag: bool,
        instance_number: u8,
        device_name: Option<TextString>,
    ) -> Self {
        Self {
            device_id,
            audio_source_flag,
            audio_sink_flag,
            video_source_flag,
            video_sink_flag,
            remote_control_flag,
            instance_number,
            device_name,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum DeviceState {
    Active,
    Inactive,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceStatusEnquireRequest {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
    #[rasn(size("1..=16"), identifier = "statusAttributeIdentifierList")]
    pub status_attribute_identifier_list: SetOf<StatusAttributeIdentifier>,
}
impl DeviceStatusEnquireRequest {
    pub fn new(
        request_handle: Handle,
        device_class: DeviceClass,
        device_id: DeviceID,
        status_attribute_identifier_list: SetOf<StatusAttributeIdentifier>,
    ) -> Self {
        Self {
            request_handle,
            device_class,
            device_id,
            status_attribute_identifier_list,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum DeviceStatusEnquireResponseResult {
    Successful,
    RequestDenied,
    UnknownDevice,
    DeviceUnavailable,
    DeviceAttributeError,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DeviceStatusEnquireResponse {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(size("1..=16"), identifier = "statusAttributeList")]
    pub status_attribute_list: Option<SetOf<StatusAttribute>>,
    pub result: DeviceStatusEnquireResponseResult,
}
impl DeviceStatusEnquireResponse {
    pub fn new(
        request_handle: Handle,
        status_attribute_list: Option<SetOf<StatusAttribute>>,
        result: DeviceStatusEnquireResponseResult,
    ) -> Self {
        Self {
            request_handle,
            status_attribute_list,
            result,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=32"))]
pub struct DeviceText(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct DeviceTime {
    pub hour: Hour,
    pub minute: Minute,
}
impl DeviceTime {
    pub fn new(hour: Hour, minute: Minute) -> Self {
        Self { hour, minute }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "SEQUENCE")]
pub struct AnonymousExternalCameraLightCapabilityLightTextLabel {
    #[rasn(value("1..=10"), identifier = "lightNumber")]
    pub light_number: u8,
    #[rasn(identifier = "lightLabel")]
    pub light_label: DeviceText,
}
impl AnonymousExternalCameraLightCapabilityLightTextLabel {
    pub fn new(light_number: u8, light_label: DeviceText) -> Self {
        Self {
            light_number,
            light_label,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=10"))]
pub struct ExternalCameraLightCapabilityLightTextLabel(
    pub SetOf<AnonymousExternalCameraLightCapabilityLightTextLabel>,
);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct ExternalCameraLightCapability {
    #[rasn(value("1..=10"), identifier = "maxNumber")]
    pub max_number: u8,
    #[rasn(size("0..=10"), identifier = "lightTextLabel")]
    pub light_text_label: Option<SetOf<ExternalCameraLightCapabilityLightTextLabel>>,
}
impl ExternalCameraLightCapability {
    pub fn new(
        max_number: u8,
        light_text_label: Option<SetOf<ExternalCameraLightCapabilityLightTextLabel>>,
    ) -> Self {
        Self {
            max_number,
            light_text_label,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum FocusContinuousFocusDirection {
    Near,
    Far,
    Stop,
    Continue,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct FocusContinuous {
    #[rasn(identifier = "focusDirection")]
    pub focus_direction: FocusContinuousFocusDirection,
    #[rasn(value("50..=1000"), identifier = "timeOut")]
    pub time_out: u16,
}
impl FocusContinuous {
    pub fn new(focus_direction: FocusContinuousFocusDirection, time_out: u16) -> Self {
        Self {
            focus_direction,
            time_out,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-127..=127"))]
pub struct FocusPosition(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("4..=255"))]
pub struct H221NonStandardIdentifier(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct Handle(pub u32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=23"))]
pub struct Hour(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IndicationPdu {
    SourceChangeEventIndication(SourceChangeEventIndication),
    DeviceLockTerminatedIndication(DeviceLockTerminatedIndication),
    DeviceEventNotifyIndication(DeviceEventNotifyIndication),
    NonStandardIndication(NonStandardPdu),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum IrisContinuousIrisDirection {
    Darker,
    Lighter,
    Stop,
    Continue,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct IrisContinuous {
    #[rasn(identifier = "irisDirection")]
    pub iris_direction: IrisContinuousIrisDirection,
    #[rasn(value("50..=1000"), identifier = "timeOut")]
    pub time_out: u16,
}
impl IrisContinuous {
    pub fn new(iris_direction: IrisContinuousIrisDirection, time_out: u16) -> Self {
        Self {
            iris_direction,
            time_out,
        }
    }
}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-127..=127"))]
pub struct IrisPosition(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum Key {
    Object(ObjectIdentifier),
    H221NonStandard(H221NonStandardIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct MaxBacklight(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=1023"))]
pub struct MaxNumberOfPrograms(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=1024"))]
pub struct MaxNumberOfSlides(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct MaxSlideDisplayTime(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct MaxWhiteBalance(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct MinFocusPositionStepSize(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct MinIrisPositionStepSize(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=1000"))]
pub struct MinZoomMagnificationStepSize(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=1023"))]
pub struct MinZoomPositionSetSize(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=59"))]
pub struct Minute(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum Mode {
    Manual,
    Auto,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=12"))]
pub struct Month(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum AnonymousNonCollapsingCapabilitiesApplicationData {
    #[rasn(size("0..=127"))]
    DeviceList(SetOf<DeviceProfile>),
    #[rasn(size("0..=127"))]
    StreamList(SetOf<StreamProfile>),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "SEQUENCE")]
pub struct AnonymousNonCollapsingCapabilities {
    #[rasn(identifier = "capabilityID")]
    pub capability_id: CapabilityID,
    #[rasn(identifier = "applicationData")]
    pub application_data: AnonymousNonCollapsingCapabilitiesApplicationData,
}
impl AnonymousNonCollapsingCapabilities {
    pub fn new(
        capability_id: CapabilityID,
        application_data: AnonymousNonCollapsingCapabilitiesApplicationData,
    ) -> Self {
        Self {
            capability_id,
            application_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct NonCollapsingCapabilities(pub SetOf<AnonymousNonCollapsingCapabilities>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum NonStandardIdentifier {
    Object(ObjectIdentifier),
    H221NonStandard(H221NonStandardIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct NonStandardPdu {
    #[rasn(identifier = "nonStandardData")]
    pub non_standard_data: NonStandardParameter,
}

impl NonStandardPdu {
    pub fn new(non_standard_data: NonStandardParameter) -> Self {
        Self { non_standard_data }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct NonStandardParameter {
    pub key: Key,
    pub data: OctetString,
}
impl NonStandardParameter {
    pub fn new(key: Key, data: OctetString) -> Self {
        Self { key, data }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum PanContinuousPanDirection {
    Left,
    Right,
    Stop,
    Continue,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct PanContinuous {
    #[rasn(identifier = "panDirection")]
    pub pan_direction: PanContinuousPanDirection,
    #[rasn(value("50..=1000"), identifier = "timeOut")]
    pub time_out: u16,
}
impl PanContinuous {
    pub fn new(pan_direction: PanContinuousPanDirection, time_out: u16) -> Self {
        Self {
            pan_direction,
            time_out,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-18000..=18000"))]
pub struct PanPosition(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct PanPositionCapability {
    #[rasn(value("-18000..=0"), identifier = "maxLeft")]
    pub max_left: i16,
    #[rasn(value("0..=18000"), identifier = "maxRight")]
    pub max_right: u16,
    #[rasn(value("1..=18000"), identifier = "minStepSize")]
    pub min_step_size: u16,
}
impl PanPositionCapability {
    pub fn new(max_left: i16, max_right: u16, min_step_size: u16) -> Self {
        Self {
            max_left,
            max_right,
            min_step_size,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-1000..=1000"))]
pub struct PanView(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct PlayBackSpeedCapability {
    #[rasn(size("1..=64"), identifier = "multiplierFactors")]
    pub multiplier_factors: SetOf<u16>,
    #[rasn(size("1..=64"), identifier = "divisorFactors")]
    pub divisor_factors: SetOf<u16>,
}
impl PlayBackSpeedCapability {
    pub fn new(multiplier_factors: SetOf<u16>, divisor_factors: SetOf<u16>) -> Self {
        Self {
            multiplier_factors,
            divisor_factors,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct PlaybackSpeed {
    #[rasn(value("10..=1000"), identifier = "scaleFactor")]
    pub scale_factor: u16,
    #[rasn(identifier = "multiplyFactor")]
    pub multiply_factor: bool,
}
impl PlaybackSpeed {
    pub fn new(scale_factor: u16, multiply_factor: bool) -> Self {
        Self {
            scale_factor,
            multiply_factor,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum PointingToggle {
    Manual,
    Auto,
    Toggle,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum PositioningMode {
    Relative,
    Absolute,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct PresetNumber(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct ProgramDuration {
    #[rasn(value("0..=24"))]
    pub hours: u8,
    #[rasn(value("0..=59"))]
    pub minutes: u8,
    #[rasn(value("0..=59"))]
    pub seconds: u8,
    #[rasn(value("0..=99999"))]
    pub microseconds: u32,
}
impl ProgramDuration {
    pub fn new(hours: u8, minutes: u8, seconds: u8, microseconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            microseconds,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=1023"))]
pub struct ProgramNumber(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum RdcPdu {
    Request(RequestPdu),
    Response(ResponsePdu),
    Indication(IndicationPdu),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct RecordForDuration {
    #[rasn(value("0..=24"))]
    pub hours: u8,
    #[rasn(value("0..=59"))]
    pub minutes: u8,
    #[rasn(value("0..=59"))]
    pub seconds: u8,
}

impl RecordForDuration {
    pub fn new(hours: u8, minutes: u8, seconds: u8) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum RequestPdu {
    SourceSelectRequest(SourceSelectRequest),
    SourceEventsRequest(SourceEventsRequest),
    DeviceAttributeRequest(DeviceAttributeRequest),
    DeviceLockRequest(DeviceLockRequest),
    DeviceLockEnquireRequest(DeviceLockEnquireRequest),
    DeviceControlRequest(DeviceControlRequest),
    DeviceStatusEnquireRequest(DeviceStatusEnquireRequest),
    ConfigureDeviceEventsRequest(ConfigureDeviceEventsRequest),
    NonStandardRequest(NonStandardPdu),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ResponsePdu {
    SourceSelectResponse(SourceSelectResponse),
    SourceEventsResponse(SourceEventsResponse),
    DeviceAttributeResponse(DeviceAttributeResponse),
    DeviceLockResponse(DeviceLockResponse),
    DeviceLockEnquireResponse(DeviceLockEnquireResponse),
    DeviceStatusEnquireResponse(DeviceStatusEnquireResponse),
    ConfigureDeviceEventsResponse(ConfigureDeviceEventsResponse),
    NonStandardResponse(NonStandardPdu),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum SelectDirection {
    Next,
    Previous,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum SelectExternalLight {
    #[rasn(value("1..=10"))]
    LightNumber(u8),
    None,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct SetFocusPosition {
    #[rasn(identifier = "focusPosition")]
    pub focus_position: FocusPosition,
    #[rasn(identifier = "positioningMode")]
    pub positioning_mode: PositioningMode,
}
impl SetFocusPosition {
    pub fn new(focus_position: FocusPosition, positioning_mode: PositioningMode) -> Self {
        Self {
            focus_position,
            positioning_mode,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct SetIrisPosition {
    #[rasn(identifier = "irisPosition")]
    pub iris_position: IrisPosition,
    #[rasn(identifier = "positioningMode")]
    pub positioning_mode: PositioningMode,
}
impl SetIrisPosition {
    pub fn new(iris_position: IrisPosition, positioning_mode: PositioningMode) -> Self {
        Self {
            iris_position,
            positioning_mode,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct SetPanPosition {
    #[rasn(identifier = "panPosition")]
    pub pan_position: PanPosition,
    #[rasn(identifier = "positioningMode")]
    pub positioning_mode: PositioningMode,
}
impl SetPanPosition {
    pub fn new(pan_position: PanPosition, positioning_mode: PositioningMode) -> Self {
        Self {
            pan_position,
            positioning_mode,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct SetTiltPosition {
    #[rasn(identifier = "tiltPosition")]
    pub tilt_position: TiltPosition,
    #[rasn(identifier = "positioningMode")]
    pub positioning_mode: PositioningMode,
}
impl SetTiltPosition {
    pub fn new(tilt_position: TiltPosition, positioning_mode: PositioningMode) -> Self {
        Self {
            tilt_position,
            positioning_mode,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct SetZoomPosition {
    #[rasn(identifier = "zoomPosition")]
    pub zoom_position: ZoomPosition,
    #[rasn(identifier = "positioningMode")]
    pub positioning_mode: PositioningMode,
}
impl SetZoomPosition {
    pub fn new(zoom_position: ZoomPosition, positioning_mode: PositioningMode) -> Self {
        Self {
            zoom_position,
            positioning_mode,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=1023"))]
pub struct SlideNumber(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SourceChangeEventIndication {
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
}
impl SourceChangeEventIndication {
    pub fn new(device_class: DeviceClass, device_id: DeviceID) -> Self {
        Self {
            device_class,
            device_id,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SourceEventsRequest {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "streamIdentifier")]
    pub stream_identifier: StreamID,
    #[rasn(identifier = "sourceEventNotify")]
    pub source_event_notify: bool,
}
impl SourceEventsRequest {
    pub fn new(
        request_handle: Handle,
        stream_identifier: StreamID,
        source_event_notify: bool,
    ) -> Self {
        Self {
            request_handle,
            stream_identifier,
            source_event_notify,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SourceEventsResponseResult {
    Successful,
    EventsNotSupported,
    InvalidStreamID,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SourceEventsResponse {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
    pub result: SourceEventsResponseResult,
}
impl SourceEventsResponse {
    pub fn new(
        request_handle: Handle,
        device_class: DeviceClass,
        device_id: DeviceID,
        result: SourceEventsResponseResult,
    ) -> Self {
        Self {
            request_handle,
            device_class,
            device_id,
            result,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SourceSelectRequest {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceID")]
    pub device_id: DeviceID,
    #[rasn(identifier = "streamIdentifier")]
    pub stream_identifier: StreamID,
}
impl SourceSelectRequest {
    pub fn new(
        request_handle: Handle,
        device_class: DeviceClass,
        device_id: DeviceID,
        stream_identifier: StreamID,
    ) -> Self {
        Self {
            request_handle,
            device_class,
            device_id,
            stream_identifier,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SourceSelectResponseResult {
    Successful,
    RequestDenied,
    DeviceUnavailable,
    InvalidStreamID,
    CurrentDeviceIsLocked,
    DeviceIncompatible,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SourceSelectResponse {
    #[rasn(identifier = "requestHandle")]
    pub request_handle: Handle,
    pub result: SourceSelectResponseResult,
}
impl SourceSelectResponse {
    pub fn new(request_handle: Handle, result: SourceSelectResponseResult) -> Self {
        Self {
            request_handle,
            result,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum StatusAttribute {
    CurrentdeviceState(CurrentDeviceState),
    CurrentDeviceDate(CurrentDeviceDate),
    CurrentDeviceTime(CurrentDeviceTime),
    CurrentDevicePreset(CurrentDevicePreset),
    CurrentIrisMode(CurrentMode),
    CurrentFocusMode(CurrentMode),
    CurrentBackLightMode(CurrentMode),
    CurrentPointingMode(CurrentPointingMode),
    CurrentCameraLens(CurrentCameraLensNumber),
    CurrentCameraFilter(CurrentCameraFilterNumber),
    CurrentExternalLight(CurrentExternalLight),
    CurrentCameraPanSpeed(CurrentCameraPanSpeed),
    CurrentCameraTiltSpeed(CurrentCameraTiltSpeed),
    CurrentBackLight(CurrentBackLight),
    CurrentWhiteBalance(CurrentWhiteBalance),
    CurrentWhiteBalanceMode(CurrentMode),
    CurrentZoomPosition(CurrentZoomPosition),
    CurrentFocusPosition(CurrentFocusPosition),
    CurrentIrisPosition(CurrentIrisPosition),
    CurrentPanPosition(CurrentPanPosition),
    CurrentTiltPosition(CurrentTiltPosition),
    CurrentSlide(CurrentSlide),
    CurrentAutoSlideDisplayTime(CurrentAutoSlideDisplayTime),
    CurrentSelectedProgram(CurrentSelectedProgram),
    CurrentstreamPlayerState(CurrentStreamPlayerState),
    CurrentProgramDuration(ProgramDuration),
    CurrentPlaybackSpeed(CurrentPlaybackSpeed),
    CurrentAudioOutputMute(CurrentAudioOutputMute),
    ConfigurableVideoInputs(DeviceInputs),
    VideoInputs(DeviceInputs),
    ConfigurableAudioInputs(DeviceInputs),
    AudioInputs(DeviceInputs),
    NonStandardStatus(NonStandardParameter),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum StatusAttributeIdentifier {
    GetDeviceState,
    GetDeviceDate,
    GetDeviceTime,
    GetdevicePreset,
    GetIrisMode,
    GetFocusMode,
    GetBacklightMode,
    GetPointingMode,
    GetCameraLens,
    GetCameraFilter,
    GetExternalLight,
    GetCameraPanSpeed,
    GetCameraTiltSpeed,
    GetBackLightMode,
    GetBackLight,
    GetWhiteBalance,
    GetWhiteBalanceMode,
    GetZoomPosition,
    GetFocusPosition,
    GetIrisPosition,
    GetPanPosition,
    GetTiltPosition,
    GetSelectedSlide,
    GetAutoSlideDisplayTime,
    GetSelectedProgram,
    GetStreamPlayerState,
    GetCurrentProgramDuration,
    GetPlaybackSpeed,
    GetAudioOutputState,
    GetConfigurableVideoInputs,
    GetVideoInputs,
    GetConfigurableAudioInputs,
    GetAudioInputs,
    GetNonStandardStatus(NonStandardIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct StreamID(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum StreamPlayerState {
    Playing,
    Recording,
    PausedOnRecord,
    PausedOnPlay,
    Rewinding,
    FastForwarding,
    SearchingForwards,
    SearchingBackwards,
    Stopped,
    ProgramUnavailable,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct StreamProfile {
    #[rasn(identifier = "streamID")]
    pub stream_id: StreamID,
    #[rasn(identifier = "videoStreamFlag")]
    pub video_stream_flag: bool,
    #[rasn(identifier = "sourceChangeFlag")]
    pub source_change_flag: bool,
    #[rasn(identifier = "streamName")]
    pub stream_name: Option<TextString>,
}
impl StreamProfile {
    pub fn new(
        stream_id: StreamID,
        video_stream_flag: bool,
        source_change_flag: bool,
        stream_name: Option<TextString>,
    ) -> Self {
        Self {
            stream_id,
            video_stream_flag,
            source_change_flag,
            stream_name,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=255"))]
pub struct TextString(pub BmpString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum TiltContinuousTiltDirection {
    Up,
    Down,
    Stop,
    Continue,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct TiltContinuous {
    #[rasn(identifier = "tiltDirection")]
    pub tilt_direction: TiltContinuousTiltDirection,
    #[rasn(value("50..=1000"), identifier = "timeOut")]
    pub time_out: u16,
}
impl TiltContinuous {
    pub fn new(tilt_direction: TiltContinuousTiltDirection, time_out: u16) -> Self {
        Self {
            tilt_direction,
            time_out,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-18000..=18000"))]
pub struct TiltPosition(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct TiltPositionCapability {
    #[rasn(value("-18000..=0"), identifier = "maxDown")]
    pub max_down: i16,
    #[rasn(value("0..=18000"), identifier = "maxUp")]
    pub max_up: u16,
    #[rasn(value("1..=18000"), identifier = "minStepSize")]
    pub min_step_size: u16,
}
impl TiltPositionCapability {
    pub fn new(max_down: i16, max_up: u16, min_step_size: u16) -> Self {
        Self {
            max_down,
            max_up,
            min_step_size,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-1000..=1000"))]
pub struct TiltView(pub i16);
#[doc = " Anonymous SET OF member "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "SEQUENCE")]
pub struct AnonymousVideoInputsCapabilityAvailableDevices {
    #[rasn(identifier = "deviceClass")]
    pub device_class: DeviceClass,
    #[rasn(identifier = "deviceIdentifier")]
    pub device_identifier: DeviceID,
}
impl AnonymousVideoInputsCapabilityAvailableDevices {
    pub fn new(device_class: DeviceClass, device_identifier: DeviceID) -> Self {
        Self {
            device_class,
            device_identifier,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2..=64"))]
pub struct VideoInputsCapabilityAvailableDevices(
    pub SetOf<AnonymousVideoInputsCapabilityAvailableDevices>,
);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct VideoInputsCapability {
    #[rasn(value("2..=64"), identifier = "numberOfDeviceInputs")]
    pub number_of_device_inputs: u8,
    #[rasn(value("1..=64"), identifier = "numberOfDeviceRows")]
    pub number_of_device_rows: u8,
    #[rasn(size("2..=64"), identifier = "availableDevices")]
    pub available_devices: Option<SetOf<VideoInputsCapabilityAvailableDevices>>,
}
impl VideoInputsCapability {
    pub fn new(
        number_of_device_inputs: u8,
        number_of_device_rows: u8,
        available_devices: Option<SetOf<VideoInputsCapabilityAvailableDevices>>,
    ) -> Self {
        Self {
            number_of_device_inputs,
            number_of_device_rows,
            available_devices,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct WhiteBalance(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1980..=2999"))]
pub struct Year(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum ZoomContinuousZoomDirection {
    Telescopic,
    Wide,
    Stop,
    Continue,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct ZoomContinuous {
    #[rasn(identifier = "zoomDirection")]
    pub zoom_direction: ZoomContinuousZoomDirection,
    #[rasn(value("50..=1000"), identifier = "timeOut")]
    pub time_out: u16,
}
impl ZoomContinuous {
    pub fn new(zoom_direction: ZoomContinuousZoomDirection, time_out: u16) -> Self {
        Self {
            zoom_direction,
            time_out,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("10..=1000"))]
pub struct ZoomMagnification(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-1023..=1023"))]
pub struct ZoomPosition(pub i16);
