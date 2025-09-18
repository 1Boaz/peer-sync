package main

import (
	"fmt"
	"net/http"
	"os"
	"strings"
)

func read(filepath string) string {
	data, err := os.ReadFile(filepath)
	if err != nil {
		panic(err)
	}
	return string(data)
}

func main() {
	file := read("filename")
	resp, err := http.Post("http://<ip>:8080/", "text", strings.NewReader(file))
	if err != nil {
		panic(err)
	}
	fmt.Println(resp.Status)
}
