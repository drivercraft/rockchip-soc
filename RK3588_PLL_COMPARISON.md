# RK3588 PLL 配置对比 (C vs Rust)

## C 代码 → Rust 代码映射

### PLL 枚举定义

| C 代码 (cru_rk3588.h:22) | Rust 代码 (pll.rs:10-33) | 状态 |
|--------------------------|--------------------------|------|
| `enum rk3588_pll_id` | `pub enum PllId` | ✅ |

```c
// C
enum rk3588_pll_id {
    B0PLL, B1PLL, LPLL, CPLL, GPLL,
    NPLL, V0PLL, AUPLL, PPLL, PLL_COUNT,
};
```

```rust
// Rust
pub enum PllId {
    B0PLL, B1PLL, LPLL, CPLL, GPLL,
    NPLL, V0PLL, AUPLL, PPLL, _Len,
}
```

### PLL 速率表

| C 代码 (clk_rk3588.c:24) | Rust 代码 (pll.rs:70) | 状态 |
|--------------------------|----------------------|------|
| `rk3588_pll_rates[]` | `PLL_RATE_TABLE` | ✅ |

```c
// C
static struct rockchip_pll_rate_table rk3588_pll_rates[] = {
    RK3588_PLL_RATE(1500000000, 2, 250, 1, 0),
    RK3588_PLL_RATE(1200000000, 2, 200, 1, 0),
    // ...
};
```

```rust
// Rust
pub const PLL_RATE_TABLE: &[PllRateTable] = &[
    pll_rate(1500000000, 2, 250, 1, 0),
    pll_rate(1200000000, 2, 200, 1, 0),
    // ...
];
```

### PLL 配置数组

| C 代码 (clk_rk3588.c:46) | Rust 代码 (pll.rs:117) | 状态 |
|--------------------------|------------------------|------|
| `rk3588_pll_clks[]` | `RK3588_PLL_CLOCKS` | ✅ |

```c
// C
static struct rockchip_pll_clock rk3588_pll_clks[] = {
    [B0PLL] = PLL(pll_rk3588, PLL_B0PLL, RK3588_B0_PLL_CON(0),
                  RK3588_B0_PLL_MODE_CON, 0, 15, 0, rk3588_pll_rates),
    [B1PLL] = PLL(pll_rk3588, PLL_B1PLL, RK3588_B1_PLL_CON(8),
                  RK3588_B1_PLL_MODE_CON, 0, 15, 0, rk3588_pll_rates),
    // ...
};
```

```rust
// Rust
pub const RK3588_PLL_CLOCKS: [PllClock; PllId::_Len as usize] = [
    pll!(B0PLL, b0_pll_con(0), RK3588_B0_PLL_MODE_CON, 0, 15, 0),
    pll!(B1PLL, b1_pll_con(8), RK3588_B1_PLL_MODE_CON, 0, 15, 0),
    // ...
];
```

## 详细对比表

| # | PLL 名称 | C 宏定义 | Rust 枚举 | 寄存器偏移 | 模式偏移 | 模式位移 | 锁定位移 |
|---|----------|---------|-----------|-----------|---------|---------|---------|
| 0 | B0PLL | `PLL_B0PLL` | `PllId::B0PLL` | `0x50000` | `0x50280` | 0 | 15 |
| 1 | B1PLL | `PLL_B1PLL` | `PllId::B1PLL` | `0x52020` | `0x52280` | 0 | 15 |
| 2 | LPLL | `PLL_LPLL` | `PllId::LPLL` | `0x58040` | `0x58280` | 0 | 15 |
| 3 | CPLL | `PLL_CPLL` | `PllId::CPLL` | `0x1a0` | `0x280` | 8 | 15 |
| 4 | GPLL | `PLL_GPLL` | `PllId::GPLL` | `0x1c0` | `0x280` | 2 | 15 |
| 5 | NPLL | `PLL_NPLL` | `PllId::NPLL` | `0x1e0` | `0x280` | 0 | 15 |
| 6 | V0PLL | `PLL_V0PLL` | `PllId::V0PLL` | `0x160` | `0x280` | 4 | 15 |
| 7 | AUPLL | `PLL_AUPLL` | `PllId::AUPLL` | `0x180` | `0x280` | 6 | 15 |
| 8 | PPLL | `PLL_PPLL` | `PllId::PPLL` | `0x8200` | `0x280` | 10 | 15 |

## 寄存器偏移计算对比

### B0PLL (BIGCORE0 PLL)

| 方面 | C 代码 | Rust 代码 | 验证 |
|------|--------|-----------|------|
| CON 偏移 | `RK3588_B0_PLL_CON(0)` | `b0_pll_con(0)` | ✅ `0x50000` |
| MODE 偏移 | `RK3588_B0_PLL_MODE_CON` | `RK3588_B0_PLL_MODE_CON` | ✅ `0x50280` |
| 实现 | `#define RK3588_B0_PLL_CON(x) ((x) * 0x4 + RK3588_BIGCORE0_CRU_BASE)` | `pub const fn b0_pll_con(x: u32) -> u32 { x * 0x4 + RK3588_BIGCORE0_CRU_BASE }` | ✅ |

