const std = @import("std");
const print = std.debug.print;

// Tagged union：Zig 的"枚举+数据"
const Shape = union(enum) {
    circle: f64, // 半径
    rectangle: struct { w: f64, h: f64 },
    triangle: struct { base: f64, height: f64 },
};

fn area(s: Shape) f64 {
    return switch (s) {
        .circle => |r| std.math.pi * r * r,
        .rectangle => |rect| rect.w * rect.h,
        .triangle => |tri| tri.base * tri.height / 2.0,
    };
}

pub fn main() void {
    // === 基本 switch ===
    const x: i32 = 3;

    // switch 作为表达式
    const label: []const u8 = switch (x) {
        1 => "one",
        2 => "two",
        3 => "three",
        4, 5 => "four or five", // 多值匹配
        6...10 => "six to ten", // 范围匹配（包含两端）
        else => "other", // 必须穷尽，else 兜底
    };
    print("x = {}, label = {s}\n", .{ x, label });

    // === switch 作为语句（带副作用） ===
    const grade: u8 = 85;
    switch (grade / 10) {
        10, 9 => print("优秀 (A)\n", .{}),
        8 => print("良好 (B)\n", .{}),
        7 => print("中等 (C)\n", .{}),
        6 => print("及格 (D)\n", .{}),
        else => print("不及格 (F)\n", .{}),
    }

    // === Tagged union 匹配（类似 Rust 的 enum match） ===
    const shapes = [_]Shape{
        .{ .circle = 5.0 },
        .{ .rectangle = .{ .w = 4.0, .h = 6.0 } },
        .{ .triangle = .{ .base = 3.0, .height = 8.0 } },
    };

    for (shapes) |s| {
        const name: []const u8 = switch (s) {
            .circle => "圆形",
            .rectangle => "矩形",
            .triangle => "三角形",
        };
        print("{s} 的面积 = {d:.2}\n", .{ name, area(s) });
    }

    // === 对 bool 使用 switch ===
    const flag = true;
    const msg: []const u8 = switch (flag) {
        true => "开",
        false => "关",
    };
    print("开关状态: {s}\n", .{msg});
}
