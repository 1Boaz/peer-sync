package main

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"log/slog"
	"os"
	"os/signal"
	"path/filepath"
	"syscall"
	"time"

	"github.com/fsnotify/fsnotify"
)

var logger *slog.Logger

// init initializes the structured logger with a JSON handler and sets the default logger.
// The logger level is set to Debug.
func init() {
	// Initialize structured logger
	handler := slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
		Level: slog.LevelDebug,
	})
	logger = slog.New(handler)
	slog.SetDefault(logger)
}

// main is the entry point of the transmitter.
// It sets up a context with cancellation and a signal handler for graceful shutdown.
// The signal handler listens for SIGINT and SIGTERM and cancels the context when either signal is received.
// After setting up the context and signal handler, main calls run to start the file watcher and send files to the receiver.
// If run returns an error, main logs the error and exits with a status code of 1.
func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Handle graceful shutdown
	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		sig := <-sigCh
		slog.Info("shutting down", "signal", sig.String())
		cancel()
	}()

	if err := run(ctx); err != nil {
		slog.Error("application error", "error", err)
		os.Exit(1)
	}
}

// run starts the file watcher and sends files to the receiver.
// It first reads the configuration from the -C flag and logs the configuration.
// Then, it creates a new watcher and adds all paths from the configuration to watch.
// The event listener is started in a goroutine and the error channel is used to report any watcher errors.
// Finally, run waits for context cancellation or an error from the watcher and returns the error if any.
func run(ctx context.Context) error {
	conf := getConfig()
	slog.Info("starting file watcher", "paths", conf.Paths, "url", conf.Url)

	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return fmt.Errorf("failed to create watcher: %w", err)
	}
	defer watcher.Close()

	// Start the event listener in a goroutine
	errCh := make(chan error, 1)
	go func() {
		if err := listen(ctx, conf, watcher); err != nil && !errors.Is(err, context.Canceled) {
			errCh <- fmt.Errorf("watcher error: %w", err)
		}
		close(errCh)
	}()

	// Add paths and their subdirectories to watch
	for _, path := range conf.Paths {
		if err := addPathRecursively(watcher, path); err != nil {
			return fmt.Errorf("failed to watch path %s: %w", path, err)
		}
		slog.Debug("watching path", "path", path)
	}

	// Wait for context cancellation or error
	select {
	case err := <-errCh:
		return err
	case <-ctx.Done():
		slog.Info("shutting down")
		return nil
	}
}

// Listen listens for file system events and handles them accordingly.
// It starts a loop that listens for events on the watcher's Events channel.
// If the context is canceled, listen returns the context error.
// If an event is received, listen logs a debug message and handles the event based on its type.
// For WRITE events, listen debounces the event by checking if the last event time is within the debounce delay.
// If the event is not debounced, listen calls handleFileEvent to send the file to the receiver.
// For REMOVE and RENAME events, listen calls handleFileEvent to delete the file from the receiver.
// If an error is received from the watcher's Errors channel, listen logs an error message.
func listen(ctx context.Context, conf Config, watcher *fsnotify.Watcher) error {
	var (
		lastEventTimes = make(map[string]time.Time)
		debounceDelay  = 2500 * time.Millisecond
	)
	for {
		select {
		case <-ctx.Done():
			return ctx.Err()

		case event, ok := <-watcher.Events:
			if !ok {
				return nil
			}

			slog.Debug("file system event", "event", event)

			if event.Has(fsnotify.Write) || event.Has(fsnotify.Chmod) || event.Has(fsnotify.Create) {
				if time.Since(lastEventTimes[event.Name]) < debounceDelay {
					slog.Debug("event debounced", "path", event.Name)
					continue
				}

				time.Sleep(100 * time.Millisecond)

				lastEventTimes[event.Name] = time.Now()
				if err := handleFileEvent(event.Name, "WRITE", conf); err != nil {
					slog.Error("failed to handle write event", "path", event.Name, "error", err)
				}
			} else if event.Has(fsnotify.Remove) {
				if err := handleFileEvent(event.Name, "DELETE", conf); err != nil {
					slog.Error("failed to handle delete/rename event", "path", event.Name, "error", err)
				}
			}

		case err, ok := <-watcher.Errors:
			if !ok {
				return nil
			}
			slog.Error("watcher error", "error", err)
		}
	}
}

// addPathRecursively adds a path and all its subdirectories to the watcher.
// For each directory encountered, it adds the directory to the watcher.
// Returns an error if any directory could not be added to the watcher.
func addPathRecursively(watcher *fsnotify.Watcher, path string) error {
	fileInfo, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("failed to stat path %s: %w", path, err)
	}

	if !fileInfo.IsDir() {
		return nil
	}

	// For directories, walk through all subdirectories
	err = filepath.WalkDir(path, func(subPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		// Only add directories to the watcher
		if d.IsDir() {
			if err := watcher.Add(subPath); err != nil {
				return fmt.Errorf("failed to watch directory %s: %w", subPath, err)
			}
			slog.Debug("watching directory", "path", subPath)
		}

		return nil
	})

	if err != nil {
		return fmt.Errorf("failed to walk directory %s: %w", path, err)
	}

	return nil
}

// handleFileEvent handles a file system event by sending the file to the receiver.
// It takes the path of the file and the action to take (WRITE or DELETE).
// For WRITE events, handleFileEvent reads the file and sends its contents to the receiver.
// For DELETE events, handleFileEvent sends an empty request to the receiver to delete the file.
// If the operation times out (after 30 seconds), handleFileEvent returns a timeout error.
func handleFileEvent(path, action string, conf Config) error {
	done := make(chan error, 1)

	go func() {
		defer close(done)
		var err error

		defer func() {
			if r := recover(); r != nil {
				err = fmt.Errorf("panic in file handler: %v", r)
			}
		}()

		switch action {
		case "WRITE":
			err = readAndSend(path, conf)
		case "DELETE":
			err = send(path, "", "DELETE", conf)
		}

		done <- err
	}()

	// Wait for the operation to complete or timeout
	select {
	case err := <-done:
		return err
	case <-time.After(30 * time.Second):
		return fmt.Errorf("timeout processing file event: %s %s", action, path)
	}
}
