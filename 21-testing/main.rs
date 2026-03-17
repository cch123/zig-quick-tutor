// ============================================================
// 被测代码
// ============================================================

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.items.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

// 数学函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn factorial(n: u32) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut result: u64 = 1;
    for i in 1..=n {
        result *= i as u64;
    }
    result
}

fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
}

fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

fn main() {
    println!("这是正常的 main 函数。");
    println!("运行 `rustc --test main.rs -o test_main && ./test_main` 来执行测试。");
}

// ============================================================
// 测试模块 - 只在 `cargo test` 或 `rustc --test` 时编译
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // 基本测试
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(-1, -1), -2);
    }

    // 表驱动测试风格
    #[test]
    fn test_factorial() {
        let cases = vec![
            (0, 1u64),
            (1, 1),
            (5, 120),
            (10, 3628800),
        ];

        for (input, expected) in cases {
            assert_eq!(
                factorial(input),
                expected,
                "factorial({}) should be {}",
                input,
                expected
            );
        }
    }

    #[test]
    fn test_fibonacci() {
        let cases = vec![(0, 0u64), (1, 1), (2, 1), (10, 55), (20, 6765)];

        for (input, expected) in cases {
            assert_eq!(fibonacci(input), expected);
        }
    }

    // Stack 测试
    #[test]
    fn test_stack_empty() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.size(), 1);
    }

    // 错误测试
    #[test]
    fn test_safe_divide_ok() {
        assert_eq!(safe_divide(10, 2), Ok(5));
    }

    #[test]
    fn test_safe_divide_by_zero() {
        assert_eq!(safe_divide(10, 0), Err(MathError::DivisionByZero));
    }

    // 使用 #[should_panic] 测试 panic
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[5]; // 这会 panic
    }

    // 使用 #[ignore] 跳过测试（运行 `cargo test -- --ignored` 来执行）
    #[test]
    #[ignore]
    fn test_slow_operation() {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
