use alloc::vec::Vec;
use core::assert_eq;
use core::ptr::NonNull;
use num_align::NumAlign;
use rockchip_soc::rk3588::{PinConfig, PinManager};
use rockchip_soc::{DriveStrength, Function, GpioDirection, Pull};

use bare_test::{
    fdt_parser::Node,
    globals::{PlatformInfoKind, global_val},
    mem::{iomap, page_size},
};
use log::*;

pub fn test_pin() {
    info!("Testing RK3588 PinManager...");

    let pinctrl = find_pinctrl();

    // 使用 blue_led (GPIO3_B6)
    let test_pin = rockchip_soc::GPIO3_B6;
    info!("Testing pin: {} (GPIO3_B6, blue_led)", test_pin.raw());

    // 验证引脚 ID
    assert_eq!(test_pin.raw(), 110, "Pin ID should be 110");
    assert_eq!(test_pin.bank().raw(), 3, "Bank ID should be 3");
    assert_eq!(test_pin.pin_in_bank(), 14, "Pin in bank should be 14");

    // 读取 u-boot 初始配置
    info!("\n=== Reading u-boot Initial Configuration ===");
    let config = pinctrl
        .get_pin_config(test_pin)
        .expect("Failed to get pin config");

    info!("Pin ID: {}", config.pin_id.raw());
    info!("Bank ID: {}", config.pin_id.bank().raw());
    info!("Pin in Bank: {}", config.pin_id.pin_in_bank());
    info!("Function: {:?}", config.function);
    info!("Pull: {:?}", config.pull);
    info!("Drive: {:?}", config.drive);

    // 验证 Function 是否为 GPIO
    assert!(
        matches!(config.function, Function::Gpio(_)),
        "Function should be GPIO, got {:?}",
        config.function
    );

    // 从 Function 中提取并验证 GPIO 方向
    if let Function::Gpio(direction) = config.function {
        info!("GPIO Direction: {:?}", direction);
    }

    // 读取并显示引脚实际电平值
    let level = pinctrl
        .read_gpio(test_pin)
        .expect("Failed to read GPIO level");
    info!("GPIO Actual Level: {}", level);

    // 测试输出配置
    info!("\n=== Testing Output Configuration ===");
    pinctrl
        .config_peripheral(PinConfig {
            pin_id: test_pin,
            function: Function::Gpio(GpioDirection::Output(false)),
            pull: Pull::PullUp,
            drive: DriveStrength::Ma8,
        })
        .expect("Failed to config as output");
    info!("✓ Configured as output (initial LOW)");

    // 写入 HIGH
    pinctrl
        .write_gpio(test_pin, true)
        .expect("Failed to write HIGH");
    info!("✓ Wrote HIGH (LED should be ON)");

    // 读取验证
    let value = pinctrl.read_gpio(test_pin).expect("Failed to read");
    assert_eq!(value, true, "Read value should be true after writing HIGH");
    info!("✓ Read back value: true (correct)");

    // 写入 LOW
    pinctrl
        .write_gpio(test_pin, false)
        .expect("Failed to write LOW");
    info!("✓ Wrote LOW (LED should be OFF)");

    // 测试输入配置
    info!("\n=== Testing Input Configuration ===");
    pinctrl
        .config_peripheral(PinConfig {
            pin_id: test_pin,
            function: Function::Gpio(GpioDirection::Input),
            pull: Pull::PullUp,
            drive: DriveStrength::Ma8,
        })
        .expect("Failed to config as input");
    info!("✓ Configured as input with PullUp");

    // 使用 get_pin_config 验证输入配置
    let config = pinctrl
        .get_pin_config(test_pin)
        .expect("Failed to get pin config");
    assert!(
        matches!(config.function, Function::Gpio(GpioDirection::Input)),
        "Function should be GPIO Input, got {:?}",
        config.function
    );
    info!("✓ get_pin_config confirmed: Input (correct)");

    // 测试统一接口 - 配置为输出
    info!("\n=== Testing Unified Interface - Output ===");
    pinctrl
        .config_peripheral(PinConfig {
            pin_id: test_pin,
            function: Function::Gpio(GpioDirection::Output(false)),
            pull: Pull::PullUp,
            drive: DriveStrength::Ma8,
        })
        .expect("Failed to config as output");
    info!("✓ Configured as output using PinConfig (initial LOW)");

    // 验证配置
    let config = pinctrl
        .get_pin_config(test_pin)
        .expect("Failed to get pin config");
    if let Function::Gpio(GpioDirection::Output(value)) = config.function {
        assert_eq!(value, false, "Output initial value should be false");
        info!("✓ get_pin_config confirmed: Output(false) (correct)");
    } else {
        panic!("Unexpected function: {:?}", config.function);
    }

    // 验证实际电平
    let level = pinctrl.read_gpio(test_pin).expect("Failed to read");
    assert_eq!(level, false, "Actual level should be false");
    info!("✓ Read actual level: false (correct)");

    info!("\n=== Test Complete ===");
}

fn find_pinctrl() -> PinManager {
    let PlatformInfoKind::DeviceTree(fdt) = &global_val().platform_info;
    let fdt = fdt.get();

    let pinctrl = fdt
        .find_compatible(&["rockchip,rk3588-pinctrl"])
        .next()
        .expect("Failed to find pinctrl node");
    info!("Found node: {}", pinctrl.name());

    let ioc = get_grf(&pinctrl, "rockchip,grf");

    let mut gpio_banks = [NonNull::dangling(); 5];

    for (idx, node) in fdt.find_compatible(&["rockchip,gpio-bank"]).enumerate() {
        if idx >= 5 {
            warn!("More than 5 GPIO banks found, ignoring extra banks");
            break;
        }
        info!("Found GPIO bank node: {}", node.name());
        let reg = node.reg().unwrap().next().unwrap();

        let gpio_mmio = iomap(
            (reg.address as usize).into(),
            reg.size.unwrap_or(0).align_up(page_size()),
        );
        gpio_banks[idx] = gpio_mmio;
    }

    PinManager::new(ioc, gpio_banks)
}

pub fn get_grf(node: &Node, name: &str) -> NonNull<u8> {
    let ph = node.find_property(name).unwrap().u32().into();

    let PlatformInfoKind::DeviceTree(fdt) = &global_val().platform_info;
    let fdt = fdt.get();
    let node = fdt.get_node_by_phandle(ph).unwrap();

    let regs = node.reg().unwrap().collect::<Vec<_>>();
    let start = regs[0].address as usize;
    let end = start + regs[0].size.unwrap_or(0);
    info!("Syscon address range: 0x{:x} - 0x{:x}", start, end);
    let start = start & !(page_size() - 1);
    let end = (end + page_size() - 1) & !(page_size() - 1);
    info!("Aligned Syscon address range: 0x{:x} - 0x{:x}", start, end);
    iomap(start.into(), end - start)
}
