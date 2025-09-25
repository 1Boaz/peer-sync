package main

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type Json struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

func send(path string, file string, config Config) {
	data := Json{Path: path, Content: file}
	jsonData, err := json.Marshal(data)
	if err != nil {
		log.Fatal(err)
	}

	var gziped bytes.Buffer
	gz := gzip.NewWriter(&gziped)

	if _, err := gz.Write(jsonData); err != nil {
		log.Fatal(err)
	}

	if err := gz.Close(); err != nil {
		log.Fatal(err)
	}

	req, err := http.NewRequest("POST", config.Url, &gziped)
	if err != nil {
		log.Fatal(err)
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Content-Encoding", "gzip")
	req.Header.Set("Authorization", config.Key)

	client := &http.Client{}

	resp, err := client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	fmt.Println("Response status:", resp.Status)
}
