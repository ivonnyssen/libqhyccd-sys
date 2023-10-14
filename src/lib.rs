use std::sync::Arc;

use eyre::eyre;
use eyre::Result;
use thiserror::Error;

mod bindings;

#[derive(Error, Debug)]
pub enum QHYError {
    #[error("Error initializing QHYCCD SDK, error code {}", error_code)]
    InitSDKError { error_code: u32 },
    #[error("Error getting QHYCCD SDK version, error code {:?}", error_code)]
    GetSDKVersionError { error_code: u32 },
    #[error("Error scanning QHYCCD cameras")]
    ScanQHYCCDError,
    #[error("Error camera id, error code {:?}", error_code)]
    GetCameraIdError { error_code: u32 },
    #[error("Error getting firmware version, error code {:?}", error_code)]
    GetFirmwareVersionError { error_code: u32 },
    #[error("Error setting camera read mode, error code {:?}", error_code)]
    SetCameraReadModeError { error_code: u32 },
    #[error("Error setting camera stream mode, error code {:?}", error_code)]
    SetCameraStreamModeError { error_code: u32 },
    #[error("Error initializing camera {:?}", error_code)]
    InitCameraError { error_code: u32 },
    #[error("Error getting camera CCD info, error code {:?}", error_code)]
    GetCameraCCDInfoError { error_code: u32 },
    #[error("Error setting camera bit mode, error code {:?}", error_code)]
    SetCameraBitModeError { error_code: u32 },
    #[error("Error setting camera debayer on/off, error code {:?}", error_code)]
    SetCameraDebayerOnOffError { error_code: u32 },
    #[error("Error setting camera bin mode, error code {:?}", error_code)]
    SetCameraBinModeError { error_code: u32 },
    #[error("Error setting camera sub frame, error code {:?}", error_code)]
    SetCameraSubFrameError { error_code: u32 },
    #[error("Error setting camera parameter, error code {:?}", error_code)]
    SetCameraParameterError { error_code: u32 },
    #[error("Error starting camera live mode, error code {:?}", error_code)]
    BeginCameraLiveError { error_code: u32 },
    #[error("Error stopping camera live mode, error code {:?}", error_code)]
    EndCameraLiveError { error_code: u32 },
    #[error("Error getting image size, error code")]
    GetImageSizeError,
    #[error("Error getting camera live frame, error code {:?}", error_code)]
    GetCameraLiveFrameError { error_code: u32 },
    #[error("Error closing camera, error code {:?}", error_code)]
    CloseCameraError { error_code: u32 },
}
#[derive(Debug, PartialEq, Clone)]
pub struct QhyccdCamera {
    pub id: Arc<[u8; 32]>,
    pub handle: bindings::QhyccdHandle,
}

