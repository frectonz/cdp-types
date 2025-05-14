use crate::common::*;
/// Describes a single graphics processor (GPU).
pub struct GpuDevice {
    pub vendor_id: Box<u64>,
    pub device_id: Box<u64>,
    pub sub_sys_id: Box<u64>,
    pub revision: Box<u64>,
    pub vendor_string: Box<String>,
    pub device_string: Box<String>,
    pub driver_vendor: Box<String>,
    pub driver_version: Box<String>,
}
/// Describes the width and height dimensions of an entity.
pub struct Size {
    pub width: Box<i64>,
    pub height: Box<i64>,
}
/** Describes a supported video decoding profile with its associated minimum and
maximum resolutions.*/
pub struct VideoDecodeAcceleratorCapability {
    pub profile: Box<String>,
    pub max_resolution: Box<Size>,
    pub min_resolution: Box<Size>,
}
/** Describes a supported video encoding profile with its associated maximum
resolution and maximum framerate.*/
pub struct VideoEncodeAcceleratorCapability {
    pub profile: Box<String>,
    pub max_resolution: Box<Size>,
    pub max_framerate_numerator: Box<i64>,
    pub max_framerate_denominator: Box<i64>,
}
/// YUV subsampling type of the pixels of a given image.
pub enum SubsamplingFormat {
    Yuv420,
    Yuv422,
    Yuv444,
}
/// Image format of a given image.
pub enum ImageType {
    Jpeg,
    Webp,
    Unknown,
}
/** Describes a supported image decoding profile with its associated minimum and
maximum resolutions and subsampling.*/
pub struct ImageDecodeAcceleratorCapability {
    pub image_type: Box<ImageType>,
    pub max_dimensions: Box<Size>,
    pub min_dimensions: Box<Size>,
    pub subsamplings: (),
}
/// Provides information about the GPU(s) on the system.
pub struct GpuInfo {
    pub devices: (),
    pub aux_attributes: Box<serde_json::Map<String, serde_json::Value>>,
    pub feature_status: Box<serde_json::Map<String, serde_json::Value>>,
    pub driver_bug_workarounds: (),
    pub video_decoding: (),
    pub video_encoding: (),
    pub image_decoding: (),
}
/// Represents process info.
pub struct ProcessInfo {
    pub _type: Box<String>,
    pub id: Box<i64>,
    pub cpu_time: Box<u64>,
}
