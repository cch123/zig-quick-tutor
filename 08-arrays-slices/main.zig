const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    // === 固定大小数组 ===
    const arr = [5]i32{ 1, 2, 3, 4, 5 };
    print("数组: ", .{});
    for (arr) |v| print("{} ", .{v});
    print("\n", .{});

    // 用 _ 让编译器推断长度
    const arr2 = [_]i32{ 10, 20, 30 };
    print("数组长度: {}\n", .{arr2.len});

    // 全部初始化为相同值
    const zeros = [_]i32{0} ** 5;
    print("全零: ", .{});
    for (zeros) |v| print("{} ", .{v});
    print("\n", .{});

    const threes = [_]i32{3} ** 4;
    print("全三: ", .{});
    for (threes) |v| print("{} ", .{v});
    print("\n", .{});

    // === 切片（胖指针：指针 + 长度） ===
    const full_slice: []const i32 = &arr;
    print("完整切片长度: {}\n", .{full_slice.len});

    // 切片操作 arr[start..end]，左闭右开
    const sub: []const i32 = arr[1..4];
    print("arr[1..4]: ", .{});
    for (sub) |v| print("{} ", .{v});
    print("\n", .{});

    // 可变切片
    var mut_arr = [_]i32{ 1, 2, 3, 4, 5 };
    const mut_slice: []i32 = &mut_arr;
    mut_slice[0] = 99;
    print("修改后: ", .{});
    for (mut_slice) |v| print("{} ", .{v});
    print("\n", .{});

    // === 哨兵终止数组（Zig 特色） ===
    // 字符串字面量就是哨兵终止的：*const [N:0]u8
    const hello: [:0]const u8 = "hello";
    print("字符串长度: {}, 内容: {s}\n", .{ hello.len, hello });

    // === 编译期数组拼接 ===
    const a = [_]i32{ 1, 2 };
    const b = [_]i32{ 3, 4 };
    const c = a ++ b; // 编译期拼接
    print("拼接: ", .{});
    for (c) |v| print("{} ", .{v});
    print("\n", .{});

    // === 动态数组（ArrayList） ===
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Zig 0.15: ArrayList 是 unmanaged，allocator 需要传给每个方法
    var list: std.ArrayList(i32) = .{};
    defer list.deinit(allocator);

    // 添加元素
    try list.append(allocator, 10);
    try list.append(allocator, 20);
    try list.append(allocator, 30);
    try list.appendSlice(allocator, &[_]i32{ 40, 50 });

    print("动态数组: ", .{});
    for (list.items) |v| print("{} ", .{v});
    print("\n长度: {}, 容量: {}\n", .{ list.items.len, list.capacity });

    // === 多维数组 ===
    const matrix = [3][3]i32{
        [_]i32{ 1, 2, 3 },
        [_]i32{ 4, 5, 6 },
        [_]i32{ 7, 8, 9 },
    };

    print("矩阵:\n", .{});
    for (matrix) |row| {
        for (row) |val| {
            print("{:>3} ", .{val});
        }
        print("\n", .{});
    }

    // === 传递切片给函数 ===
    const s = sumSlice(&arr);
    print("数组之和: {}\n", .{s});
}

fn sumSlice(slice: []const i32) i32 {
    var total: i32 = 0;
    for (slice) |v| {
        total += v;
    }
    return total;
}
