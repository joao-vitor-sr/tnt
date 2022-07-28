# Cargo Installation

If you're just interested in the tnt binary and you don't need the
[manual page](#manual-page), you can
install it directly through cargo:

```sh
cargo install tnt-note
```

# Manual Installation

1. [Prerequisites](#prerequisites)
   1. [Source Code](#clone-the-source-code)
   2. [Rust Compiler](#install-the-rust-compiler-with-rustup)
2. [Building](#building)
   1. [Linux](#linux)
3. [Post Build](#post-build)
   1. [Manual Page](#manual-page)
   2. [Shell completions](#shell-completions)
      1. [Zsh](#zsh)
      2. [Bash](#bash)

## Prerequisites

### Clone the source code

Before compiling tnt, you'll have to first clone the source code:

```sh
git clone https://github.com/joao-vitor-sr/tnt.git
cd tnt
```

### install the Rust compiler with `rustup`

1. Install [`rustup.rs`](httsp://rustup.rs/)
2. To make sure you have the right Rust compiler installed, run
   ```sh
   rustup override set stable
   rustup update stable
   ```

## Building

### Linux

```sh
cargo build --release
```

If all goes well, this should place a binary at `target/release/tnt`.

## Post Build

There are some extra things you might want to set up after installing tnt.
All the post build instruction assume you're still inside the tnt
repository.

### Manual Page

Installing the manual page requires the additional dependency `gzip`.

```sh
sudo mkdir -p /usr/local/share/man/man1
gzip -c extra/tnt.man | sudo tee /usr/local/share/man/man1/tnt.1.gz > /dev/null
```

### Shell completions

To get automatic completions for tnt's flags and arguments you can install the provided shell completions.

#### Zsh

To install the completions for zsh, you can set up using complete

```sh
complete _gnu_generic tnt
```

Place it in your `~/.zshrc`

#### Bash

To install the completions for bash, you can add the following command
in your `~/.bashrc` file.

```sh
complete -F _longopt tnt
```
