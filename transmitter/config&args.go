package main

import (
	"encoding/json"
	"log/slog"
	"os"

	"github.com/alexflint/go-arg"
)

type Config struct {
	Url   string
	Paths []string
	Key   string
}

// getConfig reads the configuration from a JSON file specified by the -C flag.
// It will exit the program if the file cannot be read or the JSON cannot be unmarshaled.
// The configuration is expected to contain the following fields:
// - Url: the URL of the receiver
// - Paths: a list of paths to watch for files
// - Key: the authorization key for the receiver
func getConfig() Config {
	var args struct {
		Config string `arg:"-C,--config,required" help:"path to a config.json file"`
		Key    string `arg:"-k,--key,required" help:"key to use for authontication same as receiver"`
	}

	arg.MustParse(&args)

	data, err := os.ReadFile(args.Config)
	if err != nil {
		slog.Error("failed to read config file", "error", err)
		os.Exit(1)
	}

	var cfg Config
	err = json.Unmarshal(data, &cfg)
	if err != nil {
		slog.Error("failed to unmarshal config", "error", err)
		os.Exit(1)
	}

	cfg.Key = args.Key

	return cfg
}
