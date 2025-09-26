# search-text

A fast and flexible command-line tool to recursively search for text
or regex patterns in files under a directory.

## Features
- Recursively search all files in a directory and its subdirectories
- Supports plain text and regex pattern matching
- Filter files by extension (e.g. `.rs`, `.txt`, or multiple)
- Asynchronous file reading for high performance
- Command-line options auto-filled from Cargo metadata
- If no directory is specified, search starts from the current working directory

## Usage

```sh
search-text [OPTIONS] -p <PATTERN>
```

### Options
- `-p`, `--pattern <PATTERN>`: Text or regex pattern to search for (required)
- `-d`, `--dir <DIR>`: Directory to search (optional, default: current working directory)
- `-r`, `--regex`: Use regex pattern matching (optional)
- `-e`, `--ext <EXT>`: File extensions to filter, comma separated (e.g. `rs,txt`). If omitted, all files are searched.

### Examples

Search for the word "TODO" in all `.rs` files:
```sh
search-text -p TODO -e rs
```

Search for a regex pattern in `.txt` and `.md` files:
```sh
search-text -p "T.O" -r -e txt,md
```

Search for a string in all files under current directory:
```sh
search-text -p hello
```

Search for a string in a specific directory:
```sh
search-text -p hello -d ./src
```

## Installation

### From crates.io
```sh
cargo install search-text
```

### From Source
Clone the repo and build with Cargo:
```sh
git clone https://github.com/ssrlive/search-text.git
cd search-text
cargo install search-text --path .
```

## License
MIT
