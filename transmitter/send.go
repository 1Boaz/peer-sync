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

func send(path string, file string, config Config) {
	data := Json{Path: path, Content: file}

	json, err := json.Marshal(data)
	if err != nil {
		panic(err)
	}

	fmt.Println("Sending:", string(json))
	req, err := http.NewRequest("Post", config.Url, strings.NewReader(string(json)))
	if err != nil {
		panic(err)
	}
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", config.Key)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}

	fmt.Println(resp)
}
