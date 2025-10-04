package main

import (
	"fmt"
	"log/slog"
	"os"
	"time"
)

// readAndSend reads a file and sends its contents to the receiver.
// It handles errors gracefully and provides detailed logging.
func readAndSend(path string, conf Config) error {
	start := time.Now()
	slog.Info("reading file", "path", path)

	var data string
	var err error

	maxRetries := 5
	for attempt := 1; attempt <= maxRetries; attempt++ {
		data, err = readFile(path)
		if err != nil {
			slog.Warn("failed to read file",
				"path", path,
				"attempt", attempt,
				"error", err,
			)
		} else if len(data) == 0 {
			slog.Warn("file is empty",
				"path", path,
				"attempt", attempt,
			)
		} else {
			break
		}

		time.Sleep(time.Duration(attempt) * 500 * time.Millisecond)
	}

	if err != nil {
		return fmt.Errorf("failed to read file %s after %d attempts: %w", path, maxRetries, err)
	}
	if len(data) == 0 {
		return fmt.Errorf("file %s is empty after %d attempts", path, maxRetries)
	}

	slog.Debug("file read successfully",
		"path", path,
		"size_bytes", len(data),
		"duration_ms", time.Since(start).Milliseconds())

	if err := send(path, data, "POST", conf); err != nil {
		return fmt.Errorf("failed to send file %s: %w", path, err)
	}

	slog.Info("file processed successfully",
		"path", path,
		"total_duration_ms", time.Since(start).Milliseconds())
	return nil
}

func readFile(path string) (string, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("failed to read file %s: %w", path, err)
	}
	return string(data), nil
}
