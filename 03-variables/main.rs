fn main() {
    // --- 变量声明 ---
    // let mut: 可变变量
    let mut x: i32 = 10;
    x += 5;
    println!("let mut x = {}", x);

    // let: 不可变绑定（默认）
    let y: i32 = 42;
    println!("let y = {}", y);

    // 类型推断
    let z = 100i32;
    println!("inferred z = {}", z);

    // --- 常量 ---
    // const 必须标注类型，值在编译期确定
    const FACTORIAL_5: i32 = 1 * 2 * 3 * 4 * 5;
    println!("const 5! = {}", FACTORIAL_5);

    println!("PI = {:.6}", PI);

    // --- Rust 允许 Shadowing ---
    let a = 10;
    println!("a = {}", a);
    let a = 20; // 遮蔽了前一个 a
    println!("shadowed a = {}", a);

    // shadowing 甚至可以改变类型
    let a = "now I'm a string";
    println!("rebound a = {}", a);

    // --- Rust 必须初始化 ---
    // 以下代码无法编译:
    // let uninit: i32;
    // println!("{}", uninit);  // 编译错误！

    // 但可以延迟初始化，只要使用前一定赋值
    let delayed: i32;
    delayed = 99;
    println!("delayed init = {}", delayed);

    // --- 块作用域 ---
    {
        let scoped = 123;
        println!("scoped = {}", scoped);
    }
    // scoped 在这里不可见
}

const PI: f64 = 3.141592653589793;
