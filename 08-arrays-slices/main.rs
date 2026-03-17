fn main() {
    // === 固定大小数组 ===
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    print!("数组: ");
    for v in &arr {
        print!("{} ", v);
    }
    println!();

    // 编译器推断长度
    let arr2 = [10, 20, 30];
    println!("数组长度: {}", arr2.len());

    // 全部初始化为相同值
    let zeros = [0i32; 5];
    print!("全零: ");
    for v in &zeros {
        print!("{} ", v);
    }
    println!();

    let threes = [3i32; 4];
    print!("全三: ");
    for v in &threes {
        print!("{} ", v);
    }
    println!();

    // === 切片（&[T] 是胖指针：指针 + 长度） ===
    let full_slice: &[i32] = &arr;
    println!("完整切片长度: {}", full_slice.len());

    // 切片操作 &arr[start..end]，左闭右开
    let sub: &[i32] = &arr[1..4];
    print!("arr[1..4]: ");
    for v in sub {
        print!("{} ", v);
    }
    println!();

    // 可变切片
    let mut mut_arr = [1, 2, 3, 4, 5];
    let mut_slice: &mut [i32] = &mut mut_arr;
    mut_slice[0] = 99;
    print!("修改后: ");
    for v in mut_slice.iter() {
        print!("{} ", v);
    }
    println!();

    // === Vec<T>：Rust 的动态数组 ===
    let mut list: Vec<i32> = Vec::new();
    list.push(10);
    list.push(20);
    list.push(30);
    list.extend_from_slice(&[40, 50]);

    print!("动态数组: ");
    for v in &list {
        print!("{} ", v);
    }
    println!("\n长度: {}, 容量: {}", list.len(), list.capacity());

    // vec! 宏快速创建
    let quick = vec![1, 2, 3, 4, 5];
    println!("vec! 宏: {:?}", quick);

    // 预分配容量
    let with_cap: Vec<i32> = Vec::with_capacity(100);
    println!("预分配: len={}, cap={}", with_cap.len(), with_cap.capacity());

    // === 切片拼接 ===
    let a = vec![1, 2];
    let b = vec![3, 4];
    let c: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    print!("拼接: ");
    for v in &c {
        print!("{} ", v);
    }
    println!();

    // 更简单的方式
    let mut d = vec![1, 2];
    d.extend_from_slice(&[3, 4]);
    println!("extend 拼接: {:?}", d);

    // === 多维数组 ===
    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("矩阵:");
    for row in &matrix {
        for val in row {
            print!("{:3} ", val);
        }
        println!();
    }

    // === 切片的实用方法（Rust 特色） ===
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    data.sort();
    println!("排序后: {:?}", data);
    println!("包含 5: {}", data.contains(&5));
    println!("窗口迭代:");
    for w in data.windows(3) {
        println!("  {:?}", w);
    }

    // === 传递切片给函数 ===
    let original = [1, 2, 3, 4, 5];
    let total = sum_slice(&original);
    println!("数组之和: {}", total);
}

fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
