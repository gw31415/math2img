# math2img

[![Crates.io](https://img.shields.io/crates/v/math2img?style=flat-square)](https://crates.io/crates/math2img)
[![Crates.io](https://img.shields.io/crates/d/math2img?style=flat-square)](https://crates.io/crates/math2img)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE)

Convert mathematical expressions into images.

<img width="672" alt="image" src="https://github.com/gw31415/math2img/assets/24710985/d768a852-7e10-49ff-9b08-c640db35a4e0">

## Installation

### with Cargo

```bash
cargo install math2img
```

## Usage

```help
Convert mathematical expressions into images

Usage: math2img [OPTIONS] [MATH]

Arguments:
  [MATH]  Mathjax expression

Options:
      --help                Print help
  -o, --output <OUTPUT>     Output filename
      --png                 Output as PNG. If the argument `output` has a `.png` extension, it will be set automatically
      --completion <SHELL>  Generate shell completion [possible values: bash, elvish, fish, powershell, zsh]
  -w, --width <WIDTH>       Number of horizontal dots in PNG image
  -h, --height <HEIGHT>     Number of vertical dots in PNG image
  -V, --version             Print version
```

## Screenshots

![output](https://github.com/gw31415/math2img/assets/24710985/01f96e75-2db5-42e2-854c-960a0587ca59)
