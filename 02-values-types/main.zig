const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    // 整数类型
    const a: i8 = -128;
    const b: u8 = 255;
    const c: i32 = 42;
    const d: u64 = 18_446_744_073_709_551_615;

    print("i8: {}\n", .{a});
    print("u8: {}\n", .{b});
    print("i32: {}\n", .{c});
    print("u64: {}\n", .{d});

    // 浮点类型
    const f1: f32 = 3.14;
    const f2: f64 = 2.718281828;
    print("f32: {d:.2}\n", .{f1});
    print("f64: {d:.9}\n", .{f2});

    // 布尔类型
    const yes: bool = true;
    const no: bool = false;
    print("bool: {} {}\n", .{ yes, no });

    // 字符 (u8)
    const ch: u8 = 'A';
    print("char: {c}\n", .{ch});
    print("char as int: {}\n", .{ch});

    // comptime_int: 编译期整数，无大小限制
    const big = 1_000_000_000_000_000;
    const result: u64 = big * 2;
    print("comptime_int result: {}\n", .{result});

    // 类型推断
    const inferred = @as(i32, 100); // 显式转换
    print("inferred: {}\n", .{inferred});
}
