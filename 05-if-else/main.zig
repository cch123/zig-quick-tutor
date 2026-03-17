const std = @import("std");
const print = std.debug.print;

fn getOptionalValue(check: bool) ?i32 {
    if (check) return 42;
    return null;
}

pub fn main() void {
    // === 基本 if/else ===
    const x: i32 = 10;

    if (x > 5) {
        print("x 大于 5\n", .{});
    } else if (x > 0) {
        print("x 大于 0 但不大于 5\n", .{});
    } else {
        print("x 小于等于 0\n", .{});
    }

    // === if 作为表达式（代替三元运算符） ===
    const y: i32 = if (x > 5) 100 else 200;
    print("y = {}\n", .{y});

    // === 用 if 表达式初始化变量 ===
    const label: []const u8 = if (x % 2 == 0) "even" else "odd";
    print("x is {s}\n", .{label});

    // === Optional 捕获：Zig 的特色功能 ===
    const maybe_val: ?i32 = getOptionalValue(true);

    // 如果 optional 有值，用 |val| 捕获
    if (maybe_val) |val| {
        print("捕获到值: {}\n", .{val});
    } else {
        print("值为 null\n", .{});
    }

    // 没有值的情况
    const no_val: ?i32 = getOptionalValue(false);
    if (no_val) |val| {
        print("捕获到值: {}\n", .{val});
    } else {
        print("值为 null\n", .{});
    }

    // === bool 运算 ===
    const a = true;
    const b = false;
    // Zig 用 and/or 关键字，不用 && ||
    if (a and !b) {
        print("a 为 true 且 b 为 false\n", .{});
    }
    if (a or b) {
        print("a 或 b 至少一个为 true\n", .{});
    }
}
