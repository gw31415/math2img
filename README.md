# math2img

Convert mathematical expressions into images.

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
