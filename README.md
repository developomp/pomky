# Pomky

[![what's this?](https://img.shields.io/badge/what's_this%3F-grey?style=for-the-badge)](https://developomp.com/portfolio/pomky)

A personal conky-like Linux system monitor built with rust.

Pomky is not as configurable, modular, or feature packed as conky.
It is tailor-made for my need and hardware.
If you are looking for an conky alternative written in rust, check out [iphands/randy](https://github.com/iphands/randy).

![screenshot](./screenshot.png)

## Installing

Since pomky is designed to only run on my hardware,
it will most likely not compile on your system.
No technical support will be provided.

- Designed using [glade](https://wiki.gnome.org/Apps/Glade)

### Requirements

- [cargo](https://doc.rust-lang.org/stable/cargo)
- [Noto Sans font](https://fonts.google.com/noto/specimen/Noto+Sans)
- [Audiowide font](https://fonts.google.com/specimen/Audiowide)

### Commands

#### Debugging with hot reload

```bash
cargo run
```

#### Installing

This command saves `pomky` binary to `~/.cargo/bin/pomky`.

```bash
cargo install --path .
```

## License

The source code for this project is available under the MIT [License](./LICENSE).
