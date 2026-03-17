const std = @import("std");
const print = std.debug.print;

// --- 基本函数 ---
fn add(a: i32, b: i32) i32 {
    return a + b;
}

// --- 多返回值 (使用结构体/元组) ---
fn swap(a: i32, b: i32) struct { i32, i32 } {
    return .{ b, a };
}

// --- 错误返回 ---
const MathError = error{
    DivisionByZero,
    Overflow,
};

fn safeDivide(a: i32, b: i32) MathError!i32 {
    if (b == 0) return MathError.DivisionByZero;
    return @divTrunc(a, b);
}

// --- 函数指针 ---
fn applyOp(a: i32, b: i32, op: *const fn (i32, i32) i32) i32 {
    return op(a, b);
}

fn multiply(a: i32, b: i32) i32 {
    return a * b;
}

// --- Comptime 参数 ---
fn repeat(comptime n: usize, value: u8) [n]u8 {
    return [_]u8{value} ** n;
}

fn maxOf(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

pub fn main() void {
    // 基本函数调用
    print("add(3, 4) = {}\n", .{add(3, 4)});

    // 多返回值
    const result = swap(10, 20);
    print("swap(10, 20) = {}, {}\n", .{ result[0], result[1] });

    // 错误处理
    if (safeDivide(10, 3)) |val| {
        print("10 / 3 = {}\n", .{val});
    } else |err| {
        print("error: {}\n", .{err});
    }

    if (safeDivide(10, 0)) |val| {
        print("10 / 0 = {}\n", .{val});
    } else |err| {
        print("10 / 0 = error: {}\n", .{err});
    }

    // try 语法糖 (在返回错误的函数中使用)
    // const val = try safeDivide(10, 3);

    // catch 提供默认值
    const safe_val = safeDivide(10, 0) catch 0;
    print("10 / 0 with catch = {}\n", .{safe_val});

    // 函数指针
    print("applyOp(3, 4, multiply) = {}\n", .{applyOp(3, 4, &multiply)});

    // comptime 参数
    const buf = repeat(5, 'A');
    print("repeat(5, 'A') = {s}\n", .{&buf});

    // 泛型函数 (通过 comptime type)
    print("maxOf(i32, 3, 7) = {}\n", .{maxOf(i32, 3, 7)});
    print("maxOf(f64, 1.5, 2.5) = {d:.1}\n", .{maxOf(f64, 1.5, 2.5)});
}
