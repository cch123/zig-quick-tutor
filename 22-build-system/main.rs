use std::env;

// ============================================================
// Rust 构建系统演示
//
// Rust 使用 Cargo 作为构建工具和包管理器。
//
// Cargo.toml 示例:
//   [package]
//   name = "my-app"
//   version = "1.0.0"
//   edition = "2021"
//
//   [dependencies]
//   serde = { version = "1.0", features = ["derive"] }
//   tokio = { version = "1", features = ["full"] }
//
//   [dev-dependencies]
//   criterion = "0.5"
//
//   [[bin]]
//   name = "my-app"
//   path = "src/main.rs"
//
//   [lib]
//   name = "my_lib"
//   path = "src/lib.rs"
//
//   [profile.release]
//   lto = true
//   codegen-units = 1
//   strip = true
//
// 常用命令:
//   cargo build              构建（debug 模式）
//   cargo build --release    构建（release 模式）
//   cargo run                编译并运行
//   cargo test               运行测试
//   cargo bench              运行基准测试
//   cargo doc --open         生成并打开文档
//   cargo clippy             代码 lint
//   cargo fmt                格式化代码
//   cargo add serde          添加依赖
//
// 交叉编译:
//   rustup target add aarch64-unknown-linux-gnu
//   cargo build --target aarch64-unknown-linux-gnu
//   # 通常还需要配置 .cargo/config.toml 指定链接器
//
// build.rs 示例（构建脚本）:
//   fn main() {
//       // 链接 C 库
//       println!("cargo:rustc-link-lib=sqlite3");
//
//       // 编译 C 源文件
//       cc::Build::new()
//           .file("src/helper.c")
//           .compile("helper");
//
//       // 生成代码
//       println!("cargo:rerun-if-changed=proto/api.proto");
//   }
// ============================================================

fn main() {
    println!("=== Rust 构建系统信息 ===\n");

    // 环境信息
    println!("目标架构: {}", env::consts::ARCH);
    println!("目标操作系统: {}", env::consts::OS);
    println!("目标系列: {}", env::consts::FAMILY);
    println!("可执行文件后缀: {:?}", env::consts::EXE_SUFFIX);
    println!("动态库后缀: {:?}", env::consts::DLL_SUFFIX);
    println!();

    // 条件编译
    println!("=== 条件编译 ===\n");

    #[cfg(target_os = "macos")]
    println!("运行在 macOS 上");

    #[cfg(target_os = "linux")]
    println!("运行在 Linux 上");

    #[cfg(target_os = "windows")]
    println!("运行在 Windows 上");

    #[cfg(target_arch = "x86_64")]
    println!("架构: x86_64");

    #[cfg(target_arch = "aarch64")]
    println!("架构: aarch64");

    #[cfg(debug_assertions)]
    println!("构建模式: Debug");

    #[cfg(not(debug_assertions))]
    println!("构建模式: Release");

    println!();

    // Cargo 命令说明
    println!("=== Cargo 常用命令 ===\n");
    println!("构建:");
    println!("  cargo build              # debug 模式（快速编译）");
    println!("  cargo build --release    # release 模式（优化）");
    println!("  cargo build --target ... # 交叉编译");
    println!();
    println!("运行与测试:");
    println!("  cargo run                # 编译并运行");
    println!("  cargo test               # 运行测试");
    println!("  cargo bench              # 基准测试");
    println!();
    println!("代码质量:");
    println!("  cargo clippy             # lint 检查");
    println!("  cargo fmt                # 代码格式化");
    println!("  cargo audit              # 安全审计");
    println!();

    // 与 Zig 对比的关键差异
    println!("=== 与 Zig 构建系统的关键差异 ===\n");
    println!("1. 交叉编译:");
    println!("   Zig:  zig build -Dtarget=aarch64-linux-gnu (开箱即用)");
    println!("   Rust: 需要 rustup target add + 配置链接器");
    println!();
    println!("2. C 互操作:");
    println!("   Zig:  内置 C 编译器，直接编译 C 源码");
    println!("   Rust: 需要 cc crate 和外部 C 编译器");
    println!();
    println!("3. 构建配置:");
    println!("   Zig:  build.zig (Zig 代码，完全可编程)");
    println!("   Rust: Cargo.toml (TOML 声明式) + build.rs (Rust 脚本)");
    println!();
    println!("4. 构建速度:");
    println!("   Zig:  编译速度很快（增量编译）");
    println!("   Rust: 首次编译较慢（但增量编译有改善）");
}
