package main

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"

	"github.com/alexflint/go-arg"
)

type Config struct {
	Url   string
	Paths []string
	Key   string
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
		err := filepath.WalkDir(path, func(path2 string, d fs.DirEntry, err error) error {
			if err != nil {
				return err
			}
			if d.IsDir() {
				return nil
			}
			expandedPaths = append(expandedPaths, path2)
			return nil
		})
		if err != nil {
			fmt.Println(err)
		}
	}
	cfg.Paths = expandedPaths

	return cfg
}
