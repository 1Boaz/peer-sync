package main

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/alexflint/go-arg"
)

type Config struct {
	Url   string
	Paths []string
}

func getConfig() Config {
	var args struct {
		Config string `arg:"-C,--config,required" help:"path to a config.json file"`
	}

	arg.MustParse(&args)

	data, err := os.ReadFile(args.Config)
	if err != nil {
		panic(err)
	}

	var cfg Config
	err = json.Unmarshal(data, &cfg)
	if err != nil {
		panic(err)
	}

	var expandedPaths []string

	for _, path := range cfg.Paths {
		fi, err := os.Stat(path)
		if err != nil {
			fmt.Println(err)
			continue
		}
		if fi.IsDir() {
			entries, err := os.ReadDir(path)
			if err != nil {
				fmt.Println(err)
				continue
			}

			for _, e := range entries {
				expandedPaths = append(expandedPaths, path+e.Name())
			}
		} else {
			expandedPaths = append(expandedPaths, path)
		}
	}
	cfg.Paths = expandedPaths

	return cfg
}
