// Rust 使用 Option<T> 枚举，完全没有 null

fn find_even(items: &[i32]) -> Option<i32> {
    for &item in items {
        if item % 2 == 0 {
            return Some(item);
        }
    }
    None
}

fn greet(name: Option<&str>) {
    // unwrap_or 提供默认值（类似 Zig 的 orelse）
    let actual_name = name.unwrap_or("stranger");
    println!("Hello, {}!", actual_name);
}

// ? 运算符可以提前返回 None
fn first_even_doubled(items: &[i32]) -> Option<i32> {
    let even = find_even(items)?; // 如果 None，直接返回 None
    Some(even * 2)
}

fn main() {
    // 1. 基本 Option
    let mut maybe_num: Option<i32> = Some(42);
    println!("maybe_num = {:?}", maybe_num); // Some(42)
    maybe_num = None;
    println!("maybe_num = {:?}", maybe_num); // None

    // 2. unwrap_or - 提供默认值
    let val = maybe_num.unwrap_or(0);
    println!("val (with default) = {}", val); // 0

    // 3. if let - 安全解包（类似 Zig 的 if 解包）
    let some_val: Option<i32> = Some(100);
    if let Some(v) = some_val {
        println!("Got value: {}", v);
    } else {
        println!("Got None");
    }

    // 4. match - 完整模式匹配
    match some_val {
        Some(v) => println!("Matched: {}", v),
        None => println!("Matched None"),
    }

    // 5. unwrap - 强制解包（None 时 panic，类似 Zig 的 .?）
    let sure_val: Option<i32> = Some(99);
    let unwrapped = sure_val.unwrap();
    println!("Unwrapped: {}", unwrapped);

    // 6. 函数返回 Option
    let items = vec![1, 3, 5, 4, 7];
    if let Some(even) = find_even(&items) {
        println!("First even: {}", even);
    }

    let odds = vec![1, 3, 5, 7];
    match find_even(&odds) {
        Some(even) => println!("First even: {}", even),
        None => println!("No even number found"),
    }

    // 7. map - 对 Option 内部值做转换
    let doubled = some_val.map(|v| v * 2);
    println!("Doubled: {:?}", doubled); // Some(200)

    // 8. and_then - 链式操作（flatmap）
    let result = some_val
        .and_then(|v| if v > 50 { Some(v) } else { None })
        .map(|v| v + 1);
    println!("Chained: {:?}", result); // Some(101)

    // 9. ? 运算符
    println!("Even doubled: {:?}", first_even_doubled(&items)); // Some(8)
    println!("Even doubled: {:?}", first_even_doubled(&odds));  // None

    // 10. Option 参数
    greet(Some("Rust"));
    greet(None);
}
