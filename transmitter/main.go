package main

import (
	"sync"
)

func main() {
	paths := []string{"main.go", "read.go", "send.go"}
	var wg sync.WaitGroup
	for i, path := range paths {
		wg.Add(1)
		go read(path, uint(i), &wg)
	}
	wg.Wait()
}
