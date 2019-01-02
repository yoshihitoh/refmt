# reser
[![Build Status](https://travis-ci.org/yoshihitoh/reser.svg?branch=master)](https://travis-ci.org/yoshihitoh/reser)

reser is a data format translation tool written in Rust. Currently only JSON, YAML and TOML are available.

# Installation
reser is written ins Rust, so you need Rust toolchains. reser compiled with Rust 1.30.0 (stable) or newer.

To build reser:

```bash
$ git clone https://github.com/yoshihitoh/reser
$ cd reser
$ cargo build --release
$ ./target/release/reser --version
reser 0.1.0
```

# Usage
``` bash
$ ./target/release/reser --help
reser 0.1.0
yoshihitoh <yoshihito.arih@gmail.com>
Translate data format into another one.

USAGE:
    reser [OPTIONS]

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
$ echo '{"id": 1, "name": {"first": "John", "last": "Doe"}}' | ./target/release/reser --input-format json --output-format yaml
---
id: 1
name:
  first: John
  last: Doe
```


## YAML to JSON
``` bash
$ cat <<EOS | ./target/release/reser --input-format yaml --output-format json
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

# Running tests
```bash
$ cargo test --all
```
