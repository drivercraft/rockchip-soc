#![cfg_attr(not(any(windows, unix)), no_std)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;

#[macro_use]
mod _macros;

#[macro_use]
mod grf;

mod clock;
mod pinctrl;
mod rst;
mod syscon;
mod variants;

use core::ptr::NonNull;

pub use variants::*;

pub use pinctrl::id::*;
pub use pinctrl::{DriveStrength, Function, PinctrlResult, Pull, GpioDirection};
pub type Mmio = NonNull<u8>;
pub use rst::{ResetRockchip, RstId};

// 导出 RK3588 特定类型
pub use variants::rk3588::PinManager;
