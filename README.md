# CLI and library wrapper for the [catbox API](https://catbox.moe/tools.php)

See [FAQ](https://catbox.moe/faq.php) for allowed filetypes/content.

*Please consider supporting catbox.moe by donating through [Ko-fi](https://ko-fi.com/catboxmoe) or by purchasing items from the [merch store](https://store.catbox.moe/) (also supports direct donations) to help with the server costs.*

<!-- For the library documentation, see [Github Pages](https://apt37.github.io/catbox/catbox/index.html). -->

## Installation

See [Releases](https://github.com/APT/catbox/releases) for the latest Windows and Linux builds of the command line tool.

Arch users may install via the [AUR](/AUR.md).

## Authentication

Some commands require a authentication (via [userhash](https://catbox.moe/user/manage.php)). This can be set using both the `CATBOX_USER_HASH` environment variable or the `--user` argument.

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

<!-- ### Library -->

<!-- You can compile and install the command line tool with Cargo:
```
cargo install catbox
```

To use the library in your project, add the repo to your Cargo.toml:

```
[dependencies]
catbox = "*"
``` -->
