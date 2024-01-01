# Grep Lite - A Rust implementation of the `grep` command-line tool

This Rust program is a simple yet powerful tool for searching for a specific pattern in a text file. Utilizing the `clap` crate for command-line argument parsing, it offers a user-friendly way to specify both the pattern to search for and the file to search within.

## Features

- Command-line argument parsing using `clap::Parser`.
- Reading and processing a text file with efficient bufferization.
- Case-insensitive pattern matching in text files.

## Usage

To use this program, you need to provide two arguments:

- `-p` or `--pattern` followed by the pattern you want to search for.
- `-f` or `--file` followed by the path to the text file.

## Example

```bash
  cargo run -- -p "your_pattern" -f "path/to/your/file.txt"
```

This will search for "your_pattern" in "file.txt" and print the lines where the pattern is found, along with their line numbers.

## Implementation Details

- The `Args` struct, derived from `clap::Parser`, handles the parsing of command-line arguments.
- The `process_file` function takes a file path, opens the file, and creates a `BufReader` for efficient line-by-line reading.
- The main function parses the arguments, processes the file, and iterates through each line to find matches. Matched lines are printed to the console with their line number.

## Dependencies

- `clap` for command-line argument parsing.

## Note

This program performs a case-insensitive search. The pattern and lines from the file are converted to lowercase before comparison.
