# Installation


## macOS and Linux
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/julienmontagut/mightty/releases/latest/download/mightty-installer.sh | sh
```

## Homebrew
```bash
brew install julienmontagut/homebrew/mightty
```

# Cargo Installation

If you prefer to build from source, you can install directly through cargo:

```sh
cargo install mightty
```

Note that you will still need to install the dependencies for your OS.
Please refer to the [Dependencies](#dependencies) section.

# Manual Installation

1. [Prerequisites](#prerequisites)
    1. [Source Code](#clone-the-source-code)
    2. [Rust Compiler](#install-the-rust-compiler-with-rustup)
    3. [Dependencies](#dependencies)
        1. [Debian/Ubuntu](#debianubuntu)
        2. [Fedora/RHEL](#fedorarhel)
        3. [macOS](#macos-dependencies)
2. [Building](#building)
3. [Post Build](#post-build)
    1. [Terminfo](#terminfo)
    2. [Desktop Entry](#desktop-entry)
    3. [Manual Page](#manual-page)
    4. [Shell completions](#shell-completions)
        1. [Zsh](#zsh)
        2. [Bash](#bash)
        3. [Fish](#fish)

## Prerequisites

### Clone the source code

Before compiling Mightty, you'll have to first clone the source code:

```sh
git clone https://github.com/mightty/mightty.git
cd mightty
```

### Install the Rust compiler with `rustup`

1. Install [`rustup.rs`](https://rustup.rs/).

3. To make sure you have the right Rust compiler installed, run

   ```sh
   rustup override set stable
   rustup update stable
   ```

### Dependencies

These are the minimum dependencies required to build Mightty, please note
that with some setups additional dependencies might be desired.

If you're running Wayland with an Nvidia GPU, you'll likely want the EGL
drivers installed too (these are called `libegl1-mesa-dev` on Ubuntu).

#### Debian/Ubuntu

If you'd like to build a local version manually, you need a few extra libraries
to build Mightty. Here's an apt command that should install all of them. If
something is still found to be missing, please open an issue.

```sh
apt install cmake g++ pkg-config libfontconfig1-dev libxcb-xfixes0-dev libxkbcommon-dev python3
```
#### Fedora/RHEL

On Fedora and RHEL-based systems, install the required dependencies:

```sh
# Fedora
sudo dnf install cmake freetype-devel fontconfig-devel libxcb-devel libxkbcommon-devel gcc-c++

# RHEL/CentOS 8+
sudo dnf install cmake freetype-devel fontconfig-devel libxcb-devel libxkbcommon-devel
sudo dnf group install "Development Tools"
```

#### macOS Dependencies

On macOS, install the required dependencies using Homebrew:

```sh
brew install cmake pkg-config
```

## Building

### All Platforms

```sh
cargo build --release
```

On Linux, Wayland support is enabled by default and is the only supported backend:

```sh
# Standard build (Wayland enabled by default)
cargo build --release
```

If all goes well, this should place a binary at `target/release/mightty`.

### Universal Binary (macOS)

To build a universal binary that runs on both Intel and Apple Silicon:

```sh
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --release --target=x86_64-apple-darwin
cargo build --release --target=aarch64-apple-darwin
lipo target/{x86_64,aarch64}-apple-darwin/release/mightty -create -output target/release/mightty-universal
```

## Post Build

There are some extra things you might want to set up after installing Mightty.
All the post build instruction assume you're still inside the Mightty
repository.

### Terminfo

To make sure Mightty works correctly, either the `mightty` or
`mightty-direct` terminfo must be used. The `mightty` terminfo will be
picked up automatically if it is installed.

If the following command returns without any errors, the `mightty` terminfo is
already installed:

```sh
infocmp mightty
```

If it is not present already, you can install it globally with the following
command:

```
sudo tic -xe mightty,mightty-direct extra/mightty.info
```

### Desktop Entry

Many Linux distributions support desktop entries for adding applications
to system menus. This will install the desktop entry for Mightty:

```sh
sudo cp target/release/mightty /usr/local/bin # or anywhere else in $PATH
sudo cp extra/logo/mightty-term.svg /usr/share/pixmaps/Mightty.svg
sudo desktop-file-install extra/linux/Mightty.desktop
sudo update-desktop-database
```

If you are having problems with Mightty's logo, you can replace it with
prerendered PNGs and simplified SVGs available in the `extra/logo/compat`
directory.

### Manual Page

Installing the manual page requires the additional dependencies `gzip` and `scdoc`.

```sh
sudo mkdir -p /usr/local/share/man/man1
sudo mkdir -p /usr/local/share/man/man5
scdoc < extra/man/mightty.1.scd | gzip -c | sudo tee /usr/local/share/man/man1/mightty.1.gz > /dev/null
scdoc < extra/man/mightty-msg.1.scd | gzip -c | sudo tee /usr/local/share/man/man1/mightty-msg.1.gz > /dev/null
scdoc < extra/man/mightty.5.scd | gzip -c | sudo tee /usr/local/share/man/man5/mightty.5.gz > /dev/null
scdoc < extra/man/mightty-bindings.5.scd | gzip -c | sudo tee /usr/local/share/man/man5/mightty-bindings.5.gz > /dev/null
```

### Shell completions

To get automatic completions for Mightty's flags and arguments you can install the provided shell completions.

#### Zsh

To install the completions for zsh, you can place the `extra/completions/_mightty` file in any
directory referenced by `$fpath`.

If you do not already have such a directory registered through your `~/.zshrc`, you can add one like this:

```sh
mkdir -p ${ZDOTDIR:-~}/.zsh_functions
echo 'fpath+=${ZDOTDIR:-~}/.zsh_functions' >> ${ZDOTDIR:-~}/.zshrc
```

Then copy the completion file to this directory:

```sh
cp extra/completions/_mightty ${ZDOTDIR:-~}/.zsh_functions/_mightty
```

#### Bash

To install the completions for bash, you can `source` the `extra/completions/mightty.bash` file
in your `~/.bashrc` file.

If you do not plan to delete the source folder of mightty, you can run

```sh
echo "source $(pwd)/extra/completions/mightty.bash" >> ~/.bashrc
```

Otherwise you can copy it to the `~/.bash_completion` folder and source it from there:

```sh
mkdir -p ~/.bash_completion
cp extra/completions/mightty.bash ~/.bash_completion/mightty
echo "source ~/.bash_completion/mightty" >> ~/.bashrc
```

#### Fish

To install the completions for fish, from inside the fish shell, run

```
mkdir -p $fish_complete_path[1]
cp extra/completions/mightty.fish $fish_complete_path[1]/mightty.fish
```
