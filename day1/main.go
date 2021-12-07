package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"strconv"
)

type IntIter interface {
	Next() (int, bool)
}

type SliceIter struct {
	n []int
}

func (i *SliceIter) Next() (int, bool) {
	if len(i.n) > 0 {
		v := i.n[0]
		i.n = i.n[1:]
		return v, true
	}
	return 0, false
}

func (i *SliceIter) Len() int {
	return len(i.n)
}

type SumIter struct {
	nums     []int
	ix, n, s int
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func head(it *SumIter) (int, bool) {
	nums := it.nums
	n := min(it.n, len(nums))
	if n == 0 {
		return 0, false
	}

	var s int
	for i := 0; i < n; i++ {
		s += nums[i]
	}
	it.s = s
	it.ix = n
	return s, true
}

func (i *SumIter) Next() (int, bool) {
	if i.ix == 0 {
		return head(i)
	}

	if i.ix >= len(i.nums) {
		return 0, false
	}

	i.s = i.s - i.nums[i.ix-i.n] + i.nums[i.ix]
	i.ix++
	return i.s, true
}

func sum(nums []int, n int) *SumIter {
	return &SumIter{
		nums: nums,
		n:    n,
	}
}

func readInput(src string) ([]int, error) {
	r, err := os.Open(src)
	if err != nil {
		return nil, err
	}

	var vals []int
	s := bufio.NewScanner(r)
	for s.Scan() {
		v, err := strconv.Atoi(s.Text())
		if err != nil {
			return nil, err
		}

		vals = append(vals, v)
	}

	if err := s.Err(); err != nil {
		return nil, err
	}

	return vals, nil
}

func part1(iter IntIter) int {
	c, ok := iter.Next()
	if !ok {
		return 0
	}

	count := 0
	for {
		v, ok := iter.Next()
		if !ok {
			break
		}

		if v > c {
			count++
		}

		c = v
	}

	return count
}

func ToArray(it IntIter) []int {
	var vals []int
	for {
		v, ok := it.Next()
		if !ok {
			break
		}
		vals = append(vals, v)
	}

	return vals
}

func main() {
	var src string
	flag.StringVar(&src, "input", "input.txt", "the input file")
	flag.Parse()

	nums, err := readInput(src)
	if err != nil {
		log.Panic(err)
	}

	fmt.Printf("Part 1: %d\n", part1(&SliceIter{n: nums}))

	fmt.Printf("Part 2: %d\n", part1(sum(nums, 3)))
}
