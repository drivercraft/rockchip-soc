use core::ptr::NonNull;

use alloc::vec::Vec;
use fdt_edit::Fdt;

use crate::{PinId, Pull};

pub struct PinctrlConfig {
    pub pin: PinId,
    pub mux: u32,
    // pub conf_phandle: u32,
}

impl PinctrlConfig {
    /// `rockchip,pins` property
    pub fn new(cells: &[u32], fdt_addr: NonNull<u8>) -> Self {
        let bank = cells[0];
        let pin = cells[1];
        let mux = cells[2];
        let conf_phandle = cells[3];
        let id = PinId::from_bank_pin(bank.into(), pin).unwrap();

        let fdt = unsafe { Fdt::from_ptr(fdt_addr.as_ptr()).unwrap() };

        let conf_node = fdt.find_by_phandle(conf_phandle.into()).unwrap();

        let mut pull = Pull::Disabled;

        for prop in conf_node.properties() {
            match prop.name() {
                "bias-disable" => {
                    pull = Pull::Disabled;
                }
                "bias-pull-up" => {
                    pull = Pull::PullUp;
                }
                "bias-pull-down" => {
                    pull = Pull::PullDown;
                }
                _ => {}
            }
        }

        Self {
            pin: id,
            mux,
            // conf_phandle,
        }
    }
}
