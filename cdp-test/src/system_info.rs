use crate::common::*;
/// Describes a single graphics processor (GPU).
pub struct GpuDevice {
    pub vendor_id: u64,
    pub device_id: u64,
    pub sub_sys_id: u64,
    pub revision: u64,
    pub vendor_string: String,
    pub device_string: String,
    pub driver_vendor: String,
    pub driver_version: String,
}
/// Describes the width and height dimensions of an entity.
pub struct Size {
    pub width: i64,
    pub height: i64,
}
/** Describes a supported video decoding profile with its associated minimum and
maximum resolutions.*/
pub struct VideoDecodeAcceleratorCapability {
    pub profile: String,
    pub max_resolution: Box<Size>,
    pub min_resolution: Box<Size>,
}
/** Describes a supported video encoding profile with its associated maximum
resolution and maximum framerate.*/
pub struct VideoEncodeAcceleratorCapability {
    pub profile: String,
    pub max_resolution: Box<Size>,
    pub max_framerate_numerator: i64,
    pub max_framerate_denominator: i64,
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
    pub subsamplings: Vec<SubsamplingFormat>,
}
/// Provides information about the GPU(s) on the system.
pub struct GpuInfo {
    pub devices: Vec<GpuDevice>,
    pub aux_attributes: serde_json::Map<String, serde_json::Value>,
    pub feature_status: serde_json::Map<String, serde_json::Value>,
    pub driver_bug_workarounds: Vec<String>,
    pub video_decoding: Vec<VideoDecodeAcceleratorCapability>,
    pub video_encoding: Vec<VideoEncodeAcceleratorCapability>,
    pub image_decoding: Vec<ImageDecodeAcceleratorCapability>,
}
/// Represents process info.
pub struct ProcessInfo {
    pub _type: String,
    pub id: i64,
    pub cpu_time: u64,
}
