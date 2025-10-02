package main

import (
	"bytes"
	"compress/gzip"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"time"
)

type Json struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// var slog = logr.Discard()
// send sends file data to the receiver with proper error handling and logging.
// It compresses the data with gzip and includes appropriate headers.
func send(path, content string, method string, config Config) error {
	start := time.Now()
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// Log the start of the send operation
	slog.Info("sending request",
		"method", method,
		"path", path,
		"content_length", len(content))

	// Prepare the request data
	data := Json{Path: path, Content: content}
	jsonData, err := json.Marshal(data)
	if err != nil {
		return fmt.Errorf("failed to marshal JSON: %w", err)
	}

	// Compress the data
	var gzipped bytes.Buffer
	gz := gzip.NewWriter(&gzipped)
	if _, err := gz.Write(jsonData); err != nil {
		return fmt.Errorf("failed to compress data: %w", err)
	}
	if err := gz.Close(); err != nil {
		return fmt.Errorf("failed to close gzip writer: %w", err)
	}

	// Create the request
	req, err := http.NewRequestWithContext(ctx, method, config.Url, &gzipped)
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Content-Encoding", "gzip")
	req.Header.Set("Authorization", config.Key)

	// Configure HTTP client with timeouts
	client := &http.Client{
		Timeout: 30 * time.Second,
		Transport: &http.Transport{
			MaxIdleConns:        10,
			IdleConnTimeout:     30 * time.Second,
			DisableCompression:  true,
			MaxIdleConnsPerHost: 10,
		},
	}

	// Send the request
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("request failed: %w", err)
	}
	defer func() {
		if err := resp.Body.Close(); err != nil {
			slog.Error("failed to close response body", "error", err)
		}
	}()

	// Read the response body for error details
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("failed to read response body: %w", err)
	}

	// Log the response
	slog.Info("request completed",
		"method", method,
		"path", path,
		"status", resp.Status,
		"status_code", resp.StatusCode,
		"duration_ms", time.Since(start).Milliseconds(),
		"response_size", len(body))

	return nil
}
