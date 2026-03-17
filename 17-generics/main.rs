use std::fmt;

// 1. 泛型函数 + trait bound
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 2. 泛型容器：栈
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
}

// 3. trait bound 约束 —— 要求类型实现特定 trait
fn double<T: std::ops::Mul<Output = T> + From<u8>>(x: T) -> T {
    x * T::from(2u8)
}

// 4. where 子句 —— 复杂约束的可读写法
fn print_max<T>(a: T, b: T)
where
    T: PartialOrd + fmt::Display,
{
    if a > b {
        println!("max = {}", a);
    } else {
        println!("max = {}", b);
    }
}

// 5. 泛型 map 函数
fn map<T, U, F>(slice: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    slice.iter().map(f).collect()
}

// 6. const fn —— Rust 的编译期计算（功能有限）
const fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    // const fn 中可以用基本控制流，但不能用迭代器等
    fibonacci(n - 1) + fibonacci(n - 2)
}

// 编译期常量
const FIB10: u32 = fibonacci(10);

// 7. 编译期查找表
const SQUARES: [usize; 10] = {
    let mut table = [0usize; 10];
    let mut i = 0;
    while i < 10 {
        table[i] = i * i;
        i += 1;
    }
    table
};

fn main() {
    // 1. 泛型 max
    println!("max(i32): {}", max(10, 20));
    println!("max(f64): {:.1}", max(3.14, 2.71));

    // 2. 泛型 double
    println!("double(i32): {}", double(21i32));
    println!("double(f64): {:.1}", double(1.5f64));

    // 3. 泛型栈
    let mut int_stack = Stack::new();
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);
    println!("peek: {:?}", int_stack.peek());
    println!("pop: {:?}", int_stack.pop());
    println!("pop: {:?}", int_stack.pop());

    let mut str_stack: Stack<&str> = Stack::new();
    str_stack.push("hello");
    str_stack.push("world");
    println!("string pop: {:?}", str_stack.pop());

    // 4. where 子句
    print_max(42, 17);
    print_max(3.14, 2.71);

    // 5. 泛型高阶函数
    let nums = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = map(&nums, |x| x * x);
    println!("squares: {:?}", squares);

    let strs: Vec<String> = map(&nums, |x| format!("item_{}", x));
    println!("mapped to strings: {:?}", strs);

    // 6. 编译期计算
    println!("fib(10) = {} (computed at compile time)", FIB10);

    // 7. 编译期查找表
    println!("squares[7] = {}", SQUARES[7]);

    // Rust 的泛型通过单态化实现零开销抽象
    // 编译器为每个具体类型生成专用代码
}
