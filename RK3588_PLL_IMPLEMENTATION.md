# RK3588 PLL 时钟配置实现总结

## 📋 概述

本文档说明了 RK3588 PLL (Phase-Locked Loop) 时钟配置的 Rust 实现，该实现基于 u-boot-orangepi 的 C 代码 (`drivers/clk/rockchip/clk_rk3588.c`)。

## 🎯 实现目标

参考 `clk_rk3588.c:46` 中的 `rk3588_pll_clks[]` 数组，实现完整的 RK3588 PLL 时钟配置。

## ✅ 已完成内容

### 1. PLL ID 枚举 (`PllId`)

位置: `src/variants/rk3588/cru/pll.rs:10-33`

定义了 RK3588 的 9 个 PLL:

```rust
pub enum PllId {
    B0PLL,   // BIGCORE0 PLL - 大核0 PLL
    B1PLL,   // BIGCORE1 PLL - 大核1 PLL
    LPLL,    // DSU PLL - 小核共享单元 PLL
    CPLL,    // 中心/通用 PLL
    GPLL,    // 通用 PLL
    NPLL,    // 网络/视频 PLL
    V0PLL,   // 视频 PLL
    AUPLL,   // 音频 PLL
    PPLL,    // PMU PLL
    _Len,    // PLL 总数
}
```

**对应 C 代码**: `cru_rk3588.h:22` 的 `enum rk3588_pll_id`

### 2. PLL 速率表 (`PLL_RATE_TABLE`)

位置: `src/variants/rk3588/cru/pll.rs:70-95`

定义了 17 个预设频率配置，支持 100MHz - 1.5GHz 范围:

```rust
pub const PLL_RATE_TABLE: &[PllRateTable] = &[
    pll_rate(1500000000, 2, 250, 1, 0),  // 1.5 GHz
    pll_rate(1200000000, 2, 200, 1, 0),  // 1.2 GHz
    pll_rate(1188000000, 2, 198, 1, 0),  // 1.188 GHz (GPLL 默认)
    pll_rate(786432000, 2, 262, 2, 9437), // 786.432 MHz (小数分频)
    // ... 更多频率
];
```

**对应 C 代码**: `clk_rk3588.c:24` 的 `rk3588_pll_rates[]`

### 3. PLL 时钟配置数组 (`RK3588_PLL_CLOCKS`)

位置: `src/variants/rk3588/cru/pll.rs:117-143`

完整的 9 个 PLL 配置:

```rust
pub const RK3588_PLL_CLOCKS: [PllClock; PllId::_Len as usize] = [
    // B0PLL - BIGCORE0 PLL (偏移 0x50000)
    pll!(B0PLL, b0_pll_con(0), RK3588_B0_PLL_MODE_CON, 0, 15, 0),
    // B1PLL - BIGCORE1 PLL (偏移 0x52000)
    pll!(B1PLL, b1_pll_con(8), RK3588_B1_PLL_MODE_CON, 0, 15, 0),
    // LPLL - DSU PLL (偏移 0x58000)
    pll!(LPLL, lpll_con(16), RK3588_LPLL_MODE_CON, 0, 15, 0),
    // V0PLL - 视频 PLL (偏移 0x160)
    pll!(V0PLL, pll_con(88), RK3588_MODE_CON0, 4, 15, 0),
    // AUPLL - 音频 PLL (偏移 0x180)
    pll!(AUPLL, pll_con(96), RK3588_MODE_CON0, 6, 15, 0),
    // CPLL - 中心/通用 PLL (偏移 0x1a0)
    pll!(CPLL, pll_con(104), RK3588_MODE_CON0, 8, 15, 0),
    // GPLL - 通用 PLL (偏移 0x1c0)
    pll!(GPLL, pll_con(112), RK3588_MODE_CON0, 2, 15, 0),
    // NPLL - 网络/视频 PLL (偏移 0x1e0)
    pll!(NPLL, pll_con(120), RK3588_MODE_CON0, 0, 15, 0),
    // PPLL - PMU PLL (偏移 0x8000)
    pll!(PPLL, pmu_pll_con(128), RK3588_MODE_CON0, 10, 15, 0),
];
```

**对应 C 代码**: `clk_rk3588.c:46` 的 `rk3588_pll_clks[]`

### 4. 辅助函数

#### 4.1 `get_pll(id: PllId)`
位置: `pll.rs:167-170`

通过 ID 获取 PLL 配置的便捷函数。

#### 4.2 `calc_pll_rate(fin, p, m, s, k)`
位置: `pll.rs:191-220`

