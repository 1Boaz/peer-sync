package main

import (
	"fmt"
	"os"
	"sync"
)

func read(path string, i uint, wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Println(i)
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	send(path, string(data))
}
