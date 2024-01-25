#[cfg_attr(any(target_os = "linux", target_os = "freebsd"), path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
pub mod platform;

pub mod default_out;
pub mod dmifn;
pub mod dmiopt;
pub mod error;

#[doc(no_inline)]
pub use dmiopt::Opt;
#[doc(no_inline)]
pub use smbioslib;
#[doc(no_inline)]
pub use structopt;
