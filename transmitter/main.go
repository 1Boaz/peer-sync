package main

import (
	"log"
	"sync"

	"github.com/fsnotify/fsnotify"
)

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

func listen(conf Config, watcher *fsnotify.Watcher) {
	for {
		select {
		case event, ok := <-watcher.Events:
			if !ok {
				return
			}
			log.Println("event:", event)
			if event.Has(fsnotify.Write) {
				var wg sync.WaitGroup
				for _, path := range conf.Paths {
					wg.Add(1)
					go read(path, &wg, conf.Url)
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
