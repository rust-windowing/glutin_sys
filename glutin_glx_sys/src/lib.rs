#![cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]

/// GLX bindings
pub mod glx {
    include!(concat!(env!("OUT_DIR"), "/glx_bindings.rs"));
}

/// Functions that are not necessarily always available
pub mod glx_extra {
    include!(concat!(env!("OUT_DIR"), "/glx_extra_bindings.rs"));
}
