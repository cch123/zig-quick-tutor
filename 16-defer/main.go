package main

import "fmt"

// 模拟资源
type Logger struct {
	Name string
}

func NewLogger(name string) *Logger {
	fmt.Printf("[%s] opened\n", name)
	return &Logger{Name: name}
}

func (l *Logger) Write(msg string) {
	fmt.Printf("[%s] %s\n", l.Name, msg)
}

func (l *Logger) Close() {
	fmt.Printf("[%s] closed\n", l.Name)
}

// 1. 基本 defer
func basicDefer() {
	fmt.Println("--- basicDefer ---")
	logger := NewLogger("basic")
	defer logger.Close() // 函数返回时关闭

	logger.Write("doing work...")
	logger.Write("more work...")
}

// 2. 多个 defer 按 LIFO 顺序执行
func multipleDefers() {
	fmt.Println("\n--- multipleDefers ---")
	defer fmt.Println("first defer (executes last)")
	defer fmt.Println("second defer (executes second)")
	defer fmt.Println("third defer (executes first)")
	fmt.Println("main body")
}

// 3. Go 的 defer 是函数级的（重要区别！）
func functionLevelDefer() {
	fmt.Println("\n--- functionLevelDefer ---")

	// 危险！所有 defer 都在函数返回时才执行
	// 如果循环次数多，会积累大量未释放的资源
	for i := 0; i < 3; i++ {
		logger := NewLogger(fmt.Sprintf("loop-%d", i))
		defer logger.Close() // 在函数返回时才执行，不是循环结束时！
		logger.Write("iteration work")
	}
	fmt.Println("After loop (defers haven't run yet!)")
	// 所有 Close 在此之后执行，顺序是 loop-2, loop-1, loop-0
}

// 4. 解决方案：用闭包创建新的函数作用域
func scopedDeferWorkaround() {
	fmt.Println("\n--- scopedDeferWorkaround ---")

	for i := 0; i < 3; i++ {
		func() {
			logger := NewLogger(fmt.Sprintf("scoped-%d", i))
			defer logger.Close() // 在匿名函数返回时执行
			logger.Write("iteration work")
		}()
	}
	fmt.Println("After loop (all loggers already closed)")
}

// 5. defer 参数在声明时求值
func deferEvaluation() {
	fmt.Println("\n--- deferEvaluation ---")

	x := 10
	defer fmt.Println("deferred x =", x) // x=10 在此时就确定了
	x = 20
	fmt.Println("current x =", x) // 20
	// defer 输出的是 10，不是 20！
}

// 6. defer 与 panic/recover
func safeOperation() {
	fmt.Println("\n--- safeOperation ---")

	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered from panic:", r)
		}
	}()

	fmt.Println("Before panic")
	panic("something went wrong")
	// 下面的代码不会执行
}

// 7. Go 没有 errdefer，需要手动处理
func riskyOperation(shouldFail bool) (string, error) {
	fmt.Printf("\n--- riskyOperation(fail=%v) ---\n", shouldFail)

	logger := NewLogger("risky")
	// Go 没有 errdefer，必须无条件 defer 或手动管理
	// 方式1：无条件关闭（即使成功也关闭）
	// defer logger.Close()

	// 方式2：根据错误状态决定（更灵活但更啰嗦）
	var err error
	defer func() {
		if err != nil {
			fmt.Println("Cleaning up due to error")
			logger.Close()
		}
	}()

	logger.Write("doing risky work")
	if shouldFail {
		err = fmt.Errorf("operation failed")
		return "", err
	}
	logger.Write("success!")
	return "result", nil
}

func main() {
	basicDefer()
	multipleDefers()
	functionLevelDefer()
	scopedDeferWorkaround()
	deferEvaluation()
	safeOperation()
	riskyOperation(true)
	riskyOperation(false)
}
