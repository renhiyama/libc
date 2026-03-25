//! Source directory: `sysdeps/unix/`
//!
//! <https://github.com/bminor/glibc/tree/master/sysdeps/unix>

#[cfg(any(target_os = "linux", target_os = "runixos"))]
pub(crate) mod linux;
