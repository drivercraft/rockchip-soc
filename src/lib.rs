#![no_std]

extern crate alloc;

#[macro_use]
mod grf;

mod variants;
mod syscon;

use core::ptr::NonNull;

pub use variants::*;

pub type Mmio = NonNull<u8>;
