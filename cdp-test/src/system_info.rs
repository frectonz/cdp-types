/// Describes a single graphics processor (GPU).
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-GPUDevice>
pub struct SystemInfoGpuDevice {
    pub vendor_id: (),
    pub device_id: (),
    pub sub_sys_id: (),
    pub revision: (),
    pub vendor_string: (),
    pub device_string: (),
    pub driver_vendor: (),
    pub driver_version: (),
}
/// Describes the width and height dimensions of an entity.
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-Size>
pub struct SystemInfoSize {
    pub width: (),
    pub height: (),
}
/** Describes a supported video decoding profile with its associated minimum and
maximum resolutions.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-VideoDecodeAcceleratorCapability>
pub struct SystemInfoVideoDecodeAcceleratorCapability {
    pub profile: (),
    pub max_resolution: (),
    pub min_resolution: (),
}
/** Describes a supported video encoding profile with its associated maximum
resolution and maximum framerate.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-VideoEncodeAcceleratorCapability>
pub struct SystemInfoVideoEncodeAcceleratorCapability {
    pub profile: (),
    pub max_resolution: (),
    pub max_framerate_numerator: (),
    pub max_framerate_denominator: (),
}
/// YUV subsampling type of the pixels of a given image.
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-SubsamplingFormat>
pub enum SystemInfoSubsamplingFormat {
    Yuv420,
    Yuv422,
    Yuv444,
}
/// Image format of a given image.
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-ImageType>
pub enum SystemInfoImageType {
    Jpeg,
    Webp,
    Unknown,
}
/** Describes a supported image decoding profile with its associated minimum and
maximum resolutions and subsampling.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-ImageDecodeAcceleratorCapability>
pub struct SystemInfoImageDecodeAcceleratorCapability {
    pub image_type: (),
    pub max_dimensions: (),
    pub min_dimensions: (),
    pub subsamplings: (),
}
/// Provides information about the GPU(s) on the system.
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-GPUInfo>
pub struct SystemInfoGpuInfo {
    pub devices: (),
    pub aux_attributes: (),
    pub feature_status: (),
    pub driver_bug_workarounds: (),
    pub video_decoding: (),
    pub video_encoding: (),
    pub image_decoding: (),
}
/// Represents process info.
/// <https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-ProcessInfo>
pub struct SystemInfoProcessInfo {
    pub _type: (),
    pub id: (),
    pub cpu_time: (),
}
