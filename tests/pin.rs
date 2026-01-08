use alloc::vec::Vec;
use core::ptr::NonNull;
use num_align::NumAlign;
use rockchip_soc::{
    Pull,
    rk3588::{PinManager, gpio::GpioDirection},
};

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

    // 设备树配置验证
    info!("=== Device Tree Configuration ===");
    info!("Expected: GPIO3_B6 (Pin 110)");
    info!("  - Bank: 3");
    info!("  - Pin in Bank: 14 (0x06 + 8)");
    info!("  - Function: GPIO");
    info!("  - Pull: PullUp");
    info!("  - Drive: Default");

    info!("=== Actual State ===");
    info!("  - Pin ID: {}", test_pin.raw());
    info!("  - Bank ID: {}", test_pin.bank().raw());
    info!("  - Pin in Bank: {}", test_pin.pin_in_bank());

    // 使用 get_pin_config 获取完整配置
    info!("\n=== Reading Pin Configuration via get_pin_config() ===");
    match pinctrl.get_pin_config(test_pin) {
        Ok(config) => {
            info!("Pin ID: {}", config.pin_id.raw());
            info!("Bank ID: {}", config.bank_id.raw());
            info!("Pin in Bank: {}", config.pin_in_bank);
            info!("Function: {:?}", config.function);
            info!("Pull: {:?}", config.pull);
            info!("Drive: {:?}", config.drive);

            match config.gpio_direction {
                Some(GpioDirection::Input) => {
                    info!("GPIO Direction: Input");
                }
                Some(GpioDirection::Output(value)) => {
                    info!("GPIO Direction: Output (value: {})", value);
                }
                None => {
                    info!("GPIO Direction: Unknown");
                }
            }

            if let Some(level) = config.gpio_level {
                info!("GPIO Actual Level: {}", level);
            }
        }
        Err(e) => {
            warn!("Failed to get pin config: {:?}", e);
        }
    }

    // 读取初始状态（兼容性测试）
    info!("\n=== Legacy API Test ===");
    let bank = pinctrl.gpio_bank(test_pin.bank()).unwrap();
    let pin_in_bank = test_pin.pin_in_bank();

    // 读取方向配置（同时获取输出值）
    match bank.get_direction(pin_in_bank) {
        Ok(GpioDirection::Input) => {
            info!("Initial direction: Input");
        }
        Ok(GpioDirection::Output(value)) => {
            info!("Initial direction: Output (current value: {})", value);
        }
        Err(e) => {
            warn!("Failed to get direction: {:?}", e);
        }
    }

    // 读取引脚实际电平值
    match bank.read(pin_in_bank) {
        Ok(value) => {
            info!("Pin actual level: {}", value);
        }
        Err(e) => {
            warn!("Failed to read pin: {:?}", e);
        }
    }

    // 测试输出配置
    info!("\n=== Testing Output Configuration ===");
    if let Err(e) = pinctrl.config_gpio_output(test_pin, Pull::PullUp, false) {
        warn!("Failed to config as output: {:?}", e);
    } else {
        info!("✓ Configured as output (initial LOW)");

        // 写入 HIGH
        if let Err(e) = pinctrl.write_gpio(test_pin, true) {
            warn!("Failed to write HIGH: {:?}", e);
        } else {
            info!("✓ Wrote HIGH (LED should be ON)");
        }

        // 读取验证
        match pinctrl.read_gpio(test_pin) {
            Ok(value) => {
                info!("✓ Read back value: {} (expected: true)", value);
            }
            Err(e) => {
                warn!("Failed to read: {:?}", e);
            }
        }

        // 写入 LOW
        if let Err(e) = pinctrl.write_gpio(test_pin, false) {
            warn!("Failed to write LOW: {:?}", e);
        } else {
            info!("✓ Wrote LOW (LED should be OFF)");
        }
    }

    // 测试输入配置
    info!("\n=== Testing Input Configuration ===");
    if let Err(e) = pinctrl.config_gpio_input(test_pin, Pull::PullUp) {
        warn!("Failed to config as input: {:?}", e);
    } else {
        info!("✓ Configured as input with PullUp");
    }

    // 测试统一接口（使用 DirectionConfig）
    info!("\n=== Testing Unified Interface (DirectionConfig) ===");

    // 使用统一接口配置为输出
    if let Err(e) = pinctrl.config_gpio(test_pin, Pull::PullUp, GpioDirection::Output(false)) {
        warn!("Failed to config as output (unified): {:?}", e);
    } else {
        info!("✓ Configured as output using DirectionConfig (initial LOW)");

        // 验证方向配置（同时读取输出值）
        let bank = pinctrl.gpio_bank(test_pin.bank()).unwrap();
        let pin_in_bank = test_pin.pin_in_bank();
        match bank.get_direction(pin_in_bank) {
            Ok(GpioDirection::Output(value)) => {
                info!(
                    "✓ get_direction returned: Output({}) (expected: false)",
                    value
                );
            }
            Ok(GpioDirection::Input) => {
                warn!("✗ Unexpected: get_direction returned Input");
            }
            Err(e) => {
                warn!("Failed to get direction: {:?}", e);
            }
        }

        // 验证实际电平
        match pinctrl.read_gpio(test_pin) {
            Ok(value) => {
                info!("✓ Read actual level: {} (expected: false)", value);
            }
            Err(e) => {
                warn!("Failed to read: {:?}", e);
            }
        }
    }

    // 使用统一接口配置为输入
    if let Err(e) = pinctrl.config_gpio(test_pin, Pull::PullUp, GpioDirection::Input) {
        warn!("Failed to config as input (unified): {:?}", e);
    } else {
        info!("✓ Configured as input using DirectionConfig");

        // 验证方向配置
        let bank = pinctrl.gpio_bank(test_pin.bank()).unwrap();
        let pin_in_bank = test_pin.pin_in_bank();
        match bank.get_direction(pin_in_bank) {
            Ok(GpioDirection::Input) => {
                info!("✓ get_direction returned: Input (correct)");
            }
            Ok(GpioDirection::Output(_)) => {
                warn!("✗ Unexpected: get_direction returned Output");
            }
            Err(e) => {
                warn!("Failed to get direction: {:?}", e);
            }
        }
    }

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
