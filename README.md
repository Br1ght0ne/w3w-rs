# w3w-rs

> Rust crates for accessing [what3words public API]

## `w3w-cli`

[![Lib.rs](https://img.shields.io/crates/v/w3w-cli?label=lib.rs)](https://lib.rs/crate/w3w-cli)
[![Crates.io downloads](https://img.shields.io/crates/d/w3w-cli)](https://crates.io/crates/w3w-cli)

CLI that provided access to [what3words public API] in batch mode.

### Features

- Reasonably fast
- Zero system dependencies
- Supports most [what3words public API] endpoints, excluding speech recognition
- *WIP:* Interactive mode with autosuggest

### Installation

Binary releases are not provided yet.
If you have a recent installation of Rust, you can use `cargo install`:

```sh
cargo install w3w-cli
```

The binary name is `w3w` for brevity.

### Usage

You need to have a [what3words API key](https://accounts.what3words.com/overview).
You can provide it via `-k|--key` flag or via `W3W_API_KEY` env variable.

`w3w` uses the same strategy as `cat(1)` for handling files and standard input.
(*WIP:* `w3w` doesn't currently support multiple files, you can use `cat(1)` beforehand :smile:)
If no `[file]` is provided, input is read from stdout.

<!-- markdownlint-disable MD033 -->

<details>
<summary>`w3w --help`</summary>

```none
USAGE:
    w3w [OPTIONS] --api-key <api-key> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --api-key <api-key>         Your what3words API key [env: W3W_API_KEY]
        --log-format <format>       Log format to be used by tracing-subscriber [env: W3W_LOG_FORMAT=]  [default: full]
                                    [possible values: compact, full, json, pretty]
    -o, --output-format <format>    Output format to write to stdout [env: W3W_OUTPUT_FORMAT=]  [default: plain]
                                    [possible values: plain, json]

SUBCOMMANDS:
    available-languages    List all available language for three-word-addresses
    help                   Prints this message or the help of the given subcommand(s)
    to-3wa                 Convert geographic coordinates to three-word-addresses
    to-coords              Convert three-word-addresses to geographic coordinates
```

</details>

### Examples

```sh
$ w3w to-coords <(echo "filled.count.soap")
51.520847,-0.195521

$ w3w -o json to-coords <(echo "filled.count.soap") | jq .coordinates
{ "lat": 51.520847, "lng": -0.195521 }

$ w3w to-3wa <(echo "51.520847,-0.195521")
filled.count.soap

$ w3w available-languages
English (en), German (de), ...
```

## `w3w-api`

[![Lib.rs](https://img.shields.io/crates/v/w3w-api?label=lib.rs)](https://lib.rs/crate/w3w-api)
[![docs.rs](https://img.shields.io/docsrs/w3w-api)](https://docs.rs/w3w-api)

Rust library for [what3words public API].

## Contributing

All kinds of contributions are welcome, especially about increasing the coverage of upstream API.

## License

This project is licensed under [MIT license](LICENSE).

[what3words public API]: https://developer.what3words.com/public-api
