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
```

## Configuration

By default the tool will try to determine your terminal and filepicker automatically by checking if you have one of the
supported ones installed, if you want to use a specific one please edit `$XDG_CONFIG_HOME/xdg-termfilechooser-bridge/config.toml`

```toml
terminal = "ghostty"
filepicker = "yazi"
start_at_last_selected_dir = true
```

## Supported file pickers

- [yazi](https://yazi-rs.github.io/)

If your favorite terminal filepicker isn't available feel free to [add it yourself](./src/filepicker.rs) or open an issue with the required invoke parameters.

## Supported terminals

- [alacritty](https://alacritty.org/)
- [foot](https://codeberg.org/dnkl/foot)
- [ghostty](https://ghostty.org/)
- [kitty](https://sw.kovidgoyal.net/kitty/)
- [xterm](https://invisible-island.net/xterm/)

If your favorite terminal isn't available feel free to [add it yourself](./src/terminals.rs) or open an issue with the required invoke parameters.

## Motivation

The default script of xdg-desktop-portal-termfilechooser has had several issues in the past and beyond these I wanted to customize
the behaviour of it further so I wrote a tool to do so.

## License

GPLv3
