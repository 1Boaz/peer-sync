package main

import (
	"os"
	"sync"
)

func read(path string, wg *sync.WaitGroup, config Config) {
	defer wg.Done()
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	send(path, string(data), config)
}
