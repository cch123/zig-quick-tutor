const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    // --- 变量声明 ---
    // var: 可变变量
    var x: i32 = 10;
    x += 5;
    print("var x = {}\n", .{x});

    // const: 不可变绑定（优先使用）
    const y: i32 = 42;
    print("const y = {}\n", .{y});

    // 类型推断
    const z = @as(i32, 100);
    print("inferred z = {}\n", .{z});

    // --- 编译期常量 ---
    // comptime: 值必须在编译期已知
    const comptime_val = comptime blk: {
        var result: i32 = 1;
        var i: i32 = 1;
        while (i <= 5) : (i += 1) {
            result *= i;
        }
        break :blk result;
    };
    print("comptime 5! = {}\n", .{comptime_val});

    // 模块级 const 天然是 comptime
    print("PI = {d:.6}\n", .{PI});

    // --- Zig 不允许 Shadowing ---
    // 以下代码无法编译:
    // const a = 10;
    // const a = 20;  // 编译错误！

    // --- undefined ---
    // Zig 允许声明未初始化的变量
    var uninitialized: i32 = undefined;
    // 使用前必须赋值，否则是未定义行为
    uninitialized = 99;
    print("was undefined, now = {}\n", .{uninitialized});

    // --- 块作用域 ---
    {
        const scoped = 123;
        print("scoped = {}\n", .{scoped});
    }
    // scoped 在这里不可见
}

const PI: f64 = 3.141592653589793;
