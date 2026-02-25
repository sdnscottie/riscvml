//! FFI Boundary — Safe Rust Wrappers Around ESP-IDF C Calls
//!
//! This module contains all `unsafe` blocks that bridge Rust to the
//! ESP-IDF C libraries. Each unsafe call is wrapped in a safe Rust
//! function that upholds the invariants required by the C API.
//!
//! The FFI boundary is the dividing line between:
//!   - Above: 100% safe Rust (application logic, HAL, Embassy async)
//!   - Below: ESP-IDF C drivers (esp-video, esp-dl, ISP, PPA, H.264)
//!
//! Unsafe is *contained*, not eliminated — the C libraries are mature
//! and battle-tested, but Rust wrappers prevent misuse at the API level.

use esp_idf_sys as _;

// ---- Camera Controller Driver FFI ----

extern "C" {
    fn esp_camera_controller_init(config: *const CameraConfig) -> i32;
    fn esp_camera_controller_start() -> i32;
    fn esp_camera_controller_get_frame(frame: *mut FrameInfo) -> i32;
}

#[repr(C)]
pub struct CameraConfig {
    pub width: u32,
    pub height: u32,
    pub lanes: u8,
    pub clock_freq: u32,
}

#[repr(C)]
pub struct FrameInfo {
    pub data: *mut u8,
    pub len: usize,
    pub timestamp: u64,
}

/// Safe wrapper: initialize the camera controller.
pub fn camera_init(width: u32, height: u32) -> Result<(), i32> {
    let config = CameraConfig {
        width,
        height,
        lanes: 2,
        clock_freq: 80_000_000,
    };
    let ret = unsafe { esp_camera_controller_init(&config) };
    if ret == 0 { Ok(()) } else { Err(ret) }
}

// ---- ISP FFI ----

extern "C" {
    fn esp_isp_new_processor(config: *const IspConfig, handle: *mut *mut core::ffi::c_void) -> i32;
    fn esp_isp_enable(handle: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
pub struct IspConfig {
    pub awb_enable: bool,
    pub ae_enable: bool,
    pub ae_target: u8,
}

/// Safe wrapper: create and enable an ISP processor.
pub fn isp_init(awb: bool, ae: bool) -> Result<*mut core::ffi::c_void, i32> {
    let config = IspConfig {
        awb_enable: awb,
        ae_enable: ae,
        ae_target: 128,
    };
    let mut handle: *mut core::ffi::c_void = core::ptr::null_mut();
    let ret = unsafe { esp_isp_new_processor(&config, &mut handle) };
    if ret == 0 {
        let ret2 = unsafe { esp_isp_enable(handle) };
        if ret2 == 0 { Ok(handle) } else { Err(ret2) }
    } else {
        Err(ret)
    }
}

// ---- esp-dl Inference FFI ----

extern "C" {
    fn esp_dl_model_load(path: *const u8, path_len: usize) -> *mut core::ffi::c_void;
    fn esp_dl_model_run(
        model: *mut core::ffi::c_void,
        input: *const u8,
        input_len: usize,
        output: *mut u8,
        output_len: usize,
    ) -> i32;
}

/// Safe wrapper: load a quantized model from flash.
pub fn dl_model_load(path: &str) -> Result<*mut core::ffi::c_void, ()> {
    let handle = unsafe { esp_dl_model_load(path.as_ptr(), path.len()) };
    if handle.is_null() { Err(()) } else { Ok(handle) }
}

/// Safe wrapper: run inference on input tensor.
pub fn dl_model_run(
    model: *mut core::ffi::c_void,
    input: &[u8],
    output: &mut [u8],
) -> Result<(), i32> {
    let ret = unsafe {
        esp_dl_model_run(model, input.as_ptr(), input.len(), output.as_mut_ptr(), output.len())
    };
    if ret == 0 { Ok(()) } else { Err(ret) }
}

// ---- PPA FFI ----

extern "C" {
    fn ppa_client_register(handle: *mut *mut core::ffi::c_void) -> i32;
    fn ppa_do_scale_rotate(
        client: *mut core::ffi::c_void,
        src: *const u8, src_w: u32, src_h: u32,
        dst: *mut u8, dst_w: u32, dst_h: u32,
    ) -> i32;
    fn ppa_do_blend(
        client: *mut core::ffi::c_void,
        fg: *const u8, bg: *mut u8,
        w: u32, h: u32, alpha: u8,
    ) -> i32;
}

/// Safe wrapper: register a PPA client.
pub fn ppa_init() -> Result<*mut core::ffi::c_void, i32> {
    let mut handle: *mut core::ffi::c_void = core::ptr::null_mut();
    let ret = unsafe { ppa_client_register(&mut handle) };
    if ret == 0 { Ok(handle) } else { Err(ret) }
}

// ---- H.264 Encoder FFI ----

extern "C" {
    fn esp_h264_enc_open(config: *const H264Config, handle: *mut *mut core::ffi::c_void) -> i32;
    fn esp_h264_enc_process(
        handle: *mut core::ffi::c_void,
        input: *const u8,
        output: *mut u8,
        output_len: *mut usize,
    ) -> i32;
}

#[repr(C)]
pub struct H264Config {
    pub width: u32,
    pub height: u32,
    pub fps: u8,
    pub bitrate: u32,
    pub gop: u8,
}

/// Safe wrapper: initialize the H.264 hardware encoder.
pub fn h264_init(width: u32, height: u32, fps: u8) -> Result<*mut core::ffi::c_void, i32> {
    let config = H264Config {
        width,
        height,
        fps,
        bitrate: 4000,
        gop: 30,
    };
    let mut handle: *mut core::ffi::c_void = core::ptr::null_mut();
    let ret = unsafe { esp_h264_enc_open(&config, &mut handle) };
    if ret == 0 { Ok(handle) } else { Err(ret) }
}
