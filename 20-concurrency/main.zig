const std = @import("std");

// ============================================================
// 示例 1: 基本线程创建
// ============================================================

fn workerFunction(id: usize) void {
    std.debug.print("线程 {d} 开始工作\n", .{id});
    // 模拟一些工作
    std.Thread.sleep(100 * std.time.ns_per_ms);
    std.debug.print("线程 {d} 完成工作\n", .{id});
}

fn basicThreadDemo() !void {
    std.debug.print("=== 基本线程创建 ===\n", .{});

    // 创建多个线程
    var threads: [4]std.Thread = undefined;
    for (0..4) |i| {
        threads[i] = try std.Thread.spawn(.{}, workerFunction, .{i});
    }

    // 等待所有线程完成 (join)
    for (&threads) |*t| {
        t.join();
    }
    std.debug.print("所有线程已完成\n\n", .{});
}

// ============================================================
// 示例 2: 互斥锁 (Mutex) 保护共享数据
// ============================================================

const SharedCounter = struct {
    mutex: std.Thread.Mutex = .{},
    count: i64 = 0,

    fn increment(self: *SharedCounter) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.count += 1;
    }

    fn getCount(self: *SharedCounter) i64 {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.count;
    }
};

fn counterWorker(counter: *SharedCounter) void {
    for (0..1000) |_| {
        counter.increment();
    }
}

fn mutexDemo() !void {
    std.debug.print("=== 互斥锁 (Mutex) ===\n", .{});

    var counter = SharedCounter{};

    // 启动 4 个线程，各自增加计数器 1000 次
    var threads: [4]std.Thread = undefined;
    for (0..4) |i| {
        threads[i] = try std.Thread.spawn(.{}, counterWorker, .{&counter});
    }

    for (&threads) |*t| {
        t.join();
    }

    std.debug.print("最终计数: {d} (期望 4000)\n\n", .{counter.getCount()});
}

// ============================================================
// 示例 3: 条件变量 (Condition Variable)
// ============================================================

const BoundedQueue = struct {
    mutex: std.Thread.Mutex = .{},
    not_empty: std.Thread.Condition = .{},
    not_full: std.Thread.Condition = .{},
    buffer: [8]i32 = undefined,
    count: usize = 0,
    head: usize = 0,
    tail: usize = 0,

    fn put(self: *BoundedQueue, value: i32) void {
        self.mutex.lock();
        defer self.mutex.unlock();

        // 等待队列不满
        while (self.count == self.buffer.len) {
            self.not_full.wait(&self.mutex);
        }

        self.buffer[self.tail] = value;
        self.tail = (self.tail + 1) % self.buffer.len;
        self.count += 1;

        // 通知消费者
        self.not_empty.signal();
    }

    fn get(self: *BoundedQueue) i32 {
        self.mutex.lock();
        defer self.mutex.unlock();

        // 等待队列非空
        while (self.count == 0) {
            self.not_empty.wait(&self.mutex);
        }

        const value = self.buffer[self.head];
        self.head = (self.head + 1) % self.buffer.len;
        self.count -= 1;

        // 通知生产者
        self.not_full.signal();

        return value;
    }
};

fn producer(queue: *BoundedQueue) void {
    for (0..10) |i| {
        const val: i32 = @intCast(i);
        queue.put(val);
        std.debug.print("生产: {d}\n", .{val});
    }
}

fn consumer(queue: *BoundedQueue) void {
    for (0..10) |_| {
        const val = queue.get();
        std.debug.print("消费: {d}\n", .{val});
    }
}

fn conditionDemo() !void {
    std.debug.print("=== 条件变量 (生产者-消费者) ===\n", .{});

    var queue = BoundedQueue{};

    const prod = try std.Thread.spawn(.{}, producer, .{&queue});
    const cons = try std.Thread.spawn(.{}, consumer, .{&queue});

    prod.join();
    cons.join();

    std.debug.print("生产者-消费者完成\n\n", .{});
}

// ============================================================
// 示例 4: 线程池 (Thread Pool)
// ============================================================

fn poolDemo() !void {
    std.debug.print("=== 线程池 (Thread Pool) ===\n", .{});

    var pool: std.Thread.Pool = undefined;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    try pool.init(.{
        .allocator = gpa.allocator(),
        .n_jobs = 4,
    });
    defer pool.deinit();

    // 向线程池提交任务
    for (0..8) |i| {
        try pool.spawn(poolTask, .{i});
    }

    // deinit 会等待所有任务完成
    std.debug.print("所有任务已提交，等待完成...\n\n", .{});
}

fn poolTask(id: usize) void {
    std.debug.print("线程池任务 {d} 在执行\n", .{id});
    std.Thread.sleep(50 * std.time.ns_per_ms);
}

// ============================================================

pub fn main() !void {
    try basicThreadDemo();
    try mutexDemo();
    try conditionDemo();
    try poolDemo();
}
