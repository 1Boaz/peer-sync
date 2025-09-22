package main

import "sync"

func main() {
	conf := getConfig()
	var wg sync.WaitGroup
	for _, path := range conf.Paths {
		wg.Add(1)
		go read(path, &wg, conf.Url)
	}
	wg.Wait()
}
