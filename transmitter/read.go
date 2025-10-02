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

	data, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("failed to read file %s: %w", path, err)
	}

	slog.Debug("file read successfully",
		"path", path,
		"size_bytes", len(data),
		"duration_ms", time.Since(start).Milliseconds())

	if err := send(path, string(data), "POST", conf); err != nil {
		return fmt.Errorf("failed to send file %s: %w", path, err)
	}

	slog.Info("file processed successfully",
		"path", path,
		"total_duration_ms", time.Since(start).Milliseconds())
	return nil
}
