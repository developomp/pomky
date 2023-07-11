# Pomky

A personal conky-like Linux system monitor built with rust.

Pomky is not as configurable, modular, or feature packed as conky.
It is tailor-made for my need and hardware.
If you are looking for an conky alternative written in rust, check out [randy](https://github.com/iphands/randy) or maybe even consider using [eww](https://github.com/elkowar/eww).

![screenshot](./screenshot.png)

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

This command builds the `pomky` binary and saves it to `~/.cargo/bin/pomky`.

Since pomky is designed to only run on my hardware,
it will most likely not compile on your system.
No technical support will be provided.

```bash
cargo install --path .
```

## License

The source code for this project is available under the MIT [License](./LICENSE).
