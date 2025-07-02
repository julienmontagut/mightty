# Mightty - A fast terminal emulator based on Mightty

<p align="center">
  <img alt="Mightty - A fast, cross-platform, OpenGL terminal emulator"
       src="https://raw.githubusercontent.com/mightty/mightty/master/extra/promo/alacritty-readme.png">
</p>

## About

Mightty is a modern terminal emulator that comes with sensible defaults, but
allows for extensive [configuration](#configuration). By integrating with other
applications, rather than reimplementing their functionality, it manages to
provide a flexible set of [features](./docs/features.md) with high performance.
The supported platforms currently consist of OpenBSD, Linux and macOS.

The software is considered to be at a **beta** level of readiness; there are
a few missing features and bugs to be fixed, but it is already used by many as
a daily driver.

Precompiled binaries are available from the [GitHub releases page](https://github.com/mightty/mightty/releases).

## Features

You can find an overview over the features available in Mightty [here](./docs/features.md).

## Installation

### Quick Install (Recommended)

#### macOS and Linux
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/julienmontagut/mightty/releases/latest/download/mightty-installer.sh | sh
```

#### Homebrew
```bash
brew install julienmontagut/homebrew/mightty
```

#### Cargo
```bash
cargo install mightty
```

### Other Installation Methods

Prebuilt binaries for macOS and Linux can also be downloaded from the
[GitHub releases page](https://github.com/julienmontagut/mightty/releases).

For building from source, detailed instructions can be found in [INSTALL.md](INSTALL.md).

## Configuration

You can find the documentation for Mightty's configuration in `man 5
mightty`, or by looking at if you do not have the manpages
installed.

Mightty doesn't create the config file for you, but it looks for one in the
following locations:

1. `$XDG_CONFIG_HOME/mightty/mightty.toml`
2. `$XDG_CONFIG_HOME/mightty.toml`
3. `$HOME/.config/mightty/mightty.toml`
3. `$HOME/.config/mightty.toml`

## Contributing

A guideline about contributing to Mightty can be found in the
[`CONTRIBUTING.md`](CONTRIBUTING.md) file.

## License

Mightty is released under the [Apache License, Version 2.0].

[Apache License, Version 2.0]: https://github.com/mightty/mightty/blob/master/LICENSE
