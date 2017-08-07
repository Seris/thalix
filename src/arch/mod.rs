#[cfg(any(target_arch = "x86_64"))]
#[macro_use]
pub mod x86_64;
#[cfg(any(target_arch = "x86_64"))]
pub use self::x86_64::*;
