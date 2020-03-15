# dotfiles-rs [![badge](https://action-badges.now.sh/jokeyrhyme/dotfiles-rs)](https://github.com/jokeyrhyme/dotfiles-rs/actions)

read my dotfiles repository and do stuff

## Status

- this project is winding down in favour of [`tuning`](https://github.com/jokeyrhyme/tuning) as configured by [my dotfiles](https://github.com/jokeyrhyme/dotfiles/tree/master/tuning)

- ideally, we will take the imperative code here and expose it as configuration-driven functionality over in `tuning`

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

- `jokeyrhyme-dotfiles all`:

  - copies or creates symbolic links from `~/.dotfiles` into `~` for settings

  - otherwise creates and/or edits settings files in `~`

  - Windows: ensures ~/bin exists in PATH

  - configures [npm-merge-driver](https://www.npmjs.com/package/npm-merge-driver) when possible

  - reads from TOML file and (un)installs/updates desired [Atom](https://atom.io/) packages

  - reads from TOML file and (un)installs/updates desired [`npm`](https://www.npmjs.com/) packages

  - reads from TOML file and installs/updates desired [rust](https://www.rust-lang.org/) [crates](https://crates.io/)

  - reads from TOML file and (un)installs/updates desired [Visual Studio Code](https://code.visualstudio.com) extensions

  - (un)installs/updates [tpm](https://github.com/tmux-plugins/tpm) and desired tmux plugins

  - (un)installs/updates vim-plug and desired vim / [neovim](https://github.com/neovim/neovim) plugins

- installs and updates the following tools:

  - [atlantis](https://github.com/runatlantis/atlantis)
  - [bazel](https://github.com/bazelbuild/bazel)
  - [bash-it](https://github.com/Bash-it/bash-it)
  - [dep](https://github.com/golang/dep)
  - [git-sizer](https://github.com/github/git-sizer)
  - [gitleaks](https://github.com/zricethezav/gitleaks)
  - [golang](https://golang.org/)
  - [jq](https://github.com/stedolan/jq)
  - [hadolint](https://github.com/hadolint/hadolint)
  - [minikube](https://github.com/kubernetes/minikube)
  - [nodejs](https://nodejs.org)
  - [oh-my-zsh](https://github.com/robbyrussell/oh-my-zsh)
  - [pure](https://github.com/sindresorhus/pure)
  - [shfmt](https://github.com/mvdan/sh)
  - [skaffold](https://github.com/GoogleCloudPlatform/skaffold)
  - [vale](https://github.com/errata-ai/vale)
  - [yq](https://github.com/mikefarah/yq)

## Configuration

### GITHUB_TOKEN

Generate a new [GitHub Personal Access Token](https://github.com/settings/tokens) without any extra permissions,
and set this as the value for the GITHUB_TOKEN environment variable.

This will reduce the likelihood of rate-limiting by GitHub's API.

## See Also

- https://github.com/jokeyrhyme/dotfiles

- https://github.com/jokeyrhyme/tuning
