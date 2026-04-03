# 🎨 kanumi

A CLI to manage collection of images.

![demo](https://github.com/user-attachments/assets/a0a83011-d028-400b-930e-f7f6826d4342)

[![Crates info](https://img.shields.io/crates/v/kanumi.svg)](https://crates.io/crates/kanumi)
[![License: GPL-3.0](https://img.shields.io/github/license/coko7/kanumi?color=blue)](LICENSE)
[![Number of Stars](https://img.shields.io/github/stars/coko7/kanumi.svg?style=flat&logo=github&colorB=green&label=stars)](https://github.com/coko7/kanumi/stargazers)
![Rust](https://img.shields.io/github/languages/top/coko7/kanumi?color=orange)

## What is it?

I have lots of wallpapers on my computer and I wanted a way to easily filter them and pipe them to other scripts.
For example, I would filter based on some attributes, and send that to [`awww`](https://codeberg.org/LGFae/awww) to update my wallpaper.

Think of `kanumi` like the `find` command, but tailored for images with metadata.
For example, the following command will select all images at least 1920x1080 with a `favorite` score between 3 and 7.
```console
coko7@example:~$ kanumi list --width=1920.. --height=1080 --scores favorite=3..7
# shorter equivalent:
coko7@example:~$ kanumi list -W 1920.. -H 1080 -s favorite=3..7
```

## Installation

The easiest way to install is through [crates.io](https://crates.io/crates/kanumi):
```sh
cargo install kanumi
```

The other option is to build from source:
```sh
git clone https://github.com/coko7/kanumi.git
cd kanumi
cargo build --release
```

## Usage

kanumi has multiple main commands:
- [config](#config-command): view/manager kanumi configuration
- [metadata](#metadata-command): view/manage image metadatas
- [dirs](#dirs-command): list all dirs containing images
- [list](#list-command): list images that match given selectors
- [scan](#scan-command): scan for missing image/metadata

```console
coko7@example:~$ kanumi -h
kanumi 0.2.1

Manage collection of images from your terminal

Usage:
kanumi [OPTIONS] <COMMAND>

Commands:
  config    View and manage configuration
  metadata  View and manage metadata
  dirs      List all directories containing images
  list      List images that match given selectors
  scan      Scan the entire images directory to find missing data
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version

Made by @coko7 <contact@coko7.fr>
```

### ⚙️ `config` command

```console
coko7@example:~$ kanumi config --help
View and manage configuration

Usage: kanumi config [OPTIONS] <COMMAND>

Commands:
  show      Print configuration and exit
  generate  Generate a default configuration file [aliases: gen]
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

### ✨ `metadata` command

```console
coko7@example:~$ kanumi meta help
View and manage metadata

Usage: kanumi metadata [OPTIONS] <COMMAND>

Commands:
  show      Print all metadatas and exit
  get       Get the metadata associated to a given image file
  generate  Generate default metadata for a given image [aliases: gen]
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

### 📂 `dirs` command

```console
coko7@example:~$ kanumi dirs -h
List all directories containing images

Usage: kanumi dirs [OPTIONS]

Options:
  -j, --json        Output in JSON
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

### 🗒️ `list` command

```console
coko7@example:~$ kanumi ls --help
List images that match given selectors

Usage: kanumi list [OPTIONS]

Options:
  -d, --directories <ACTIVE_DIRECTORIES>  Filter based on parent directories
  -s, --scores <SCORES>                   Filter based on score range
  -W, --width <WIDTH_RANGE>               Filter based on width range
  -H, --height <HEIGHT_RANGE>             Filter based on height range
  -i, --ignore                            Ignore selectors preset from config
  -v, --verbose...                        Increase logging verbosity
  -j, --json                              Output in JSON
  -q, --quiet...                          Decrease logging verbosity
  -h, --help                              Print help
```

#### Examples

1. Select images with width >= 1920, height >= 1080, with a "favs" < 2
```console
coko7@example:~$ kanumi list --width=1920.. --height=1080.. --scores favs=0..1
coko7@example:~$ kanumi ls -W 1920.. -H 1080.. -s favs=..1
```

2. Select tiny images with a "simple" score of exactly 5:
```console
coko7@example:~$ kanumi ls -W ..50 -H ..50 -s favs=5
coko7@example:~$ kanumi ls -W ..50 -H 0..50 -s favs=5..5
coko7@example:~$ kanumi ls -W 0..50 -H ..50 -s favs=5..5
```

### 🔍 `scan` command

```console
coko7@example:~$ kanumi scan --help
Scan the entire images directory to find missing data

Usage: kanumi scan [OPTIONS]

Options:
  -j, --json        Output in JSON
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```
