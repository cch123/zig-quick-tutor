use std::fmt;
use std::num::ParseIntError;

// 1. 自定义错误类型
#[derive(Debug)]
enum AppError {
    NotFound(String),
    PermissionDenied,
    DivisionByZero,
    ParseError(ParseIntError),
}

// 实现 Display trait
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(path) => write!(f, "not found: {}", path),
            AppError::PermissionDenied => write!(f, "permission denied"),
            AppError::DivisionByZero => write!(f, "division by zero"),
            AppError::ParseError(e) => write!(f, "parse error: {}", e),
        }
    }
}

// 2. From trait 实现自动错误转换
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

// 3. Result<T, E> 返回类型
fn divide(a: f64, b: f64) -> Result<f64, AppError> {
    if b == 0.0 {
        return Err(AppError::DivisionByZero);
    }
    Ok(a / b)
}

fn read_config(path: &str) -> Result<String, AppError> {
    if path == "missing.txt" {
        return Err(AppError::NotFound(path.to_string()));
    }
    if path == "secret.txt" {
        return Err(AppError::PermissionDenied);
    }
    Ok("config_data".to_string())
}

// 4. ? 运算符 - 自动传播错误（类似 Zig 的 try）
fn load_and_process(path: &str) -> Result<String, AppError> {
    let _data = read_config(path)?; // 错误时自动返回 Err
    Ok("processed_data".to_string())
}

// 5. ? 运算符 + From trait 自动转换
fn parse_and_double(s: &str) -> Result<i32, AppError> {
    let num: i32 = s.parse()?; // ParseIntError 自动转换为 AppError
    Ok(num * 2)
}

fn main() {
    // 1. 基本错误处理 - match
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e) => println!("Error: {}", e),
    }

    // 2. 处理除零
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("10 / 0 error: {}", e),
    }

    // 3. unwrap_or - 提供默认值（类似 Zig 的 catch）
    let safe_result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("10 / 0 (with default) = {:.2}", safe_result);

    // 4. if let - 只关心成功情况
    if let Ok(config) = read_config("normal.txt") {
        println!("Good config: {}", config);
    }

    // 5. 错误匹配
    match read_config("missing.txt") {
        Ok(data) => println!("Data: {}", data),
        Err(AppError::NotFound(path)) => println!("File {} not found", path),
        Err(AppError::PermissionDenied) => println!("Permission denied!"),
        Err(e) => println!("Other error: {}", e),
    }

    // 6. ? 运算符传播（在 main 中演示需要用 match 包裹）
    match load_and_process("missing.txt") {
        Ok(data) => println!("Processed: {}", data),
        Err(e) => println!("load_and_process failed: {}", e),
    }

    // 7. From trait 自动错误转换
    match parse_and_double("42") {
        Ok(v) => println!("Parsed and doubled: {}", v),
        Err(e) => println!("Error: {}", e),
    }
    match parse_and_double("abc") {
        Ok(v) => println!("Parsed: {}", v),
        Err(e) => println!("Parse error: {}", e),
    }

    // 8. map / and_then 组合子
    let result = divide(10.0, 2.0)
        .map(|v| v as i32)
        .map(|v| v * 3);
    println!("Chained result: {:?}", result); // Ok(15)
}
