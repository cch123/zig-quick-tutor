fn get_optional_value(check: bool) -> Option<i32> {
    if check {
        Some(42)
    } else {
        None
    }
}

fn main() {
    // === 基本 if/else ===
    let x: i32 = 10;

    if x > 5 {
        println!("x 大于 5");
    } else if x > 0 {
        println!("x 大于 0 但不大于 5");
    } else {
        println!("x 小于等于 0");
    }

    // === if 作为表达式（代替三元运算符） ===
    let y: i32 = if x > 5 { 100 } else { 200 };
    println!("y = {}", y);

    // === 用 if 表达式初始化变量 ===
    let label = if x % 2 == 0 { "even" } else { "odd" };
    println!("x is {}", label);

    // === Option 解构：if let 语法 ===
    let maybe_val = get_optional_value(true);

    // if let 从 Option 中解构值
    if let Some(val) = maybe_val {
        println!("捕获到值: {}", val);
    } else {
        println!("值为 None");
    }

    let no_val = get_optional_value(false);
    if let Some(val) = no_val {
        println!("捕获到值: {}", val);
    } else {
        println!("值为 None");
    }

    // === bool 运算（Rust 用 && || ） ===
    let a = true;
    let b = false;
    if a && !b {
        println!("a 为 true 且 b 为 false");
    }
    if a || b {
        println!("a 或 b 至少一个为 true");
    }
}
