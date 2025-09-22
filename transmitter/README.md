# Peer Sync

Peer Sync is a simple cross-platform file sender and receiver. The transmitter (written in Go) sends files to a receiver (written in Rust, using Actix Web) over HTTP.

## Project Structure

```
receiver/      # Rust HTTP server that receives and saves files
transmitter/   # Go client that reads files and sends them to the receiver
```

### Choice Of Languages
The receiver is written in Rust for maximum preformence and minimal resource usage, making it suitable for devices like the Raspberry Pi.  
The transmitter is written in Go to speed up development.

## Receiver (Rust)

### Requirements

- Rust 1.70+ (or latest stable)

### Build & Run

```sh
cd receiver
cargo build --release
./target/release/receiver -p 8080
```

- The server listens on the local IP and specified port (default: 8080).
- Receives HTTP POST requests with JSON payload:
  ```json
  {
    "path": "relative/or/absolute/path/to/file",
    "content": "file contents as string"
  }
  ```
- Saves the file to the given path, creating directories as needed.

## Transmitter (Go)

### Requirements

- Go 1.24+

### Configuration

Create a `config.json` file (location does not matter if you specify the path in the command-line flag):

```json
{
  "url": "http://<receiver-ip>:8080/",
  "paths": ["file1.txt", "dir/file2.txt"]
}
```

### Build & Run

```sh
cd transmitter
go build -o transmitter
./transmitter --config path/to/config.json
```

- Reads each file in `paths` and sends its contents to the receiver’s URL.
- Uses concurrent goroutines for sending files.

## Usage Example

1. Start the receiver:
   ```sh
    ./ executable path
   ```
2. Start the transmitter:
   ```sh
   ./transmitter --config config.json
   ```

## Notes

- The transmitter sends files as plain text in the JSON payload.
- The receiver creates directories as needed and overwrites existing files.
- Adjust the `url` in `config.json` to match your receiver’s IP and port.

## License

MIT