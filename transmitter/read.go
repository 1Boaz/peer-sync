package main

import (
	"os"
	"sync"
)

// / Reads a file at the given path and sends its contents to the receiver using the provided configuration.
// /
// / # Parameters
// / * `path` - The path to the file to be read
// / * `wg` - A WaitGroup to be notified when the read operation is complete
// / * `config` - The configuration struct containing the receiver's URL and passkey
// /
// / # Panics
// / * If an error occurs while reading the file, the program will panic.
// /
// / # Errors
// / * Returns an error if the file cannot be read or if the send operation fails.
func read(path string, wg *sync.WaitGroup, config Config) {
	defer wg.Done()
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	send(path, string(data), config)
}
