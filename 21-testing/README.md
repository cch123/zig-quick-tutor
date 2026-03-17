# 21 - 测试 (Testing)

测试是软件开发中至关重要的环节。Zig、Go、Rust 都提供了内置的测试支持，但设计理念各有不同。

## 核心概念对比

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 测试声明 | `test "名称" { }` 块 | `func TestXxx(t *testing.T)` | `#[test] fn xxx()` |
| 运行命令 | `zig test file.zig` | `go test` | `cargo test` |
| 断言 | `std.testing.expect*` | `t.Error/t.Fatal` | `assert!` / `assert_eq!` |
| 测试位置 | 与源码同文件 | `_test.go` 文件 | 同文件 `#[cfg(test)]` 模块 |
| 内存泄漏检测 | `std.testing.allocator` 内置！ | 无内置（需外部工具） | 无内置（需 valgrind 等） |
| 表驱动测试 | 编译期数组 + inline for | 切片 + for 循环（惯用法） | `Vec` + for 循环 |
| 基准测试 | 无内置 | `func BenchmarkXxx(b *testing.B)` | `#[bench]`（nightly）/ criterion |
| 测试过滤 | `zig test --test-filter "名称"` | `go test -run "正则"` | `cargo test 名称` |
| 跳过测试 | `return error.SkipZigTest` | `t.Skip()` | `#[ignore]` |
| 并行测试 | 默认并行 | 默认并行 | 默认并行 |

## 测试哲学

### Zig：测试即代码
Zig 的测试块直接写在源码文件中，与被测代码紧密相邻。最突出的特性是 **`std.testing.allocator`**——一个特殊的分配器，它能在测试结束时自动检测内存泄漏！这在系统编程语言中非常有价值。`zig test` 编译时，测试块会被编译运行；正常 `zig build` 时，测试块会被完全忽略。

### Go：约定优于配置
Go 的测试遵循简单的文件命名约定（`_test.go`）和函数命名约定（`TestXxx`）。表驱动测试（table-driven tests）是 Go 社区的最佳实践，通过一个测试用例切片来覆盖多种场景。`go test` 命令可以自动发现和运行所有测试。

### Rust：模块化测试
Rust 使用属性宏（`#[test]`、`#[cfg(test)]`）来标记测试。单元测试通常放在源文件底部的 `#[cfg(test)] mod tests` 模块中，集成测试放在 `tests/` 目录。`assert!`、`assert_eq!`、`assert_ne!` 是标准的断言宏。

## 运行示例

```bash
# Zig - 运行文件中的所有测试
zig test main.zig

# Go - 运行当前目录的测试
go test -v

# Rust - 编译运行测试（此处为单文件示例）
rustc --test main.rs -o test_main && ./test_main
```

## Zig 的杀手锏：内存泄漏检测

Zig 的 `std.testing.allocator` 是一个包装了通用分配器的调试分配器。当测试结束时，如果有未释放的内存，测试会自动失败并报告泄漏位置。这意味着：

- 不需要 valgrind 或 AddressSanitizer
- 内存泄漏检测是测试框架的一等公民
- 检测发生在测试级别，能精确定位到哪个测试泄漏了内存

这是 Zig 相比 Go 和 Rust 在测试方面最独特的优势。
