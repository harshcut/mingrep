# Getting Started

This is a minimal grep command-line utility built on Rust. It provides searching for plain-text files for lines that match a given string.

> grep is a command-line utility for searching plain-text data sets for lines that match a regular expression. Its name comes from the ed command g/re/p, which has the same effect. grep was originally developed for the Unix operating system, but later available for all Unix-like systems and some others such as OS-9.

## Developing

Fork the repository using [this](https://docs.github.com/en/github/getting-started-with-github/fork-a-repo) guide, then clone it locally. Use the `test` command to check if the search functions work as intended.

```shell
git clone https://github.com/harshcut/mingrep.git
cd mingrep
cargo test
```

## Usage

To run search on a plain-text file, pass the query string followed by the file path. You can also write the output to an external file.

```shell
cargo run mind dune.txt
cargo run mind dune.txt > output.txt
```

By default, the search case is set to `sensitive`. To alter the case, set an environment variable `CASE_INSENSITIVE` as `true`. To set an environment variable through your bash terminal use `export`.

```bash
export CASE_INSENSITIVE=true
cargo run where dune.txt
unset CASE_INSENSITIVE
```

## License

This project is licensed under the [MIT License](https://github.com/harshcut/mingrep/blob/main/LICENSE).
