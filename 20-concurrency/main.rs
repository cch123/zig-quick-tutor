use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

// ============================================================
// 示例 1: 基本线程创建
// ============================================================

fn basic_thread_demo() {
    println!("=== 基本线程创建 ===");

    let mut handles = vec![];

    for i in 0..4 {
        let handle = thread::spawn(move || {
            println!("线程 {} 开始工作", i);
            thread::sleep(Duration::from_millis(100));
            println!("线程 {} 完成工作", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("所有线程已完成\n");
}

// ============================================================
// 示例 2: Channel 通信 (mpsc)
// ============================================================

fn channel_demo() {
    println!("=== Channel (mpsc) ===");

    // mpsc: multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("你好，来自子线程！".to_string()).unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("{}", msg);

    // 多个生产者
    let (tx, rx) = mpsc::channel();
    for i in 0..4 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(format!("消息来自线程 {}", i)).unwrap();
        });
    }
    drop(tx); // 丢弃原始发送端

    // 接收所有消息
    for msg in rx {
        println!("{}", msg);
    }
    println!();
}

// ============================================================
// 示例 3: Mutex 保护共享数据
// ============================================================

fn mutex_demo() {
    println!("=== Arc<Mutex<T>> 共享数据 ===");

    // Arc: 原子引用计数（跨线程共享所有权）
    // Mutex: 互斥锁
    let counter = Arc::new(Mutex::new(0i64));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut count = counter.lock().unwrap();
                *count += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {} (期望 4000)\n", *counter.lock().unwrap());
}

// ============================================================
// 示例 4: 条件变量 (生产者-消费者)
// ============================================================

fn condvar_demo() {
    println!("=== 条件变量 (生产者-消费者) ===");

    let queue = Arc::new((Mutex::new(Vec::<i32>::new()), Condvar::new()));
    let done = Arc::new(Mutex::new(false));

    // 生产者
    let queue_p = Arc::clone(&queue);
    let done_p = Arc::clone(&done);
    let producer = thread::spawn(move || {
        let (lock, cvar) = &*queue_p;
        for i in 0..10 {
            {
                let mut q = lock.lock().unwrap();
                q.push(i);
                println!("生产: {}", i);
            }
            cvar.notify_one();
        }
        // 标记完成
        *done_p.lock().unwrap() = true;
        cvar.notify_one();
    });

    // 消费者
    let queue_c = Arc::clone(&queue);
    let done_c = Arc::clone(&done);
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*queue_c;
        let mut consumed = 0;
        while consumed < 10 {
            let mut q = lock.lock().unwrap();
            while q.is_empty() {
                if *done_c.lock().unwrap() && q.is_empty() {
                    return;
                }
                q = cvar.wait(q).unwrap();
            }
            while let Some(val) = q.pop() {
                println!("消费: {}", val);
                consumed += 1;
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
    println!("生产者-消费者完成\n");
}

// ============================================================
// 示例 5: 简单线程池 (用 channel 实现)
// ============================================================

fn thread_pool_demo() {
    println!("=== 简单线程池 ===");

    let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
    let rx = Arc::new(Mutex::new(rx));
    let mut workers = vec![];

    // 启动 4 个 worker 线程
    for id in 0..4 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                let task = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };
                match task {
                    Ok(task) => {
                        println!("Worker {} 执行任务", id);
                        task();
                    }
                    Err(_) => break, // channel 关闭
                }
            }
        });
        workers.push(handle);
    }

    // 提交 8 个任务
    for i in 0..8 {
        tx.send(Box::new(move || {
            println!("  任务 {} 完成", i);
            thread::sleep(Duration::from_millis(50));
        }))
        .unwrap();
    }

    drop(tx); // 关闭 channel，worker 会退出循环

    for w in workers {
        w.join().unwrap();
    }
    println!("所有任务完成\n");
}

fn main() {
    basic_thread_demo();
    channel_demo();
    mutex_demo();
    condvar_demo();
    thread_pool_demo();
}
