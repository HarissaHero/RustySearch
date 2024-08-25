# RustySearch

RustySearch is a simple, blazzingly fast command-line tool written in Rust for searching a specific word in a text file. It highlights the occurrences of the word within one or multiple files and provides a preview of the context around each occurrence.

## Features

- Search for a specific word in a text file or files within a directory.
- Display occurrences with line number, column number, and a preview of the surrounding text.
- Simple and easy-to-use command-line interface.
- Set max-threads for an even blazzingly faster search (only while searching within directory)

## Installation

To use RustySearch, you'll need to have [Rust](https://www.rust-lang.org/learn/get-started) installed on your machine. You can then build the project using Cargo.

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/rustysearch.git
    cd rustysearch
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. The binary will be located in `target/release/rustysearch`.

## Usage

Run RustySearch from the command line by specifying the word to search for and the file to search in. 

### Basic Usage

```sh
rustysearch <word> -f <file>
rustysearch <word> -d <directory> 
rustysearch <word> -t <max-threads> 
```

#### Example

```sh
rustysearch "what is love" -f the-book-of-love.md
rustysearch "what is love" -d the-library-of-love
rustysearch "what is love" -d the-library-of-love -t 4
```

