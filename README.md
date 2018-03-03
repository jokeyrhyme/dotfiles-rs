# dotfiles-rs [![Build Status](https://travis-ci.org/jokeyrhyme/dotfiles-rs.svg?branch=master)](https://travis-ci.org/jokeyrhyme/dotfiles-rs)

read my dotfiles repository and do stuff

## Usage

```sh
$ cargo install jokeyrhyme-dotfiles
$ jokeyrhyme-dotfiles sync && jokeyrhyme-dotfiles update
```

Note that you need to "Run As Administrator" in Windows so that this tool can properly create symbolic links

## Features

* `jokeyrhyme-dotfiles sync`:

  * copies or creates symbolic links from `~/.dotfiles` into `~` for settings

  * otherwise creates and/or edits settings files in `~`

  * reads from TOML file and installs desired rust crates

  * reads from TOML file and installs desired Visual Studio Code extensions

* `jokeyrhyme-dotfiles update`:

  * updates rust and installed rust crates

## Roadmap

* [x] automatically build and publish for new git tags

## See Also

* https://github.com/jokeyrhyme/dotfiles