### GPLL (通用 PLL)

| 方面 | C 代码 | Rust 代码 | 验证 |
|------|--------|-----------|------|
| CON 偏移 | `RK3588_PLL_CON(112)` | `pll_con(112)` | ✅ `0x1c0` |
| MODE 偏移 | `RK3588_MODE_CON0` | `RK3588_MODE_CON0` | ✅ `0x280` |
| MODE 位移 | 直接传 `2` | 直接传 `2` | ✅ |
| 实现 | `#define RK3588_PLL_CON(x) ((x) * 0x4)` | `pub const fn pll_con(x: u32) -> u32 { x * 0x4 }` | ✅ |

## 代码风格对比

### 宏定义风格

**C 代码** (clk_rk3588.c:97):
```c
#define PLL(_type, _id, _con, _mode, _mshift, _lshift, _pflags, _rate_table) \
    { \
        .id = _id, \
        .con_offset = _con, \
        .mode_offset = _mode, \
        .mode_shift = _mshift, \
        .lock_shift = _lshift, \
        .pll_type = _type, \
        .pll_flags = _pflags, \
        .rate_table = _rate_table, \
        .mode_mask = 0, \
    }
```

**Rust 代码** (pll.rs:97):
```rust
macro_rules! pll {
    ($id:ident, $con:expr, $mode:expr, $mshift:expr, $lshift:expr, $pflags:expr) => {
        PllClock {
            id: PllId::$id as u32,
            con_offset: $con,
            mode_offset: $mode,
            mode_shift: $mshift,
            lock_shift: $lshift,
            pll_type: RockchipPllType::Rk3588,
            pll_flags: $pflags,
            rate_table: PLL_RATE_TABLE,
            mode_mask: 0,
        }
    };
}
```

**改进**:
- ✅ 使用 Rust 强类型枚举 `PllId`
- ✅ 编译时类型检查
- ✅ 更简洁的语法
- ✅ 自动推断 `pll_type` 为 `Rk3588`

## 默认频率对比

| PLL | C 常量 (cru_rk3588.h) | Rust 方法 (pll.rs) | 频率值 | 状态 |
|-----|---------------------|-------------------|--------|------|
| LPLL | `LPLL_HZ` | `PllId::LPLL.default_rate()` | 816 MHz | ✅ |
| GPLL | `GPLL_HZ` | `PllId::GPLL.default_rate()` | 1188 MHz | ✅ |
| CPLL | `CPLL_HZ` | `PllId::CPLL.default_rate()` | 1500 MHz | ✅ |
| NPLL | `NPLL_HZ` | `PllId::NPLL.default_rate()` | 850 MHz | ✅ |
| PPLL | `PPLL_HZ` | `PllId::PPLL.default_rate()` | 1100 MHz | ✅ |

## 辅助功能对比

| 功能 | C 代码 | Rust 代码 | 状态 |
|------|--------|-----------|------|
| 通过 ID 获取配置 | 数组索引 `rk3588_pll_clks[id]` | `get_pll(id: PllId)` | ✅ |
| 计算输出频率 | 运行时计算 | `calc_pll_rate()` const fn | ✅ 更好 (编译时) |
| 获取 PLL 名称 | 无 | `PllId::name()` | ✅ 新增 |
| 获取默认频率 | 手动查找 | `PllId::default_rate()` | ✅ 新增 |

## 测试覆盖对比

| 测试项 | C 代码 | Rust 代码 | 状态 |
|--------|--------|-----------|------|
| 频率表数量 | ❌ | `test_pll_rate_table_count()` | ✅ |
| 频率计算验证 | ❌ | `test_pll_rate_calculation()` | ✅ |
| PLL 数量验证 | ❌ | `test_pll_count()` | ✅ |
| ID 顺序验证 | ❌ | `test_pll_ids()` | ✅ |
| 名称映射验证 | ❌ | `test_pll_names()` | ✅ |
| 默认频率验证 | ❌ | `test_pll_default_rates()` | ✅ |
| 寄存器偏移验证 | ❌ | `test_pll_config_offsets()` | ✅ |

## 总结

### 优势

1. **类型安全**: Rust 使用枚举而非整数 ID
2. **编译时检查**: `const fn` 允许编译时计算
3. **零成本抽象**: 宏展开后与 C 代码效率相同
4. **文档完整**: 每个函数都有详细文档注释
5. **测试覆盖**: 包含 7 个单元测试用例

### 完整性

- ✅ 所有 9 个 PLL 配置完整
- ✅ 17 个预设频率配置
- ✅ 寄存器偏移与 C 代码 100% 一致
- ✅ 符合 Rust 最佳实践
- ✅ 遵循 SOLID、KISS、DRY、YAGNI 原则

---

**生成时间**: 2025-12-31
**对比版本**: u-boot-orangepi vs rockchip-soc v0.1.0
