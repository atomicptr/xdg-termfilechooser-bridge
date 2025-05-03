# xdg-termfilechooser-bridge

The bridge between [xdg-desktop-portal-termfilechooser](https://github.com/hunkyburrito/xdg-desktop-portal-termfilechooser) and your favorite terminal based filepicker

## Installation

### Install the application

#### via cargo

```bash
$ cargo install xdg-termfilechooser-bridge
```

### Setup xdg-desktop-portal-termfilechooser

Edit `$XDG_CONFIG_HOME/xdg-desktop-portal-termfilechooser/config`

```bash
[filechooser]
cmd = /path/to/xdg-termfilechooser-bridge
default_dir = $HOME
```

## Configuration

Edit `$XDG_CONFIG_HOME/xdg-termfilechooser-bridge/config.toml`

```toml
terminal = "ghostty"
filepicker = "yazi"
start_at_last_selected_dir = true
```

## Supported file pickers

- [yazi](https://yazi-rs.github.io/)

If your favorite terminal filepicker isn't available feel free to [add it yourself](./src/filepicker.rs) or open an issue with the required invoke parameters.

## Supported terminals

- [ghostty](https://ghostty.org/)
- [kitty](https://sw.kovidgoyal.net/kitty/)

If your favorite terminal isn't available feel free to [add it yourself](./src/terminals.rs) or open an issue with the required invoke parameters.

## Motivation

The default script of xdg-desktop-portal-termfilechooser has had several issues in the past and beyond these I wanted to customize
the behaviour of it further so I wrote a tool to do so.

## License

GPLv3
