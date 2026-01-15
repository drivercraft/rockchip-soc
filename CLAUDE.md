# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Rockchip SoC 的 Rust `no_std` 实现,提供时钟和复位单元 (CRU) 驱动。当前主要支持 RK3588 芯片。

**核心设计原则**: SOLID、KISS、DRY、YAGNI

## 构建和测试

### 环境要求

- Rust nightly toolchain (见 `rust-toolchain.toml`)
- 目标架构: `aarch64-unknown-none-softfloat`
- 集成测试需要: `ostool` (用于裸机测试运行)

## 参考资料

rockchip 文档： `/home/zhourui/opensource/proj_usb/CrabUSB2/.spec-workflow/Rockchip_RK3588_TRM_V1.0-Part1.md`
设备树：`/home/zhourui/opensource/proj_usb/u-boot-orangepi/orangepi5plus.dts`
u-boot: `/home/zhourui/opensource/proj_usb/u-boot-orangepi`
linux: `/home/zhourui/orangepi-build/kernel/orange-pi-6.1-rk35xx`

## 必须遵守

修改完代码后，确保 `cargo check --test test --target aarch64-unknown-none-softfloat` 可以通过，
执行 `cargo fmt --all` 保持代码风格一致。
使用最新版本的依赖库，use context7 查询使用方法。
使用 `tock-registers` 进行寄存器定义和操作。

### 常用命令

```bash
# 运行所有库测试 (单元测试)
cargo test --lib

# 运行特定模块测试 (PLL 相关)
cargo test --lib pll

# 运行 I2C 时钟测试
cargo test --lib i2c

# 运行 UART 时钟测试
cargo test --lib uart

# 运行集成测试 (需要 ostool 和硬件)
cargo install ostool

# 带 u-boot 的开发板测试
cargo test --test test --target aarch64-unknown-none-softfloat -- uboot
```

### 构建配置

- 使用 `build.rs` 在非 Windows/Linux 平台上自动生成测试框架
- `.cargo/config.toml` 配置了裸机测试运行器 (`cargo osrun`)
- 依赖项包括: `dma-api`、`log`、`mbarrier`、`thiserror`

## 代码架构

### 核心模块结构

```
src/
├── lib.rs              # 库入口,导出公共 API
├── clock/              # 通用时钟模块
│   ├── mod.rs          # ClkId 定义和时钟 ID 常量
│   └── pll.rs          # 通用 PLL 类型定义
├── rst.rs              # 复位控制 (RstId, ResetRockchip)
├── gpio/               # GPIO 支持 (开发中)
├── grf/                # General Register Files
├── syscon/             # 系统控制器
└── variants/           # 芯片变体实现
    ├── mod.rs          # 变体入口
    └── rk3588/         # RK3588 实现
        ├── mod.rs      # 导出 Cru
        ├── cru/        # CRU (Clock and Reset Unit)
        │   ├── mod.rs          # Cru 主结构体和核心方法
        │   ├── pll.rs          # PLL 配置和频率计算
        │   ├── consts.rs       # 寄存器常量和偏移
        │   ├── error.rs        # 错误类型定义
        │   ├── gate.rs         # 时钟门控表
        │   ├── peripheral.rs   # 外设时钟 (I2C, UART, SPI, PWM, ADC, MMC, USB)
        │   └── clock/
        │       └── mod.rs      # 时钟 ID 常量
        └── syscon.rs   # SYSCON 寄存器定义
```

### 关键设计模式

**1. 分层架构**

- **通用层**: `clock/`, `rst/` 提供跨芯片的抽象
- **变体层**: `variants/rk3588/` 实现芯片特定功能
- **硬件层**: 直接 MMIO 操作,使用 `NonNull<u8>` 作为 `Mmio` 类型

**2. 时钟 ID 系统**

- `ClkId(u64)`: 新类型包装,提供类型安全的时钟标识
- 时钟 ID 直接映射到设备树绑定 (`rk3588-cru.h`)
- 支持范围检查 (通过 `RangeBounds` trait)

**3. PLL 管理**

- PLL 频率通过查找表 (`rate_table`) 配置
- 支持整数和小数分频 (通过 K 参数)
- 频率计算公式: `rate = ((fin / p) * m + frac) >> s`

**4. Rockchip 寄存器操作**

使用特殊的写掩码机制:

- 高 16 位: 要清除的位掩码
- 低 16 位: 要设置的值
- 方法: `clrsetreg()`, `clrreg()`, `setreg()`

