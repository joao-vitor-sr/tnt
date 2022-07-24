# tnt

`tnt` is a program to manage notes in your terminal. It uses files in the
[Markdown](https://en.wikipedia.org/wiki/Markdown) format so it's possible
to sync the files between devices

Quick links

- [How to use](#how-to-use)
- [Installation](#installation)

## Demo

![Demo](docs/screenshot.svg)

## How to use

First, to get an overview of all available command line options, you can either run
[`tnt -h`](#command-line-options) for a concise help message or `man tnt` for a more detailed
version.

#### Examples

Create a note

```bash
tnt note1
```

If a note already exists it will edit the note using the default $EDITOR

Remove a note

```bash
tnt note1 -r
```

Find all notes

```bash
tnt
```

Note that to list all notes no arguments are necessary

### Command-line options

This is the output of `tnt -h`. To see the full set of command-line options, use `man tnt` which
also includes a much more detailed help text.

```
USAGE:
    tnt [NOTE] <args>
FLAGS:
    -h, --help              Prints help information
    -v, --version           Prints version information
    -r, --remove            Removes the note
```

## Installation

_for now the tnt has no installer, it will be added in later releases_