#[derive(Debug, PartialEq)]
pub enum CameraFeature {
    ControlBrightness = 0,                  // image brightness
    ControlContrast = 1,                    // image contrast
    ControlWbr = 2,                         // the red of white balance
    ControlWbb = 3,                         // the blue of white balance
    ControlWbg = 4,                         // the green of white balance
    ControlGamma = 5,                       // screen gamma
    ControlGain = 6,                        // camera gain
    ControlOffset = 7,                      // camera offset
    ControlExposure = 8,                    // expose time (us)
    ControlSpeed = 9,                       // transfer speed
    ControlTransferbit = 10,                // image depth bits
    ControlChannels = 11,                   // image channels
    ControlUsbTraffic = 12,                 // hblank
    ControlRowDeNoise = 13,                 // row denoise
    ControlCurTemp = 14,                    // current cmos or ccd temprature
    ControlCurPWM = 15,                     // current cool pwm
    ControlManulPWM = 16,                   // set the cool pwm
    ControlCfwPort = 17,                    // control camera color filter wheel port
    ControlCooler = 18,                     // check if camera has cooler
    ControlSt4Port = 19,                    // check if camera has st4port
    CamColor = 20,                          // FIXME!  CAM_IS_COLOR CAM_COLOR conflict
    CamBin1x1mode = 21,                     // check if camera has bin1x1 mode
    CamBin2x2mode = 22,                     // check if camera has bin2x2 mode
    CamBin3x3mode = 23,                     // check if camera has bin3x3 mode
    CamBin4x4mode = 24,                     // check if camera has bin4x4 mode
    CamMechanicalShutter = 25,              // mechanical shutter
    CamTrigerInterface = 26,                // check if camera has triger interface
    CamTecoverprotectInterface = 27,        // tec overprotect
    CamSignalClampInterface = 28,           // signal clamp
    CamFinetoneInterface = 29,              // fine tone
    CamShutterMotorHeatingInterface = 30,   // shutter motor heating
    CamCalibrateFpnInterface = 31,          // calibrated frame
    CamChipTemperatureSensorInterface = 32, // chip temperaure sensor
    CamUsbReadoutSlowestInterface = 33,     // usb readout slowest
    Cam8bits = 34,                          // 8bit depth
    Cam16bits = 35,                         // 16bit depth
    CamGps = 36,                            // check if camera has gps
    CamIgnoreOverscanInterface = 37,        // ignore overscan area
    Qhyccd3aAutoexposure = 39,              // auto exposure
    Qhyccd3aAutofocus = 40,                 // auto focus
    ControlAmpv = 41,                       // ccd or cmos ampv
    ControlVcam = 42,                       // Virtual Camera on off
    CamViewMode = 43,                       //
    ControlCfwSlotsNum = 44,                // check CFW slots number
    IsExposingDone = 45,                    //
    ScreenStretchB = 46,                    //
    ScreenStretchW = 47,                    //
    ControlDDR = 48,                        //
    CamLightPerformanceMode = 49,           //
    CamQhy5iiGuideMode = 50,                //
    DDRBufferCapacity = 51,                 //
    DDRBufferReadThreshold = 52,            //
    DefaultGain = 53,                       //
    DefaultOffset = 54,                     //
    OutputDataActualBits = 55,              //
    OutputDataAlignment = 56,               //
    CamSingleFrameMode = 57,                //
    CamLiveVideoMode = 58,                  //
    CamIsColor = 59,                        //
    HasHardwareFrameCounter = 60,           //
    ControlMaxIdError = 61,                 // Not Used , previous max index
    CamHumidity = 62, // check if camera has  humidity sensor  20191021 LYL Unified humidity function
    CamPressure = 63, // check if camera has pressure sensor
    ControlVacuumPump = 64, // if camera has VACUUM PUMP
    ControlSensorChamberCyclePump = 65, //air cycle pump for sensor drying
    Cam32bits = 66,   // 32bit depth
    CamSensorUlvoStatus = 67, // Sensor working status [0:init  1:good  2:checkErr  3:monitorErr 8:good 9:powerChipErr]  410 461 411 600 268 [Eris board]
    CamSensorPhaseReTrain = 68, // 2020,4040/PRO，6060,42PRO
    CamInitConfigFromFlash = 69, // 2410 461 411 600 268 for now
    CamTriggerMode = 70,      //check if camera has multiple triger mode
    CamTriggerOut = 71,       //check if camera support triger out function
    CamBurstMode = 72,        //check if camera support burst mode
    CamSpeakerLedAlarm = 73,  // for OEM-600
    CamWatchDogFpga = 74, // for _QHY5III178C Celestron, SDK have to feed this dog or it go reset
    CamBin6x6mode = 75,   // check if camera has bin6x6 mode
    CamBin8x8mode = 76,   // check if camera has bin8x8 mode
    CamGlobalSensorGpsLED = 77, //Show GPS LED tab on sharpCap
    ControlImgProc = 78,  // Process image
    ControlRemoveRbi = 79, // Remove single RBI
    ControlGlobalReset = 80, //image stabilization
    ControlFrameDetect = 81, //
    CamGainDbConversion = 82, //Supports the conversion between db and gain
    CamCurveSystemGain = 83, //
    CamCurveFullWell = 84, //
    CamCurveReadoutNoise = 85, //
    ControlMaxId = 86, // Do not Put Item after  CONTROL_MAX_ID !! This should be the max index of the list - so much for that :)
    ControlAutowhitebalance = 1024, //auto white balance  eg.CONTROL_TEST=1024
    ControlAutoexposure = 1025, //auto exposure
    ControlAutoexpMessureValue = 1026, //
    ControlAutoexpMessureMethod = 1027, //
    ControlImageStabilization = 1028, //image stabilization
    ControlGaindB = 1029, //used to test dBGain control  //CONTROL_dB_TO_GAIN
}

