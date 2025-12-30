//! RK3588 GRF 寄存器定义
//! 自动从 orangepi5plus.dts 提取

pub mod grf_mmio {
    define_grf!(
        BIGCORE0_GRF, 0xfd590000, 0x100;
        BIGCORE1_GRF, 0xfd592000, 0x100;
        DSU_GRF, 0xfd598000, 0x100;
        GPU_GRF, 0xfd5a0000, 0x100;
        HDPTXPHY0_GRF, 0xfd5e0000, 0x100;
        HDPTXPHY1_GRF, 0xfd5e4000, 0x100;
        LITCORE_GRF, 0xfd594000, 0x100;
        MIPI_DCPHY0_GRF, 0xfd5e8000, 0x4000;
        MIPI_DCPHY1_GRF, 0xfd5ec000, 0x4000;
        MIPI_DPHY0_GRF, 0xfd5b4000, 0x1000;
        MIPI_DPHY1_GRF, 0xfd5b5000, 0x1000;
        NPU_GRF, 0xfd5a2000, 0x100;
        PHP_GRF, 0xfd5b0000, 0x1000;
        PIPE_PHY0_GRF, 0xfd5bc000, 0x100;
        PIPE_PHY1_GRF, 0xfd5c0000, 0x100;
        PIPE_PHY2_GRF, 0xfd5c4000, 0x100;
        PMU0_GRF, 0xfd588000, 0x2000;
        PMU1_GRF, 0xfd58a000, 0x2000;
        SYS_GRF, 0xfd58c000, 0x1000;
        USB_GRF, 0xfd5ac000, 0x4000;
        USB2PHY0_GRF, 0xfd5d0000, 0x4000;
        USB2PHY1_GRF, 0xfd5d4000, 0x4000;
        USB2PHY2_GRF, 0xfd5d8000, 0x4000;
        USB2PHY3_GRF, 0xfd5dc000, 0x4000;
        USBDPPHY0_GRF, 0xfd5c8000, 0x4000;
        USBDPPHY1_GRF, 0xfd5cc000, 0x4000;
        VO0_GRF, 0xfd5a6000, 0x2000;
        VO1_GRF, 0xfd5a8000, 0x100;
        VOP_GRF, 0xfd5a4000, 0x2000;
    );
}
