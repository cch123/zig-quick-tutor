# 08 - Arrays & Slices 数组与切片

## 概述

数组和切片是最常用的数据结构。三种语言在这方面差异很大：Go 的切片是动态的并自带 `append`；Zig 的切片是胖指针（指针+长度）；Rust 区分固定大小数组、切片引用和 `Vec`。

## 核心差异

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 固定数组 | `[5]i32` | `[5]int` | `[i32; 5]` |
| 切片类型 | `[]T`（胖指针） | `[]T`（ptr+len+cap） | `&[T]`（胖指针） |
| 动态数组 | `std.ArrayList(T)` | 内置切片 `[]T` | `Vec<T>` |
| 数组初始化 | `.{1, 2, 3}` | `[3]int{1, 2, 3}` | `[1, 2, 3]` |
| 零值初始化 | `[_]i32{0} ** 5` 或 `std.mem.zeroes` | 自动零值 | `[0; 5]` |
| 切片操作 | `arr[1..3]` | `arr[1:3]` | `&arr[1..3]` |
| 追加元素 | `ArrayList.append()` | `append(slice, elem)` | `vec.push(elem)` |
| 哨兵终止 | `[*:0]u8` | 无 | 无（CString 除外） |
| 多维数组 | `[M][N]T` | `[M][N]T` | `[[T; N]; M]` |
| 编译期长度 | 必须编译期已知 | 必须编译期已知 | 必须编译期已知 |

## Zig 特色

1. **切片是胖指针**：`[]T` 包含指针和长度，没有容量。
2. **哨兵终止数组**：`[*:0]u8` 表示以 0 结尾的数组，兼容 C 字符串。
3. **编译期数组操作**：`++`（拼接）和 `**`（重复）在编译期工作。
4. **显式分配器**：动态数组需要传入分配器，内存管理完全透明。

## Go 特色

1. **切片三元组**：Go 切片内部是 `(ptr, len, cap)`，有容量概念。
2. **内置 append**：`append` 是内置函数，自动扩容。
3. **make 创建**：`make([]T, len, cap)` 预分配容量。
4. **切片表达式**：支持三索引切片 `a[low:high:max]` 控制容量。

## Rust 特色

1. **所有权区分**：`Vec<T>` 拥有数据，`&[T]` 是借用的切片。
2. **切片是引用**：`&arr[1..3]` 是对原数组的引用，不拷贝。
3. **丰富的方法**：Vec 和切片有大量内置方法（sort, contains, windows 等）。
4. **into_iter vs iter**：迭代器有所有权和借用之分。

## 内存布局对比

```
Zig []T:     [pointer][length]           （16 字节）
Go  []T:     [pointer][length][capacity]  （24 字节）
Rust &[T]:   [pointer][length]           （16 字节）
Rust Vec<T>: [pointer][length][capacity]  （24 字节，在栈上）
```

## 运行方式

```bash
# Zig
zig run main.zig

# Go
go run main.go

# Rust
rustc main.rs -o main_rs && ./main_rs
```
