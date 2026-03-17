const std = @import("std");
const print = std.debug.print;

// 一个返回可选值的函数
fn findEven(items: []const i32) ?i32 {
    for (items) |item| {
        if (@mod(item, 2) == 0) return item;
    }
    return null;
}

// 可选类型作为参数
fn greet(name: ?[]const u8) void {
    // orelse 提供默认值
    const actual_name = name orelse "stranger";
    print("Hello, {s}!\n", .{actual_name});
}

pub fn main() void {
    // 1. 基本可选类型
    var maybe_num: ?i32 = 42;
    print("maybe_num = {?}\n", .{maybe_num}); // 42
    maybe_num = null;
    print("maybe_num = {?}\n", .{maybe_num}); // null

    // 2. orelse - 提供默认值
    const val: i32 = maybe_num orelse 0;
    print("val (with default) = {}\n", .{val}); // 0

    // 3. if 解包 - 安全访问可选值
    const some_val: ?i32 = 100;
    if (some_val) |v| {
        print("Got value: {}\n", .{v});
    } else {
        print("Got null\n", .{});
    }

    // 4. .? 强制解包（null 时会 panic）
    const sure_val: ?i32 = 99;
    const unwrapped = sure_val.?;
    print("Unwrapped: {}\n", .{unwrapped});

    // 5. 函数返回可选值
    const items = [_]i32{ 1, 3, 5, 4, 7 };
    if (findEven(&items)) |even| {
        print("First even: {}\n", .{even});
    }

    const odds = [_]i32{ 1, 3, 5, 7 };
    if (findEven(&odds)) |even| {
        print("First even: {}\n", .{even});
    } else {
        print("No even number found\n", .{});
    }

    // 6. 可选类型作为函数参数
    greet("Zig");
    greet(null);

    // 7. 可选指针 - ?*T
    var x: i32 = 42;
    var opt_ptr: ?*i32 = &x;
    if (opt_ptr) |ptr| {
        print("Pointer value: {}\n", .{ptr.*});
        ptr.* = 100;
    }
    print("x after modification: {}\n", .{x});
    opt_ptr = null;
    print("opt_ptr is null: {}\n", .{opt_ptr == null});
}
