# Cargo Installation

If you're just interested in the tnt binary and you don't need the
[manual page](#manual-page) or [shell completions](#shell-completions), you can
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
      3. [Fish](#fish)

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
gzip -c docs/tnt.man | sudo tee /usr/local/share/man/man1/tnt.1.gz > /dev/null
```

### Shell completions

To get automatic completions for tnt's flags and arguments you can install the provided shell completions.

#### Zsh

To install the completions for zsh, you can place the `extra/completions/_tnt` file in any
directory referenced by `$fpath`.

If you do not already have such a directory registered through your `~/.zshrc`, you can add one like this:

```sh
mkdir -p ${ZDOTDIR:-~}/.zsh_functions
echo 'fpath+=${ZDOTDIR:-~}/.zsh_functions' >> ${ZDOTDIR:-~}/.zshrc
```

Then copy the completion file to this directory:

```sh
cp extra/completions/_tnt ${ZDOTDIR:-~}/.zsh_functions/_tnt
```

#### Bash

To install the completions for bash, you can `source` the `extra/completions/tnt.bash` file
in your `~/.bashrc` file.

If you do not plan to delete the source folder of tnt, you can run

```sh
echo "source $(pwd)/extra/completions/tnt.bash" >> ~/.bashrc
```

Otherwise you can copy it to the `~/.bash_completion` folder and source it from there:

```sh
mkdir -p ~/.bash_completion
cp extra/completions/tnt.bash ~/.bash_completion/tnt
echo "source ~/.bash_completion/tnt" >> ~/.bashrc
```

#### Fish

To install the completions for fish, run

```
mkdir -p $fish_complete_path[1]
cp extra/completions/tnt.fish $fish_complete_path[1]/tnt.fish
```
