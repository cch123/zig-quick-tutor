const std = @import("std");
const print = std.debug.print;

fn increment(ptr: *i32) void {
    ptr.* += 1; // 通过指针修改值
}

fn sum(values: []const i32) i32 {
    var total: i32 = 0;
    for (values) |v| {
        total += v;
    }
    return total;
}

pub fn main() void {
    // 1. 基本指针 *T
    var x: i32 = 42;
    const ptr: *i32 = &x;
    print("x = {}, *ptr = {}\n", .{ x, ptr.* });

    // 通过指针修改值
    ptr.* = 100;
    print("After ptr.* = 100: x = {}\n", .{x});

    // 2. 通过指针传参
    increment(&x);
    print("After increment: x = {}\n", .{x}); // 101

    // 3. const 指针 vs 可变指针
    const y: i32 = 10;
    const const_ptr: *const i32 = &y;
    print("const_ptr.* = {}\n", .{const_ptr.*});
    // const_ptr.* = 20; // 编译错误！不能通过 const 指针修改

    // 4. 可选指针 ?*T（替代 null 指针）
    var val: i32 = 55;
    var opt_ptr: ?*i32 = &val;
    if (opt_ptr) |p| {
        print("opt_ptr has value: {}\n", .{p.*});
    }
    opt_ptr = null;
    if (opt_ptr) |_| {
        print("This won't print\n", .{});
    } else {
        print("opt_ptr is null\n", .{});
    }

    // 5. 多元素指针 [*]T 和指针算术
    var arr = [_]i32{ 10, 20, 30, 40, 50 };
    const many_ptr: [*]i32 = &arr;
    print("many_ptr[0] = {}\n", .{many_ptr[0]});
    print("many_ptr[2] = {}\n", .{many_ptr[2]});

    // 指针算术
    const offset_ptr = many_ptr + 2;
    print("(many_ptr + 2)[0] = {}\n", .{offset_ptr[0]}); // 30

    // 6. 多元素指针转切片（更安全）
    const slice: []i32 = arr[0..5];
    print("slice sum = {}\n", .{sum(slice)});

    // 7. 哨兵终止指针 [*:0]u8（C 字符串兼容）
    const c_str: [*:0]const u8 = "Hello from Zig";
    // 转换为切片使用
    const zig_str = std.mem.span(c_str);
    print("C string: {s} (len={})\n", .{ zig_str, zig_str.len });

    // 8. @ptrCast - 指针类型转换
    const bytes = [_]u8{ 0x01, 0x00, 0x00, 0x00 };
    const byte_ptr: [*]const u8 = &bytes;
    // 将字节指针视为 u32 指针（注意字节序）
    const int_ptr: *const u32 = @ptrCast(@alignCast(byte_ptr));
    print("Bytes as u32: {}\n", .{int_ptr.*});

    // 9. 指针与数组的关系
    var nums = [_]i32{ 1, 2, 3 };
    const nums_ptr: *[3]i32 = &nums;
    // 数组指针可以自动转为切片
    const nums_slice: []i32 = nums_ptr;
    print("nums_slice[1] = {}\n", .{nums_slice[1]});
}