#[derive(Debug, PartialEq)]
pub enum CameraStreamMode {
    SingleFrameMode = 0,
    LiveMode = 1,
}

#[derive(Debug, PartialEq)]
pub struct CCDChipInfo {
    pub chip_width: f64,
    pub chip_height: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub pixel_width: f64,
    pub pixel_height: f64,
    pub bits_per_pixel: u32,
}

#[derive(Debug, PartialEq)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u32,
    pub channels: u32,
}

/// The camera is using PCIE to transfer data
pub const QHYCCD_PCIE: u32 = 9;

/// The camera using WINPCAP to transfer data
pub const QHYCCD_WINPCAP: u32 = 8;

/// The camera is using GiGaE to transfer data
pub const QHYCCD_QGIGAE: u32 = 7;

/// The camera using usb sync to transfer data
pub const QHYCCD_USBSYNC: u32 = 6;

/// The camera using usb async to transfer data
pub const QHYCCD_USBASYNC: u32 = 5;

/// It is a color camera
pub const QHYCCD_COLOR: u32 = 4;

/// It is a monochrome camera
pub const QHYCCD_MONO: u32 = 3;

/// Camera has a cooler
pub const QHYCCD_COOL: u32 = 2;

/// Camera does not have a cooler
pub const QHYCCD_NOTCOO: u32 = 1;

/// Function returned successfully
const QHYCCD_SUCCESS: u32 = 0;

// Function returned unsuccessfully
const QHYCCD_ERROR: u32 = 0xFFFFFFFF;

/// initialize the QHYCCD SDK
///
/// # Example
///
/// ```
/// # use libqhyccd_sys::init_sdk;
/// use eyre::Result;
///
/// let result = init_sdk();
/// assert!(result.is_ok());
/// ```
pub fn init_sdk() -> Result<()> {
    match unsafe { bindings::InitQHYCCDResource() } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::InitSDKError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}
