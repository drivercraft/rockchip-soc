# 运行测试

安装 `ostool`

```bash
cargo install ostool
```

运行测试

```bash
cargo test --test test -- tests --show-output
# 带uboot的开发板测试
cargo test --test test -- tests --show-output --uboot 
```
