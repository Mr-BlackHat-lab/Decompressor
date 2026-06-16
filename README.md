# unzip

> This README was made by AI.

`unzip` is a small command-line tool for extracting `.zip` archives in Rust.
It is designed to be simple, safe, and easy to run from the terminal.

## Features

- Streamed extraction: files are copied directly from the archive to disk with `std::io::copy`, so extraction does not require loading file contents into memory first.
- Path safety: entries are resolved with `enclosed_name()` to avoid writing files outside the target directory.
- Unix permission restoration: on Unix platforms, extracted files keep the original permission bits stored in the archive.
- Minimal interface: the CLI currently accepts one positional argument, the path to the `.zip` file.

## Installation

Make sure Rust and Cargo are installed, then build the project:

```bash
cargo build --release
```

The compiled binary will be available at:

```bash
target/release/unzip
```

## Usage

Pass the path to a `.zip` archive as the only argument:

```bash
# Run from source during development
cargo run -- my_archive.zip

# Run the compiled binary
target/release/unzip my_archive.zip
```

Extraction happens in the current working directory. Parent directories are created automatically when needed.

## How It Works

The current implementation performs the following steps:

1. Opens the archive file provided on the command line.
2. Reads the archive with `zip::ZipArchive`.
3. Iterates through each entry in the archive.
4. Skips entries that do not have a safe enclosed path.
5. Creates directories for folder entries and parent directories for files.
6. Streams file contents to disk.
7. Restores Unix file permissions when the archive contains them.

## Dependencies

- `clap` for parsing command-line arguments.
- `zip` for reading ZIP archives.

## Notes

- This project currently supports standard `.zip` archives only.
- It does not support `.7z` or `.rar` archives.
- Files are extracted relative to the directory where the command is run.
- Unsafe archive entries are skipped instead of being written to disk.

## Example

```bash
cargo run -- downloads/example.zip
```

If the archive contains:

```text
docs/readme.txt
bin/tool
```

the tool will extract those files into matching paths under the current directory.
