package main

import (
	"fmt"
)

func main() {
	path := ""
	file := read(path)
	resp := send(path, file)
	fmt.Println(resp.Status)
}
