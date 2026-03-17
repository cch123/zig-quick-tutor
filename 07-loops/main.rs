fn main() {
    // === 基本 while 循环 ===
    let mut i: u32 = 0;
    while i < 5 {
        print!("{} ", i);
        i += 1;
    }
    println!();

    // === for..in 迭代范围 ===
    let mut sum: u32 = 0;
    for j in 0..10 {
        if j % 2 == 0 {
            continue;
        }
        sum += j;
    }
    println!("奇数之和 (1..9) = {}", sum);

    // === 迭代切片 ===
    let fruits = ["apple", "banana", "cherry"];
    for fruit in &fruits {
        print!("{} ", fruit);
    }
    println!();

    // === 带索引迭代 ===
    for (idx, fruit) in fruits.iter().enumerate() {
        println!("[{}] {}", idx, fruit);
    }

    // === 范围迭代 ===
    print!("0 到 4: ");
    for n in 0..5 {
        print!("{} ", n);
    }
    println!();

    // === 同时迭代多个切片（用 zip） ===
    let names = ["Alice", "Bob", "Carol"];
    let ages = [30, 25, 28];
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} 的年龄是 {}", name, age);
    }

    // === loop + break 返回值 ===
    let data = [1, 3, 7, 4, 9, 2];
    let found = 'search: {
        for (idx, &v) in data.iter().enumerate() {
            if v > 5 {
                break 'search Some(idx);
            }
        }
        None
    };
    if let Some(fi) = found {
        println!("第一个大于 5 的元素在索引 {}, 值为 {}", fi, data[fi]);
    }

    // === 标签循环 ===
    println!("乘法表中大于 20 的第一个结果:");
    let mut ra = 0;
    let mut rb = 0;
    'outer: for a in 1..10 {
        for b in 1..10 {
            if a * b > 20 {
                ra = a;
                rb = b;
                break 'outer;
            }
        }
    }
    println!("{} x {} = {}", ra, rb, ra * rb);

    // === loop 无限循环 ===
    let mut count = 0;
    loop {
        count += 1;
        if count >= 3 {
            break;
        }
    }
    println!("无限循环执行了 {} 次", count);

    // === loop break 返回值（Rust 特色） ===
    let mut c = 0;
    let result = loop {
        c += 1;
        if c == 10 {
            break c * 2; // loop 返回 20
        }
    };
    println!("loop 返回值: {}", result);
}
