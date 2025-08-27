toml2json
=========

[![CI](https://github.com/kkew3/toml2json/actions/workflows/ci.yml/badge.svg)](https://github.com/kkew3/toml2json/actions/workflows/ci.yml)

A command-line tool that converts TOML to JSON. Nothing more, nothing less.

## Installation

### Cargo

```bash
cargo install --git https://github.com/kkew3/toml2json.git
```

## Usage

Convert TOML on `stdin` to JSON, filtering it through `jq`:

```bash
toml2json <<< 'wow = "amazing"' | jq
```

Produces:

```json
{
  "wow": "amazing"
}
```

Amazing. What more could you want? Hopefully nothing, because it will never do anything else.