计算 RK3588 PLL 输出频率:

```text
整数分频: FOUT = (FIN * M) / (P * (2^S))
小数分频: FOUT = (FIN * (M + K/65536)) / (P * (2^S))
```

### 5. 测试用例

位置: `src/variants/rk3588/cru/pll.rs:222-293`

包含完整的单元测试:
- ✅ 频率表项数量验证
- ✅ PLL 频率计算测试 (整数和小数分频)
- ✅ PLL 数量和 ID 顺序验证
- ✅ PLL 名称和默认频率验证
- ✅ 寄存器偏移验证

## 📊 寄存器映射

| PLL   | CON Offset | Mode Offset | Mode Shift | Lock Shift | 用途             |
|-------|-----------|-------------|------------|------------|------------------|
| B0PLL | 0x50000   | 0x50280     | 0          | 15         | 大核0 PLL        |
| B1PLL | 0x52000+32| 0x52280     | 0          | 15         | 大核1 PLL        |
| LPLL  | 0x58000+64| 0x58280     | 0          | 15         | 小核 PLL         |
| V0PLL | 0x160     | 0x280       | 4          | 15         | 视频 PLL         |
| AUPLL | 0x180     | 0x280       | 6          | 15         | 音频 PLL         |
| CPLL  | 0x1a0     | 0x280       | 8          | 15         | 中心/通用 PLL    |
| GPLL  | 0x1c0     | 0x280       | 2          | 15         | 通用 PLL         |
| NPLL  | 0x1e0     | 0x280       | 0          | 15         | 网络/视频 PLL    |
| PPLL  | 0x8000+512| 0x280       | 10         | 15         | PMU PLL          |

## 🔧 关键技术细节

### RK3588 PLL 参数

RK3588 使用 (p, m, s, k) 参数格式:
- **p**: Pre-divider (预分频)
- **m**: Main divider (主分频)
- **s**: Post-divider power (后分频指数，2^S)
- **k**: Fractional divider (小数分频，16-bit)

### 示例计算

```rust
// GPLL @ 1.188 GHz (整数分频)
calc_pll_rate(24_000_000, 2, 198, 1, 0)
= (24MHz * 198) / (2 * 2^1)
= 4752MHz / 4
= 1188MHz ✓

// 小数分频示例 @ 786.432 MHz
calc_pll_rate(24_000_000, 2, 262, 2, 9437)
= (24MHz * (262 + 9437/65536)) / (2 * 2^2)
≈ 786.432MHz ✓
```

## 📁 文件结构

```
rockchip-soc/src/
├── clock/
│   └── pll.rs              # PLL 基础类型定义
└── variants/rk3588/cru/
    ├── consts.rs           # CRU 寄存器常量
    ├── mod.rs              # 模块导出
    └── pll.rs              # RK3588 PLL 配置 (本实现)
```

## 🎓 设计原则遵循

1. **SOLID 原则**:
   - ✅ **单一职责**: `PllId` 枚举、配置数组、计算函数职责明确
   - ✅ **开闭原则**: 通过 `PllRateTable` 可扩展频率配置

2. **KISS (简单至上)**:
   - ✅ 使用 `const fn` 宏简化配置语法
   - ✅ 清晰的注释说明每个 PLL 用途

3. **DRY (不重复)**:
   - ✅ 使用 `pll!` 宏消除重复代码
   - ✅ 寄存器偏移函数复用

4. **YAGNI (精益求精)**:
   - ✅ 只实现必需的 9 个 PLL
   - ✅ 预设频率表覆盖常用场景

## 🔍 验证清单

- [x] 所有 9 个 PLL 配置完整
- [x] 寄存器偏移与 C 代码一致
- [x] 频率表包含 17 个预设值
- [x] 单元测试覆盖关键功能
- [x] 代码可编译通过
- [x] 文档完整 (注释 + markdown)

## 📚 参考资料

1. u-boot 源码: `u-boot-orangepi/drivers/clk/rockchip/clk_rk3588.c`
2. 头文件: `u-boot-orangepi/arch/arm/include/asm/arch-rockchip/cru_rk3588.h`
3. RK3588 TRM (技术参考手册)

## 🚀 后续工作

1. 添加 PLL 设置/获取频率的运行时函数
2. 实现 PLL 锁定状态检测
3. 添加 PLL 模式切换支持
4. 集成到完整的 CRU 驱动中

---

**作者**: AI Assistant (Claude)
**日期**: 2025-12-31
**版本**: 1.0
