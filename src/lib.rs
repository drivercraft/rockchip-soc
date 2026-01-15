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

pub(crate) mod pinctrl;
mod rst;
mod syscon;
pub(crate) mod variants;

use core::ptr::NonNull;

pub use pinctrl::id::*;
pub use pinctrl::{GpioDirection, PinConfig, PinCtrl, PinCtrlOp, PinctrlResult, Pull};
pub type Mmio = NonNull<u8>;
pub use rst::{ResetRockchip, RstId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocType {
    Rk3588,
}
