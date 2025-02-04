# f2i

Preview .fits files directly in the terminal and convert them to images. Works on remote hosts over ssh!

![Screenshot From 2025-02-02 15-37-47](https://github.com/user-attachments/assets/a2aef716-a8a4-4f53-b46a-8bc2f518166d)

## Compatible Terminals
Your terminal must support either the Kitty or iTerm image protocols.

A few terminals with known support:
* [Ghostty](https://ghostty.org/)
* [Kitty](https://sw.kovidgoyal.net/kitty/)
* [Wezterm](https://wezfurlong.org/wezterm/index.html)
* [iTerm2](https://iterm2.com/) (OSX only)

## Thumbnail Generation
f2i can also quickly save .fits to common image formats such as PNG and JPEG
using the `--output` parameter. See `f2i --help` for more options.


## Benchmarks
When used as a thumbnailer, f2i has significant speed gains over fits2image:

| Test | f2i | fits2image|
|------|-----|-----------|
|tfn0m436-sq33-20250201-0129-e91.fits - 800x800 jpg | 84 ms | 255 ms |
|tfn0m436-sq33-20250201-0129-e91.fits - original size | 106 ms | 255 ms |
|tfn0m436-sq33-20250201-0129-e91.fits - 800x800 jpg 10 times | 858 ms | 2.6 seconds |
|tfn0m436-sq33-20250201-0129-e91.fits - original size 10 times | 1.1 seconds | 2.6 seconds |


## Installation

Binaries for Linux-x86_64 can be downloaded from the [release page](https://github.com/Fingel/f2i/releases).

## Building

`cfitsio` and `openblas` are required to build f2i. Built and tested with Rust 1.84.

`cargo build --release && cargo install`
