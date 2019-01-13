# refmt
[![Build Status](https://travis-ci.org/yoshihitoh/refmt.svg?branch=master)](https://travis-ci.org/yoshihitoh/refmt)

refmt is a data format translation tool written in Rust. Currently only JSON, YAML and TOML are available.

# Syntax highlighting
refmt supports syntax highlighting.

![Syntax highlighting example](https://i.imgur.com/vSlsRAC.png)

# Installation
refmt is written ins Rust, so you need Rust toolchains. refmt compiled with Rust 1.30.0 (stable) or newer.

To build refmt:

```bash
$ git clone https://github.com/yoshihitoh/refmt
$ cd refmt
$ cargo build --release
$ ./target/release/refmt --version
refmt 0.1.2
```

# Usage
``` bash
$ ./target/release/refmt --help
refmt 0.1.2
yoshihitoh <yoshihito.arih@gmail.com>
Translate data format into another one.

USAGE:
    refmt [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>                   set the input file to use
        --input-format <FORMAT_NAME>     set the name of input format [possible values: json, yaml]
    -o, --output <FILE>                  set the output file to use
        --output-format <FORMAT_NAME>    set the name of output format [possible values: json, yaml]
```

# Examples

## JSON to YAML
``` bash
$ echo '{"id": 1, "name": {"first": "John", "last": "Doe"}}' | ./target/release/refmt --input-format json --output-format yaml
---
id: 1
name:
  first: John
  last: Doe
```


## YAML to JSON
``` bash
$ cat <<EOS | ./target/release/refmt --input-format yaml --output-format json
> ---
> id: 1
> name:
>   first: John
>   last: Doe
> EOS
{
  "id": 1,
  "name": {
    "first": "John",
    "last": "Doe"
  }
}
```

## JSON to TOML
``` bash
$ echo '{"id": 1, "name": {"first": "John", "last": "Doe"}}' | refmt --input-format json --output-format toml
id = 1

[name]
first = 'John'
last = 'Doe'

```

# Running tests
```bash
$ cargo test --all
```
