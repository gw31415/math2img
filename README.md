# math2img

Convert mathematical expressions into images.

![output](https://github.com/gw31415/math2img/assets/24710985/01f96e75-2db5-42e2-854c-960a0587ca59)

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
