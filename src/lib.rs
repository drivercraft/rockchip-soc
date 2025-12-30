#![no_std]

extern crate alloc;

#[macro_use]
extern crate log;

#[macro_use]
mod grf;

mod clock;
mod syscon;
mod variants;

use core::ptr::NonNull;

pub use variants::*;

pub type Mmio = NonNull<u8>;
