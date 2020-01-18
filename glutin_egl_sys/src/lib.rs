#![cfg(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "android",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#![allow(non_camel_case_types)]

pub mod egl {
    use std::os::raw;

    pub type NativeDisplayType = self::EGLNativeDisplayType;
    pub type NativePixmapType = self::EGLNativePixmapType;
    pub type NativeWindowType = self::EGLNativeWindowType;

    pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
    pub type khronos_uint64_t = u64;
    pub type khronos_ssize_t = raw::c_long;
    pub type EGLint = i32;
    pub type EGLNativeDisplayType = *const raw::c_void;
    pub type EGLNativePixmapType = *const raw::c_void; // FIXME: egl_native_pixmap_t instead

    #[cfg(target_os = "windows")]
    pub type EGLNativeWindowType = winapi::shared::windef::HWND;
    #[cfg(target_os = "linux")]
    pub type EGLNativeWindowType = *const raw::c_void;
    #[cfg(target_os = "android")]
    pub type EGLNativeWindowType = *const raw::c_void;
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub type EGLNativeWindowType = *const raw::c_void;

    include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));
}
