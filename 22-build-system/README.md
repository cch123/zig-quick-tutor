# 22 - 构建系统 (Build System)

构建系统是语言生态的核心组成部分。Zig 的构建系统是其最独特的设计之一——用 Zig 代码本身来描述构建过程。

## 核心概念对比

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 构建配置文件 | `build.zig`（Zig 代码！） | `go.mod` | `Cargo.toml` |
| 构建命令 | `zig build` | `go build` | `cargo build` |
| 配置格式 | Zig 源码 | Go module DSL | TOML |
| 构建脚本 | 不需要，build.zig 就是脚本 | 不需要 | `build.rs`（Rust 代码） |
| 包管理 | 内置（`zig fetch`） | 内置（go modules） | 内置（crates.io） |
| 交叉编译 | **内置！一行命令** | 内置（GOOS/GOARCH） | 需要安装 target + 配置 |
| 链接 C 库 | 内置支持，无需 CMake | cgo（较复杂） | cc crate / build.rs |
| 构建产物 | 可执行文件/静态库/动态库 | 可执行文件 | 可执行文件/库 |
| 构建模式 | Debug/ReleaseSafe/Fast/Small | 无（默认优化） | debug/release |
| 自定义构建步骤 | Zig 代码，完全可编程 | `go generate` | build.rs |

## 构建系统哲学

### Zig：代码即构建
Zig 的 `build.zig` 是用 Zig 写的——这意味着你拥有一个完整编程语言的所有能力来定义构建过程。不需要学习额外的 DSL（如 CMake、Makefile），也不需要 shell 脚本。你可以在 `build.zig` 中：
- 添加编译目标（可执行文件、库）
- 配置编译选项
- 链接 C/C++ 库
- 定义自定义构建步骤
- 交叉编译到任意目标平台

**交叉编译**是 Zig 的杀手锏：`zig build -Dtarget=aarch64-linux-gnu` 即可交叉编译，无需安装任何额外的交叉编译工具链。Zig 自带所有主流平台的 libc 头文件和系统调用定义。

### Go：简单至上
Go 的构建系统极其简单：`go build` 就够了。`go.mod` 管理依赖，`go generate` 处理代码生成。交叉编译也很简单：`GOOS=linux GOARCH=arm64 go build`。Go 的哲学是减少构建配置的复杂性。

### Rust：Cargo 生态
Rust 的 Cargo 是一个功能强大的构建工具 + 包管理器。`Cargo.toml` 使用 TOML 格式声明项目元数据和依赖。需要自定义构建逻辑时，可以编写 `build.rs`（用 Rust 写的构建脚本）。Cargo 与 crates.io 生态系统紧密集成。

## 交叉编译对比

```bash
# Zig - 一行命令，无需任何额外安装
zig build -Dtarget=aarch64-linux-gnu
zig build -Dtarget=x86_64-windows-gnu
zig build -Dtarget=aarch64-macos-none

# Go - 设置环境变量
GOOS=linux GOARCH=arm64 go build
GOOS=windows GOARCH=amd64 go build

# Rust - 需要先安装 target
rustup target add aarch64-unknown-linux-gnu
# 还需要配置链接器...
cargo build --target aarch64-unknown-linux-gnu
```

## 链接 C 库

Zig 在链接 C 库方面有巨大优势：它内置了 C/C++ 编译器，可以直接编译 C 源文件并链接，无需 CMake、pkg-config 等外部工具。这使得 Zig 成为 C 语言的优秀替代品和互操作伙伴。

## 运行示例

```bash
# Zig - 使用 build.zig 构建
cd 22-build-system && zig build
# 或直接运行
zig run main.zig

# Go
go run main.go
# 或构建
go build -o main main.go

# Rust
rustc main.rs -o main && ./main
# 或用 Cargo（推荐）
# cargo build && cargo run
```

## 文件说明

- `build.zig` - Zig 构建脚本示例，演示各种构建配置
- `main.zig` - Zig 源码
- `main.go` - Go 源码
- `main.rs` - Rust 源码（展示 Cargo.toml 结构说明）
