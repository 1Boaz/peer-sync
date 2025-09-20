package main

import (
	"fmt"
	"net/http"
	"strings"
)

func send(filepath string, file string) *http.Response {
	json := fmt.Sprintf(`{"path":"%s","content":"%s"}`, filepath, file)
	fmt.Println("Sending:", json)
	resp, err := http.Post("http://192.168.1.119:8080/", "application/json", strings.NewReader(json))
	if err != nil {
		panic(err)
	}
	return resp
}
