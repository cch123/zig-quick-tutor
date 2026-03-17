const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    // === 基本 while 循环 ===
    var i: u32 = 0;
    while (i < 5) {
        print("{} ", .{i});
        i += 1;
    }
    print("\n", .{});

    // === while 带 continue 表达式 ===
    // : (j += 1) 在每次迭代末尾执行，包括 continue 时
    // 这类似于 C 的 for(j=0; j<10; j++)，但更安全
    var sum: u32 = 0;
    var j: u32 = 0;
    while (j < 10) : (j += 1) {
        if (j % 2 == 0) continue; // j += 1 仍会执行
        sum += j;
    }
    print("奇数之和 (1..9) = {}\n", .{sum}); // 1+3+5+7+9=25

    // === for 迭代切片 ===
    const fruits = [_][]const u8{ "apple", "banana", "cherry" };
    for (fruits) |fruit| {
        print("{s} ", .{fruit});
    }
    print("\n", .{});

    // === for 带索引 ===
    for (fruits, 0..) |fruit, idx| {
        print("[{}] {s}\n", .{ idx, fruit });
    }

    // === for 迭代范围 ===
    print("0 到 4: ", .{});
    for (0..5) |n| {
        print("{} ", .{n});
    }
    print("\n", .{});

    // === 同时迭代多个切片 ===
    const names = [_][]const u8{ "Alice", "Bob", "Carol" };
    const ages = [_]u32{ 30, 25, 28 };
    for (names, ages) |name, age| {
        print("{s} 的年龄是 {}\n", .{ name, age });
    }

    // === break 返回值（while 作为表达式） ===
    const data = [_]i32{ 1, 3, 7, 4, 9, 2 };
    var idx: usize = 0;
    const found: ?usize = while (idx < data.len) : (idx += 1) {
        if (data[idx] > 5) break idx;
    } else null; // 没找到时返回 null

    if (found) |fi| {
        print("第一个大于 5 的元素在索引 {}, 值为 {}\n", .{ fi, data[fi] });
    }

    // === 标签循环（嵌套循环 break/continue） ===
    print("乘法表中大于 20 的第一个结果:\n", .{});
    const result = outer: for (1..10) |a| {
        for (1..10) |b| {
            if (a * b > 20) {
                break :outer .{ a, b };
            }
        }
    } else .{ @as(usize, 0), @as(usize, 0) };

    print("{} x {} = {}\n", .{ result[0], result[1], result[0] * result[1] });

    // === 无限循环 ===
    var count: u32 = 0;
    while (true) {
        count += 1;
        if (count >= 3) break;
    }
    print("无限循环执行了 {} 次\n", .{count});
}
