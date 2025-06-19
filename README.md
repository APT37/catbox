<h2>CLI and library wrapper for the <a href="https://catbox.moe/tools.php">Catbox API</a></h2>

See [FAQ][faq] for allowed filetypes and content.

*Please consider supporting Catbox.moe by donating through [Ko-fi][kofi] or by purchasing items from the [merch store][store] (also supports direct donations) to help with the server costs.*

- [Installation](#installation)
  - [AUR](#aur)
  - [Binaries](#binaries)
  - [From Source (with cargo)](#from-source-with-cargo)
- [Authentication](#authentication)
- [Commands](#commands)
- [Examples](#examples)
- [Library](#library)

## Installation

### AUR

Arch users may install one of our [AUR packages](AUR.md).

### Binaries

Pre-built binaries for Windows and Linux are available for every [release][releases].

### From Source (with cargo)

```fish
# 1. build dependencies
sudo apt-get install git rustup

# 2. Rust toolchain
rustup default stable

# 3a. latest release (via crates.io)
cargo install catbox-ng

# 3b. latest commit (via GitHub)
cargo install --git=https://github.com/APT37/catbox
```

## Authentication

Some commands require a authentication (via [userhash][manage]). This can be set using both the `CATBOX_USER_HASH` environment variable or the `--user` argument.

The `--user` argument takes precedence over the environment variable.

*Uploads without a userhash are anonymous.*

## Commands

All successful commands return the verbatim server response, usually a link to the given file or album, or an error.

`catbox <cmd>`

- `upload` local files or URLs (authentication optional)
- `delete` files
- `album` see below
- `litter` see examples

The `album` subcommand has additional subcommands:

`catbox album <album_cmd>`

- `create` a new album (authentication optional)
- `delete` an existing album
- `edit` an existing album
- `add` files to an existing album
- `remove` files from an existing album

## Examples

Upload a file:

```
catbox upload cute_picture.png
```

Upload multiple files:

```
catbox upload *.jpg  # Upload all jpg files
catbox upload image.png file.txt  # Upload image.png and file.txt
```

Delete a file:

```
catbox delete abc123.jpg --user 1234567890123456789012345
catbox delete https://files.catbox.moe/123456.png  # Or just 123456.png
```

Create an album:

```
catbox album create --title 'My album' --desc 'An excellent album' abc123.jpg def456.png
```


Upload a file to Litterbox for 3 days:

```
catbox litter --time 72h homework.zip
```

## Library

You can compile and install the command line tool with Cargo:
```
cargo install catbox-ng
```

To use the library in your project, add the repo to your Cargo.toml:

```
[dependencies]
catbox-ng = "*"
```

<!-- link definitions -->

[tools]: https://catbox.moe/tools.php
[faq]: https://catbox.moe/faq.php
[kofi]: https://ko-fi.com/catboxmoe
[store]: https://store.catbox.moe/
[manage]: https://catbox.moe/user/manage.php

[releases]: https://github.com/APT/catbox/releases
