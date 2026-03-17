const std = @import("std");

// ============================================================
// 演示：Zig 构建系统的核心概念
// ============================================================

// 获取编译目标信息（这是 Zig 独有的能力！）
const builtin = @import("builtin");

pub fn main() void {
    const print = std.debug.print;

    print("=== Zig 构建系统信息 ===\n\n", .{});

    // 编译期可以获取的目标平台信息
    print("目标架构: {s}\n", .{@tagName(builtin.cpu.arch)});
    print("目标操作系统: {s}\n", .{@tagName(builtin.os.tag)});
    print("目标 ABI: {s}\n", .{@tagName(builtin.abi)});
    print("字节序: {s}\n", .{@tagName(builtin.cpu.arch.endian())});
    print("指针宽度: {d} bits\n", .{@bitSizeOf(usize)});

    // 优化模式
    print("优化模式: {s}\n", .{@tagName(builtin.mode)});

    print("\n", .{});

    // 条件编译 - 基于目标平台
    print("=== 条件编译 ===\n\n", .{});

    if (builtin.os.tag == .linux) {
        print("运行在 Linux 上\n", .{});
    } else if (builtin.os.tag == .macos) {
        print("运行在 macOS 上\n", .{});
    } else if (builtin.os.tag == .windows) {
        print("运行在 Windows 上\n", .{});
    } else {
        print("运行在其他操作系统上\n", .{});
    }

    // 编译期计算版本信息
    const version = comptime getVersion();
    print("应用版本: {s}\n", .{version});

    // Zig 的优势：编译时确定的优化
    print("\n=== 构建模式说明 ===\n\n", .{});
    print("Zig 提供 4 种构建模式:\n", .{});
    print("  Debug        - 调试信息，安全检查，未优化\n", .{});
    print("  ReleaseSafe  - 优化 + 保留安全检查\n", .{});
    print("  ReleaseFast  - 最大性能优化\n", .{});
    print("  ReleaseSmall - 最小二进制体积\n", .{});

    print("\n=== 交叉编译 ===\n\n", .{});
    print("Zig 的交叉编译无需任何额外工具链:\n", .{});
    print("  zig build -Dtarget=aarch64-linux-gnu\n", .{});
    print("  zig build -Dtarget=x86_64-windows-gnu\n", .{});
    print("  zig build -Dtarget=riscv64-linux-gnu\n", .{});
    print("  zig build -Dtarget=wasm32-wasi\n", .{});
}

fn getVersion() []const u8 {
    return "1.0.0";
}

// ============================================================
// 演示：Zig 可以直接调用 C 代码（无需额外工具链）
// ============================================================

// 示例：使用 C 标准库（Zig 可以直接调用 C）
const c = @cImport({
    @cInclude("stdlib.h");
});

// 这个函数演示 Zig 与 C 的无缝互操作
pub fn cInteropDemo() void {
    // 调用 C 的 abs 函数
    const result = c.abs(-42);
    std.debug.print("C abs(-42) = {d}\n", .{result});
}
