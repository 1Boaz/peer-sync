package main

import (
	"fmt"
	"log"
	"sync"
	"time"

	"github.com/fsnotify/fsnotify"
)

// / Main entry point for the file sender.
// /
// / # Description
// / This function configures the file system watcher to listen for write events
// / on the specified paths, and triggers the reading of files when a write event
// / is detected. The function debounces write events within 2.5 seconds to prevent
// / excessive file reads on rapid file changes. The function will block until the watcher
// / is closed.
// /
// / # Errors
// / * Returns an error if the watcher is closed or if an error occurs while
// /   watching the file system events.
func main() {
	conf := getConfig()
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return
	}

	defer watcher.Close()

	go listen(conf, watcher)

	for _, path := range conf.Paths {
		err = watcher.Add(path)
		if err != nil {
			log.Fatal(err)
		}
	}

	<-make(chan struct{})
}

// / Listens for file system events and triggers the reading of files
// / when a write event is detected. The function debounces write events
// / within 2.5 seconds to prevent excessive file reads on rapid file
// / changes. The function will block until the watcher is closed.
// /
// / # Parameters
// / * `conf` - The configuration struct containing the paths to watch
// / * `watcher` - The file system watcher
// /
// / # Errors
// / * Returns an error if the watcher is closed or if an error occurs
// /   while watching the file system events.
func listen(conf Config, watcher *fsnotify.Watcher) {
	called_at := time.Time{}
	for {
		select {
		case event, ok := <-watcher.Events:
			if !ok {
				return
			}
			log.Println("event:", event)
			if event.Has(fsnotify.Write) {
				if time.Since(called_at) < 2500*time.Millisecond {
					fmt.Println("debounce")
					continue
				}
				called_at = time.Now()
				var wg sync.WaitGroup
				for _, path := range conf.Paths {
					wg.Add(1)
					go read(path, &wg, conf)
				}
				wg.Wait()
			}
		case err, ok := <-watcher.Errors:
			if !ok {
				return
			}
			log.Println("error:", err)
		}
	}
}
