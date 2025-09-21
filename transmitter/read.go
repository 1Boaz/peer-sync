package main

import (
	"os"
)

func read(paths []string) {
	for _, path := range paths {
		data, err := os.ReadFile(path)
		if err != nil {
			panic(err)
		}
		send(path, string(data))
	}
}
