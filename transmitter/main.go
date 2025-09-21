package main

import (
	"sync"
)

func main() {
	paths := []string{"main.go", "read.go", "send.go"}
	var wg sync.WaitGroup
	for _, path := range paths {
		wg.Add(1)
		go read(path, &wg)
	}
	wg.Wait()
}
