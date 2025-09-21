package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
)

type Json struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

func send(path string, file string) {
	data := Json{Path: path, Content: file}

	json, err := json.Marshal(data)
	if err != nil {
		panic(err)
	}

	fmt.Println("Sending:", string(json))
	resp, err := http.Post("http://192.168.1.117:8080/", "application/json", strings.NewReader(string(json)))
	if err != nil {
		panic(err)
	}

	fmt.Println(resp)
}
