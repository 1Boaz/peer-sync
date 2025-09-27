package main

import (
	"os"
	"sync"
)

// Reads a file at the given path and sends its contents to the receiver using the provided configuration.
//
// # Parameters
// * `path` - The path to the file to be read
// * `wg` - A WaitGroup to be notified when the read operation is complete
// * `config` - The configuration struct containing the receiver's URL and passkey
//
// # Panics
// * This function will panic if the file cannot be read or if the send operation fails.
func read(path string, wg *sync.WaitGroup, config Config) {
	defer wg.Done()
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	send(path, string(data), "POST", config)
}
