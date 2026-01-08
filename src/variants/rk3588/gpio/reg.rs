use tock_registers::{register_structs, registers::*};

register_structs! {
    pub Registers{
        (0x00 => pub swport_dr: ReadWrite<u32>),
        (0x04 => pub swport_ddr: ReadWrite<u32>),
        (0x08 => pub swport_ctl: ReadWrite<u32>),
        (0x0c => _rsv1),
        (0x10 => pub inten: ReadWrite<u32>),
        (0x14 => pub intmask: ReadWrite<u32>),
        (0x18 => pub inttype_level: ReadWrite<u32>),
        (0x1c => pub int_polarity: ReadWrite<u32>),
        (0x20 => pub int_status: ReadOnly<u32>),
        (0x24 => pub raw_int_status: ReadOnly<u32>),
        (0x28 => pub debounce: ReadWrite<u32>),
        (0x2c => pub porta_eoi: ReadWrite<u32>),
        (0x30 => pub ext_port: ReadOnly<u32>),
        (0x34 => pub lvs_level: ReadOnly<u32>),
        (0x38 => _rsv2),
        (0x40 => pub dbnce_con: ReadWrite<u32>),
        (0x44 => @END),
    }
}
