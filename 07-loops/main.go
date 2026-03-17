package main

import "fmt"

func main() {
	// === 基本 for 循环（当 while 用） ===
	i := 0
	for i < 5 {
		fmt.Printf("%d ", i)
		i++
	}
	fmt.Println()

	// === C 风格 for 循环 ===
	sum := 0
	for j := 0; j < 10; j++ {
		if j%2 == 0 {
			continue
		}
		sum += j
	}
	fmt.Printf("奇数之和 (1..9) = %d\n", sum)

	// === range 迭代切片 ===
	fruits := []string{"apple", "banana", "cherry"}
	for _, fruit := range fruits {
		fmt.Printf("%s ", fruit)
	}
	fmt.Println()

	// === range 带索引 ===
	for idx, fruit := range fruits {
		fmt.Printf("[%d] %s\n", idx, fruit)
	}

	// === range 迭代范围（Go 1.22+） ===
	fmt.Print("0 到 4: ")
	for n := range 5 {
		fmt.Printf("%d ", n)
	}
	fmt.Println()

	// === 同时迭代多个切片（Go 不直接支持，需手动索引） ===
	names := []string{"Alice", "Bob", "Carol"}
	ages := []int{30, 25, 28}
	for k := range names {
		fmt.Printf("%s 的年龄是 %d\n", names[k], ages[k])
	}

	// === Go 没有 break 返回值，需要额外变量 ===
	data := []int{1, 3, 7, 4, 9, 2}
	found := -1
	for idx, v := range data {
		if v > 5 {
			found = idx
			break
		}
	}
	if found >= 0 {
		fmt.Printf("第一个大于 5 的元素在索引 %d, 值为 %d\n", found, data[found])
	}

	// === 标签循环 ===
	fmt.Println("乘法表中大于 20 的第一个结果:")
	ra, rb := 0, 0
outer:
	for a := 1; a < 10; a++ {
		for b := 1; b < 10; b++ {
			if a*b > 20 {
				ra, rb = a, b
				break outer
			}
		}
	}
	fmt.Printf("%d x %d = %d\n", ra, rb, ra*rb)

	// === 无限循环 ===
	count := 0
	for {
		count++
		if count >= 3 {
			break
		}
	}
	fmt.Printf("无限循环执行了 %d 次\n", count)
}