**5. 错误处理**

- 使用 `thiserror` 定义错误类型
- `ClockError`: 支持时钟、频率读取失败、无效频率等
- `ClockResult<T>`: 类型别名

### RK3588 CRU 模块详解

`Cru` 结构体是核心时钟管理器,提供:

**初始化和验证** (`src/variants/rk3588/cru/mod.rs:87-217`):

- `init()`: 验证 u-boot 配置的 PLL 和时钟分频
- 不修改寄存器,仅读取和验证

**时钟控制**:

- `clk_enable(id)`: 使能时钟 (清除门控 bit)
- `clk_disable(id)`: 禁止时钟 (设置门控 bit)
- `clk_is_enabled(id)`: 检查时钟状态

**频率管理**:

- `clk_get_rate(id)`: 获取时钟频率
- `clk_set_rate(id, rate)`: 设置时钟频率
- 支持的时钟类型: PLL, I2C, UART, SPI, PWM, ADC, MMC, USB, 根时钟

**复位控制**:

- `reset_assert(id)`: 断言复位
- `reset_deassert(id)`: 解除断言

**外设时钟实现** (`peripheral.rs`):

- I2C: 100/200MHz
- UART: 可配置频率
- SPI: 可配置频率
- PWM: 可配置频率
- ADC: SARADC, TSADC
- MMC/EMMC/SDIO/SFC
- USB (新增)

### 时钟门控机制

时钟门控表 (`gate.rs`) 定义每个时钟的:

- 寄存器偏移
- 位位置
- 时钟类型 (普通/复合)

查找流程:

1. `find_clk_gate(id)`: 在门控表中查找时钟
2. `get_gate_reg_offset(gate)`: 计算寄存器偏移
3. Rockchip 写掩码操作使能/禁止

### PLL 配置

**PLL ID 映射** (`pll.rs`):

- PllId 枚举值从 1 开始,匹配设备树绑定
- 9 个 PLL: B0PLL, B1PLL, LPLL, CPLL, GPLL, NPLL, V0PLL, AUPLL, PPLL

**PLL 参数**:

- p: 参考分频
- m: 反馈分频
- s: 输出分频
- k: 小数分频 (0-65535)

**频率计算** (`pll.rs:calc_pll_rate`):

```rust
rate = ((fin / p) * m + (fin * k) / (p * 65536)) >> s
```

**验证方法**:

- `verify_pll_frequency()`: 允许 0.1% 误差

## 测试策略

### 单元测试

- 寄存器位掩码验证
- PLL 频率计算一致性
- u-boot 配置值验证
- PLL 参数查找

### 集成测试

`tests/test.rs` 需要裸机环境:

- 使用 `bare-test` 框架
- 通过 `ostool` 在硬件上运行
- 测试 EMMC 时钟集成

## 文档

详细文档位于 `doc/3588/`:

- `PLL.md`: PLL 配置说明
- `PLL_ID_MAPPING.md`: ID 映射说明
- `TEST_REPORT.md`: 测试报告

## 参考资料

1. u-boot RK3588 时钟驱动: `drivers/clk/rockchip/clk_rk3588.c`
2. RK3588 TRM (需要 NDA)
3. Linux kernel: `drivers/clk/rockchip/clk-rk3588.c`

## 开发注意事项

1. **不要主动执行 git 操作**: 用户未明确要求时,不要计划和执行 git 提交和分支操作
2. **时钟 ID 常量**: 定义在 `src/clock/mod.rs` 和 `src/variants/rk3588/cru/clock/mod.rs`
3. **寄存器偏移**: 定义在 `src/variants/rk3588/cru/consts.rs`
4. **频率常量**: 定义在 `src/variants/rk3588/cru/consts.rs` (如 GPLL_HZ = 1188MHz)
5. **新增外设时钟**: 在 `peripheral.rs` 添加 `xxx_get_rate()` 和 `xxx_set_rate()`,并在 `clk_get_rate()` 和 `clk_set_rate()` 中添加分发逻辑
6. **保持与 u-boot 一致**: 所有配置都应参考 u-boot 实现,确保兼容性

## 添加新芯片支持

1. 在 `variants/` 下创建新目录 (如 `rk3568/`)
2. 实现芯片特定的 `Cru` 结构体
3. 定义寄存器常量和偏移
4. 实现外设时钟支持
5. 在 `variants/mod.rs` 中导出
