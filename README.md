# dotfiles-rs [![Build Status](https://travis-ci.org/jokeyrhyme/dotfiles-rs.svg?branch=master)](https://travis-ci.org/jokeyrhyme/dotfiles-rs)

read my dotfiles repository and do stuff

## Usage

```sh
$ cargo install jokeyrhyme-dotfiles
$ jokeyrhyme-dotfiles sync && jokeyrhyme-dotfiles update
```

Note that you need to "Run As Administrator" in Windows so that this tool can properly create symbolic links

## Features

- `jokeyrhyme-dotfiles env`:

  - export generated environment variables

  - EDITOR: `nvim` (preferred), `vim`

- `jokeyrhyme-dotfiles sync`:

  - copies or creates symbolic links from `~/.dotfiles` into `~` for settings

  - otherwise creates and/or edits settings files in `~`

  - Windows: ensures ~/bin exists in PATH

  - configures [npm-merge-driver](https://www.npmjs.com/package/npm-merge-driver) when possible

  - reads from TOML file and (un)installs desired [Atom](https://atom.io/) packages

  - reads from TOML file and (un)installs desired [`npm`](https://www.npmjs.com/) packages

  - reads from TOML file and installs desired [rust](https://www.rust-lang.org/) [crates](https://crates.io/)

  - reads from TOML file and (un)installs desired [Visual Studio Code](https://code.visualstudio.com) extensions

  - (un)installs vim-plug and desired vim plugins

- `jokeyrhyme-dotfiles update`:

  - updates installed Atom packages

  - updates installed `npm` packages

  - updates rust and installed rust crates

  - updates vim-plug and vim plugins

- installs and updates the following tools:

  - [shfmt](https://github.com/mvdan/sh)

  - [skaffold](https://github.com/GoogleCloudPlatform/skaffold)

  - [yq](https://github.com/mikefarah/yq)

## Configuration

### GITHUB_TOKEN

Generate a new [GitHub Personal Access Token](https://github.com/settings/tokens) without any special permissions,
and set this as the value for the GITHUB_TOKEN environment variable.

This will reduce the likelihood of rate-limiting by GitHub's API.

## Roadmap

- [x] automatically build and publish for new git tags

## See Also

- https://github.com/jokeyrhyme/dotfiles