/// Scan the connected cameras and return the nubmer found
///
/// # Example
///
/// ```
/// # use libqhyccd_sys::scan_qhyccd;
/// use eyre::Result;
///
/// let result = scan_qhyccd().unwrap();
/// assert_eq!(result, 0);
/// ```
pub fn scan_qhyccd() -> Result<u32> {
    match unsafe { bindings::ScanQHYCCD() } {
        /*
            below error is -2 and commented, out, return is uint, so cannot be true
            QHYCCD_ERROR_NO_DEVICE => {
            tracing::error!("error scanning QHYCCD cameras, no device found");
            Err(eyre!("error scanning QHYCCD cameras, no device found"))
        }*/
        QHYCCD_ERROR => {
            let error = QHYError::ScanQHYCCDError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        num => Ok(num),
    }
}

#[derive(Debug, PartialEq)]
pub struct SDKVersion {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub subday: u32,
}

/// Get the QHYCCD SDK version
/// # Example
/// ```
/// # use libqhyccd_sys::get_sdk_version;
/// # use libqhyccd_sys::SDKVersion;
/// use eyre::Result;
///
/// let version = get_sdk_version().unwrap();
/// assert_eq!(version, SDKVersion{year: 23, month: 9, day: 6, subday: 14});
/// ```
/// # Errors
/// Returns `QHYError::GetSDKVersionError` if the SDK version cannot be retrieved
pub fn get_sdk_version() -> Result<SDKVersion> {
    let mut year: u32 = 0;
    let mut month: u32 = 0;
    let mut day: u32 = 0;
    let mut subday: u32 = 0;
    match unsafe { bindings::GetQHYCCDSDKVersion(&mut year, &mut month, &mut day, &mut subday) } {
        QHYCCD_SUCCESS => Ok(SDKVersion {
            year,
            month,
            day,
            subday,
        }),
        error_code => {
            let error = QHYError::GetSDKVersionError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_camera_id(index: u32) -> Result<Arc<[u8; 32]>> {
    let mut id = [0u8; 32];
    match unsafe { bindings::GetQHYCCDId(index, id.as_mut_ptr()) } {
        QHYCCD_SUCCESS => Ok(Arc::new(id)),
        error_code => {
            let error = QHYError::GetCameraIdError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn open_camera(id: Arc<[u8; 32]>) -> QhyccdCamera {
    let handle = unsafe { bindings::OpenQHYCCD(id.as_ptr()) };
    QhyccdCamera { id, handle }
}

pub fn close_camera(camera: QhyccdCamera) -> Result<()> {
    match unsafe { bindings::CloseQHYCCD(camera.handle) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::CloseCameraError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_firmware_version(camera: QhyccdCamera) -> Result<String> {
    let mut version = [0u8; 32];
    match unsafe { bindings::GetQHYCCDFWVersion(camera.handle, version.as_mut_ptr()) } {
        QHYCCD_SUCCESS => {
            if version[0] >> 4 <= 9 {
                Ok(format!(
                    "Firmware version: 20{}_{}_{}",
                    (((version[0] >> 4) + 0x10) as u32),
                    version[0] & 0x0F,
                    version[1]
                ))
            } else {
                Ok(format!(
                    "Firmware version: 20{}_{}_{}",
                    ((version[0] >> 4) as u32),
                    version[0] & 0x0F,
                    version[1]
                ))
            }
        }
        error_code => {
            let error = QHYError::GetFirmwareVersionError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn is_camera_feature_supported(camera: QhyccdCamera, feature: CameraFeature) -> bool {
    unsafe { bindings::IsQHYCCDControlAvailable(camera.handle, feature as u32) != QHYCCD_ERROR }
}

pub fn set_camera_read_mode(camera: QhyccdCamera, mode: u32) -> Result<()> {
    match unsafe { bindings::SetQHYCCDReadMode(camera.handle, mode) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetCameraReadModeError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_camera_stream_mode(camera: QhyccdCamera, mode: CameraStreamMode) -> Result<()> {
    match unsafe { bindings::SetQHYCCDStreamMode(camera.handle, mode as u8) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetCameraStreamModeError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn init_camera(camera: QhyccdCamera) -> Result<()> {
    match unsafe { bindings::InitQHYCCD(camera.handle) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::InitCameraError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_camera_ccd_info(camera: QhyccdCamera) -> Result<CCDChipInfo> {
    let mut chipw: f64 = 0.0;
    let mut chiph: f64 = 0.0;
    let mut imagew: u32 = 0;
    let mut imageh: u32 = 0;
    let mut pixelw: f64 = 0.0;
    let mut pixelh: f64 = 0.0;
    let mut bpp: u32 = 0;
    match unsafe {
        bindings::GetQHYCCDChipInfo(
            camera.handle,
            &mut chipw as *mut f64,
            &mut chiph as *mut f64,
            &mut imagew as *mut u32,
            &mut imageh as *mut u32,
            &mut pixelw as *mut f64,
            &mut pixelh as *mut f64,
            &mut bpp as *mut u32,
        )
    } {
        QHYCCD_SUCCESS => Ok(CCDChipInfo {
            chip_width: chipw,
            chip_height: chiph,
            image_width: imagew,
            image_height: imageh,
            pixel_width: pixelw,
            pixel_height: pixelh,
            bits_per_pixel: bpp,
        }),
        error_code => {
            let error = QHYError::GetCameraCCDInfoError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_camera_bit_mode(camera: QhyccdCamera, mode: u32) -> Result<()> {
    match unsafe { bindings::SetQHYCCDBitsMode(camera.handle, mode) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetCameraBitModeError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_camera_debayer_on_off(camera: QhyccdCamera, on: bool) -> Result<()> {
    match unsafe { bindings::SetQHYCCDDebayerOnOff(camera.handle, on) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetCameraDebayerOnOffError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_camera_bin_mode(camera: QhyccdCamera, bin_x: u32, bin_y: u32) -> Result<()> {
    match unsafe { bindings::SetQHYCCDBinMode(camera.handle, bin_x, bin_y) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetCameraBinModeError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_camera_roi(
    camera: QhyccdCamera,
    start_x: u32,
    start_y: u32,
    width: u32,
    height: u32,
) -> Result<()> {
    match unsafe { bindings::SetQHYCCDResolution(camera.handle, start_x, start_y, width, height) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetCameraSubFrameError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_camera_parameter(
    camera: QhyccdCamera,
    feature: CameraFeature,
    value: f64,
) -> Result<()> {
    match unsafe { bindings::SetQHYCCDParam(camera.handle, feature as u32, value) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetCameraParameterError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn begin_camera_live(camera: QhyccdCamera) -> Result<()> {
    match unsafe { bindings::BeginQHYCCDLive(camera.handle) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::BeginCameraLiveError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn end_camera_live(camera: QhyccdCamera) -> Result<()> {
    match unsafe { bindings::StopQHYCCDLive(camera.handle) } {
        QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::EndCameraLiveError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_camera_image_size(camera: QhyccdCamera) -> Result<u32> {
    match unsafe { bindings::GetQHYCCDMemLength(camera.handle) } {
        QHYCCD_ERROR => {
            let error = QHYError::GetImageSizeError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        size => Ok(size),
    }
}

pub fn get_camera_live_frame(camera: QhyccdCamera, buffer_size: usize) -> Result<ImageData> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bpp: u32 = 0;
    let mut channels: u32 = 0;
    let mut buffer = vec![0u8; buffer_size];
    match unsafe {
        bindings::GetQHYCCDLiveFrame(
            camera.handle,
            &mut width as *mut u32,
            &mut height as *mut u32,
            &mut bpp as *mut u32,
            &mut channels as *mut u32,
            buffer.as_mut_ptr(),
        )
    } {
        QHYCCD_SUCCESS => Ok(ImageData {
            data: buffer,
            width,
            height,
            bits_per_pixel: bpp,
            channels,
        }),
        error_code => {
            let error = QHYError::GetCameraLiveFrameError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

/*
QHYCCD|QHYCCD.CPP|GetQHYCCDSDKVersion|23 9 6 14
QHYCCD SDK Version: V20230906_14

-- qhyccd.cpp param
QHYCCD|QHYCCD.CPP|InitQHYCCDResource()|START
QHYCCD|QHYCCD.CPP|InitQHYCCDResource|auto_detect_camera:false,call InitQHYCCDResourceInside
QHYCCD|QHYCCD.CPP|InitQHYCCDResourceInside|START
QHYCCD|QHYCCD.CPP|libusb_version 1.0.26.11724
QHYCCD|QHYCCD.CPP|libusb_init(libqhyccd_context) called...
QHYCCD|QHYCCD.CPP|InitQHYCCDResourceInside|numdev set to 0
QHYCCD|QHYCCD.CPP|InitQHYCCDResourceInside|END
************************** config file path  23.9.6.14 svn: 13210  ************************************
QHYCCD|QHYCCD.CPP|InitQHYCCDResource|Load ini filePath = /home/parallels/projects/libqhyccd-sys  fileName = qhyccd.ini
Init SDK success!
Yes!Found QHYCCD,the num is 1
connected to the first camera from the list,id is QHY178M-222b16468c5966524
Open QHYCCD success!
Firmware version:2022_9_5

Init QHYCCD success!
GetQHYCCDChipInfo success!
CCD/CMOS chip information:
Chip width 7334.400000 mm,Chip height 4915.200000 mm
Chip pixel width 2.400000 um,Chip pixel height 2.400000 um
Chip Max Resolution is 3056 x 2048,depth is 8
SetQHYCCDResolution success!
BeginQHYCCDLive success!
^C
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_sdk_version();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            SDKVersion {
                year: 23,
                month: 9,
                day: 6,
                subday: 14
            }
        );
        let result = init_sdk();
        assert!(result.is_ok());
        let result = scan_qhyccd();
        assert_eq!(result.unwrap(), 1);
        let result = get_camera_id(0);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id.as_slice(), b"QHY178M-222b16468c5966524\0\0\0\0\0\0\0");
        let camera = open_camera(id.clone());
        assert_eq!(camera.id, id);
        let result = get_firmware_version(camera.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Firmware version: 2022_9_5");
        let result = is_camera_feature_supported(camera.clone(), CameraFeature::CamLiveVideoMode);
        assert!(result);
        let result = set_camera_read_mode(camera.clone(), 0);
        assert!(result.is_ok());
        let result = set_camera_stream_mode(camera.clone(), CameraStreamMode::LiveMode);
        assert!(result.is_ok());
        let result = init_camera(camera.clone());
        assert!(result.is_ok());
        let result = get_camera_ccd_info(camera.clone());
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(
            info,
            CCDChipInfo {
                chip_width: 7334.4,
                chip_height: 4915.2,
                image_width: 3056,
                image_height: 2048,
                pixel_width: 2.4,
                pixel_height: 2.4,
                bits_per_pixel: 8
            }
        );
        assert!(is_camera_feature_supported(
            camera.clone(),
            CameraFeature::ControlTransferbit
        ));
        let result = set_camera_bit_mode(camera.clone(), 8);
        assert!(result.is_ok());
        let result = set_camera_debayer_on_off(camera.clone(), false);
        assert!(result.is_ok());
        let result = set_camera_bin_mode(camera.clone(), 1, 1);
        assert!(result.is_ok());
        let result = set_camera_roi(camera.clone(), 0, 0, 3056, 2048);
        assert!(result.is_ok());
        let result = set_camera_parameter(camera.clone(), CameraFeature::ControlTransferbit, 8.0);
        assert!(result.is_ok());
        let result = set_camera_parameter(camera.clone(), CameraFeature::ControlExposure, 2000.0);
        assert!(result.is_ok());
        let result = set_camera_parameter(camera.clone(), CameraFeature::ControlUsbTraffic, 255.0);
        assert!(result.is_ok());
        let result = set_camera_parameter(camera.clone(), CameraFeature::ControlDDR, 1.0);
        assert!(result.is_ok());
        let result = begin_camera_live(camera.clone());
        assert!(result.is_ok());
        let result = get_camera_image_size(camera.clone());
        assert!(result.is_ok());
        let size = result.unwrap();
        assert_eq!(size, 27116352);

        loop {
            let result = get_camera_live_frame(camera.clone(), size as usize);
            if result.is_err() {
                continue;
            }
            let image = result.unwrap();
            assert_eq!(image.data.len(), size as usize);
            assert_eq!(image.width, 3056);
            assert_eq!(image.height, 2048);
            assert_eq!(image.bits_per_pixel, 8);
            assert_eq!(image.channels, 1);
            break;
        }
    }
}
