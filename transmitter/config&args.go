package main

import (
	"encoding/json"
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

	return cfg
}
