package main

import "fmt"

func main() {
	// === 固定大小数组 ===
	arr := [5]int{1, 2, 3, 4, 5}
	fmt.Print("数组: ")
	for _, v := range arr {
		fmt.Printf("%d ", v)
	}
	fmt.Println()

	// 编译器推断长度
	arr2 := [...]int{10, 20, 30}
	fmt.Printf("数组长度: %d\n", len(arr2))

	// 全部初始化为零值（Go 自动零值初始化）
	var zeros [5]int
	fmt.Print("全零: ")
	for _, v := range zeros {
		fmt.Printf("%d ", v)
	}
	fmt.Println()

	// 指定索引初始化
	indexed := [5]int{0: 10, 3: 40}
	fmt.Print("指定索引: ")
	for _, v := range indexed {
		fmt.Printf("%d ", v)
	}
	fmt.Println()

	// === 切片（ptr + len + cap 三元组） ===
	fullSlice := arr[:]
	fmt.Printf("完整切片长度: %d, 容量: %d\n", len(fullSlice), cap(fullSlice))

	// 切片操作 arr[start:end]，左闭右开
	sub := arr[1:4]
	fmt.Print("arr[1:4]: ")
	for _, v := range sub {
		fmt.Printf("%d ", v)
	}
	fmt.Println()

	// 切片是引用，修改会影响原数组
	mutSlice := arr[:]
	mutSlice[0] = 99
	fmt.Print("修改后: ")
	for _, v := range mutSlice {
		fmt.Printf("%d ", v)
	}
	fmt.Println()

	// === make 创建切片 ===
	s := make([]int, 3, 10) // 长度 3，容量 10
	fmt.Printf("make 切片: len=%d, cap=%d\n", len(s), cap(s))

	// === append 追加 ===
	list := make([]int, 0)
	list = append(list, 10)
	list = append(list, 20)
	list = append(list, 30)
	list = append(list, 40, 50)

	fmt.Print("动态切片: ")
	for _, v := range list {
		fmt.Printf("%d ", v)
	}
	fmt.Printf("\n长度: %d, 容量: %d\n", len(list), cap(list))

	// === 三索引切片（控制容量） ===
	base := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	limited := base[2:5:6] // 从索引2到5，容量限制到6
	fmt.Printf("三索引切片: len=%d, cap=%d, %v\n", len(limited), cap(limited), limited)

	// === 切片拼接 ===
	a := []int{1, 2}
	b := []int{3, 4}
	c := append(a, b...) // ... 展开切片
	fmt.Print("拼接: ")
	for _, v := range c {
		fmt.Printf("%d ", v)
	}
	fmt.Println()

	// === 多维数组 ===
	matrix := [3][3]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}
	fmt.Println("矩阵:")
	for _, row := range matrix {
		for _, val := range row {
			fmt.Printf("%3d ", val)
		}
		fmt.Println()
	}

	// === 传递切片给函数 ===
	original := [5]int{1, 2, 3, 4, 5}
	total := sumSlice(original[:])
	fmt.Printf("数组之和: %d\n", total)
}

func sumSlice(slice []int) int {
	total := 0
	for _, v := range slice {
		total += v
	}
	return total
}
