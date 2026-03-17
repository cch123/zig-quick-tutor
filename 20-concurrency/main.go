package main

import (
	"fmt"
	"sync"
	"time"
)

// ============================================================
// 示例 1: goroutine 基本使用
// ============================================================

func basicGoroutineDemo() {
	fmt.Println("=== goroutine 基本使用 ===")

	var wg sync.WaitGroup

	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			fmt.Printf("goroutine %d 开始工作\n", id)
			time.Sleep(100 * time.Millisecond)
			fmt.Printf("goroutine %d 完成工作\n", id)
		}(i)
	}

	wg.Wait()
	fmt.Println("所有 goroutine 已完成")
	fmt.Println()
}

// ============================================================
// 示例 2: Channel 通信
// ============================================================

func channelDemo() {
	fmt.Println("=== Channel 通信 ===")

	// 无缓冲 channel
	ch := make(chan string)

	go func() {
		ch <- "你好，来自 goroutine！"
	}()

	msg := <-ch
	fmt.Println(msg)

	// 带缓冲 channel
	buffered := make(chan int, 3)
	buffered <- 1
	buffered <- 2
	buffered <- 3

	fmt.Println(<-buffered) // 1
	fmt.Println(<-buffered) // 2
	fmt.Println(<-buffered) // 3
	fmt.Println()
}

// ============================================================
// 示例 3: Select 多路复用
// ============================================================

func selectDemo() {
	fmt.Println("=== Select 多路复用 ===")

	ch1 := make(chan string)
	ch2 := make(chan string)

	go func() {
		time.Sleep(50 * time.Millisecond)
		ch1 <- "来自 channel 1"
	}()

	go func() {
		time.Sleep(100 * time.Millisecond)
		ch2 <- "来自 channel 2"
	}()

	// 等待两个 channel
	for i := 0; i < 2; i++ {
		select {
		case msg := <-ch1:
			fmt.Println(msg)
		case msg := <-ch2:
			fmt.Println(msg)
		}
	}
	fmt.Println()
}

// ============================================================
// 示例 4: Mutex 保护共享数据
// ============================================================

type SafeCounter struct {
	mu    sync.Mutex
	count int64
}

func (c *SafeCounter) Increment() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.count++
}

func (c *SafeCounter) GetCount() int64 {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.count
}

func mutexDemo() {
	fmt.Println("=== Mutex 保护共享数据 ===")

	counter := &SafeCounter{}
	var wg sync.WaitGroup

	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < 1000; j++ {
				counter.Increment()
			}
		}()
	}

	wg.Wait()
	fmt.Printf("最终计数: %d (期望 4000)\n", counter.GetCount())
	fmt.Println()
}

// ============================================================
// 示例 5: 生产者-消费者模式 (用 channel 实现)
// ============================================================

func producerConsumerDemo() {
	fmt.Println("=== 生产者-消费者 (channel) ===")

	ch := make(chan int, 8) // 带缓冲的 channel

	// 生产者
	go func() {
		for i := 0; i < 10; i++ {
			ch <- i
			fmt.Printf("生产: %d\n", i)
		}
		close(ch) // 关闭 channel 通知消费者
	}()

	// 消费者：range 自动在 channel 关闭后退出
	for val := range ch {
		fmt.Printf("消费: %d\n", val)
	}

	fmt.Println("生产者-消费者完成")
	fmt.Println()
}

// ============================================================
// 示例 6: Worker Pool 模式
// ============================================================

func workerPoolDemo() {
	fmt.Println("=== Worker Pool ===")

	jobs := make(chan int, 8)
	var wg sync.WaitGroup

	// 启动 4 个 worker
	for w := 0; w < 4; w++ {
		wg.Add(1)
		go func(workerID int) {
			defer wg.Done()
			for job := range jobs {
				fmt.Printf("Worker %d 处理任务 %d\n", workerID, job)
				time.Sleep(50 * time.Millisecond)
			}
		}(w)
	}

	// 提交 8 个任务
	for j := 0; j < 8; j++ {
		jobs <- j
	}
	close(jobs)

	wg.Wait()
	fmt.Println("所有任务完成")
	fmt.Println()
}

func main() {
	basicGoroutineDemo()
	channelDemo()
	selectDemo()
	mutexDemo()
	producerConsumerDemo()
	workerPoolDemo()
}
