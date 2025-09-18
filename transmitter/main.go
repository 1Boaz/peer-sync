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
	filepath := "main.go"
	file := read(filepath)
	json := fmt.Sprintf(`{"path":%s,"content":%s}`, filepath, file)
	resp, err := http.Post("http://192.168.1.117:8080/", "application/json", strings.NewReader(json))
	if err != nil {
		panic(err)
	}
	fmt.Println(resp.Status)
}
