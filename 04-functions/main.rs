use std::fmt;

// --- 基本函数 ---
fn add(a: i32, b: i32) -> i32 {
    a + b // 最后一个表达式作为返回值（无分号）
}

// --- 多返回值 (使用元组) ---
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

// --- 错误返回 ---
#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

// --- 函数指针 ---
fn apply_op(a: i32, b: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(a, b)
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// --- 泛型 ---
fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    // 基本函数调用
    println!("add(3, 4) = {}", add(3, 4));

    // 多返回值（元组解构）
    let (x, y) = swap(10, 20);
    println!("swap(10, 20) = {}, {}", x, y);

    // 错误处理: match
    match safe_divide(10, 3) {
        Ok(val) => println!("10 / 3 = {}", val),
        Err(e) => println!("error: {}", e),
    }

    match safe_divide(10, 0) {
        Ok(val) => println!("10 / 0 = {}", val),
        Err(e) => println!("10 / 0 = error: {}", e),
    }

    // unwrap_or 提供默认值
    let safe_val = safe_divide(10, 0).unwrap_or(0);
    println!("10 / 0 with unwrap_or = {}", safe_val);

    // 函数指针
    println!("apply_op(3, 4, multiply) = {}", apply_op(3, 4, multiply));

    // 闭包
    let double = |x: i32| -> i32 { x * 2 };
    println!("double(5) = {}", double(5));

    // 泛型
    println!("max_of(3, 7) = {}", max_of(3, 7));
    println!("max_of(1.5, 2.5) = {:.1}", max_of(1.5, 2.5));
}
