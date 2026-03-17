use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    // ============================================================
    // 1. 栈分配 —— 默认行为
    // ============================================================
    println!("=== stack allocation ===");
    let x: i32 = 42;
    let arr: [u8; 10] = [0; 10];
    println!("stack x = {}, arr len = {}", x, arr.len());
    // 离开作用域自动释放，零开销

    // ============================================================
    // 2. Box<T> —— 堆分配的智能指针
    // ============================================================
    println!("\n=== Box<T> (heap allocation) ===");
    let boxed = Box::new(42);
    println!("boxed value: {}", boxed);

    // Box 常用于递归类型
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("list: {:?}", list);
    // Box 离开作用域时自动释放堆内存（Drop trait）

    // ============================================================
    // 3. 所有权转移（move）
    // ============================================================
    println!("\n=== ownership (move) ===");
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权转移给 s2
    // println!("{}", s1); // 编译错误！s1 已经无效
    println!("s2 = {}", s2);

    // 借用 —— 不转移所有权
    let s3 = String::from("world");
    let len = calculate_length(&s3); // 借用，不转移所有权
    println!("'{}' has length {}", s3, len); // s3 仍然有效

    // ============================================================
    // 4. Rc<T> —— 引用计数（单线程）
    // ============================================================
    println!("\n=== Rc<T> (reference counting) ===");
    let shared = Rc::new(String::from("shared data"));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);
    println!("ref count: {}", Rc::strong_count(&shared));
    println!("clone1: {}, clone2: {}", clone1, clone2);
    drop(clone1);
    println!("ref count after drop: {}", Rc::strong_count(&shared));

    // ============================================================
    // 5. Arc<T> —— 原子引用计数（多线程安全）
    // ============================================================
    println!("\n=== Arc<T> (atomic reference counting) ===");
    let arc_data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&arc_data);
        let handle = thread::spawn(move || {
            let sum: i32 = data.iter().sum();
            println!("thread {}: sum = {}", i, sum);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // ============================================================
    // 6. Vec<T> —— 动态数组（自动管理堆内存）
    // ============================================================
    println!("\n=== Vec<T> (dynamic array) ===");
    let mut v = Vec::with_capacity(10); // 预分配容量
    for i in 0..5 {
        v.push(i * 10);
    }
    println!("vec: {:?}, len: {}, capacity: {}", v, v.len(), v.capacity());
    // Vec 离开作用域时自动释放

    // ============================================================
    // 7. 内存操作 —— copy / fill
    // ============================================================
    println!("\n=== memory operations ===");

    // 等价于 memcpy
    let src = vec![1u8, 2, 3, 4, 5];
    let mut dst = vec![0u8; 5];
    dst.copy_from_slice(&src);
    println!("copied: {:?}", dst);

    // 等价于 memset
    let mut buf = vec![0u8; 10];
    buf.fill(b'A');
    println!("filled: {}", String::from_utf8_lossy(&buf));

    // ptr::copy (unsafe，底层操作)
    let mut data = [0u8; 5];
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), data.as_mut_ptr(), 5);
    }
    println!("ptr::copy: {:?}", data);

    // ============================================================
    // 8. RAII —— Drop trait 自动清理
    // ============================================================
    println!("\n=== RAII (Drop trait) ===");
    {
        let _resource = Resource::new("my_resource");
        println!("using resource...");
        // resource 在这里离开作用域，自动调用 drop()
    }
    println!("after scope");

    // Rust 的内存管理哲学：
    // - 所有权系统在编译期保证内存安全，无 GC
    // - 默认栈分配，堆分配通过 Box/Vec/String 等智能指针
    // - RAII：资源获取即初始化，离开作用域自动释放
    // - Rc/Arc 用于共享所有权的场景
    // - unsafe 块中可以进行底层内存操作
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

struct Resource {
    name: String,
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("Resource '{}' created", name);
        Resource {
            name: name.to_string(),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource '{}' dropped (auto cleanup)", self.name);
    }
}
