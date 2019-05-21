# Changelog

We document all notable changes to this project in this file.

This project adheres to [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- auto-skip pure GitHub Release `update` tasks when `sync` had to install something

- plex: keep [Plex (font)](https://github.com/IBM/plex) `sync`ed and `update`d

- sourcesanspro: keep [Source Sans Pro (font)](https://github.com/adobe-fonts/source-sans-pro) `sync`ed and `update`d

- sourceserifpro: keep [Source Serif Pro (font)](https://github.com/adobe-fonts/source-serif-pro) `sync`ed and `update`d

### Changed

- extracted out bashrc, inputrc, profile, zshrc tasks from bash and zsh tasks

- simplify/tweak task Status output a little

### Fixed

- fix Windows build in `set_executable` RE: PartialEq

## [0.31.0] - 2019-04-19

### Added

- brewbundle: use [`brew bundle`](https://github.com/Homebrew/homebrew-bundle) for non-Windows favourites

- brewfile: symlink ~/.dotfiles/config/Brewfile to [~/.Brewfile](https://github.com/Homebrew/homebrew-bundle)

- fira: keep [Fira (font)](https://github.com/mozilla/Fira) `sync`ed and `update`d

- overpass: keep [Overpass (font)](https://github.com/RedHatBrand/Overpass) `sync`ed and `update`d

- publicsans: keep [Public Sans (font)](https://github.com/uswds/public-sans) `sync`ed and `update`d

### Fixed

- parse possible-`null` "name" in GitHub Releases (#213)

## [0.30.0] - 2019-04-12

### Added

- new `some example,tasks` command to run specific tasks

- pip: `env` exports PATH for `site.USER_SITE` packages

- pip: read favourites from a TOML file, keep them `sync`ed and `update`d

## [0.29.0] - 2019-04-09

### Added

- git: synchronise settings from ~/.dotfiles/config/git.toml

### Removed

- rustc: no longer install "nightly" toolchain by default

### Fixed

- fccache: skip when unavailable

## [0.28.0] - 2019-03-05

### Added

- golang: `env` exports GOROOT and GOPATH when found

- fccache: `update` (rebuild) font cache

- firacode: keep [FiraCode (font)](https://github.com/tonsky/FiraCode) `sync`ed and `update`d

- hack: keep [Hack (font)](https://github.com/source-foundry/Hack) `sync`ed and `update`d

- hasklig: keep [Hasklig (font)](https://github.com/i-tu/Hasklig) `sync`ed and `update`d

- inter: keep [inter (font)](https://github.com/rsms/inter) `sync`ed and `update`d

### Fixed

- golang: Windows: (hopefully) fix bad download URL (#190)

- better error handling in GitHub Release tasks (#191)

## [0.27.0] - 2019-02-02

### Added

- brew: `env` exports PATH including [Homebrew](https://brew.sh) if found

- brew: keep [Homebrew](https://brew.sh) `sync`ed and `update`d (does not automatically install Homebrew yet)

### Changed

- little bit less concurrency for now

- less output when there aren't any errors

## [0.26.2] - 2018-12-28

### Fixed

- golang: properly avoid non-stable versions of `go`

## [0.26.1] - 2018-12-12

### Fixed

- macOS: builds again (probably), yay (fixes #159)

## [0.26.0] - 2018-12-09

### Added

- googlecloudsdk: keep [Google Cloud SDK](https://github.com/google-cloud-sdk/google-cloud-sdk) `sync`ed and `update`d (fixes #154)

### Changed

- check OS at runtime rather than at compile time in more places

### Fixed

- Windows: builds again (probably), yay

## [0.25.0] - 2018-12-02

### Added

- goget: support TOML list of packages to uninstall

### Changed

- goget: install/update with less verbosity

### Fixed

- git: reorder tasks to avoid race with node/npm tasks

## [0.24.0] - 2018-12-01

### Added

- rust: support TOML list of crates to uninstall

### Changed

- dotfiles, vim: run serially before everything else

- bash, zsh: don't `sync` or `update` on Windows

- golang, goget: run concurrently with other tasks

- nodejs npm: run concurrently with other tasks

### Fixed

- ssh: honour ControlMaster "auto" value

## [0.23.0] - 2018-11-25

### Added

- local: new task; add "~/.local/bin" to PATH given by `env` command

### Changed

- goget: no longer run `gometalinter --install`

- golang, nodejs, rustup: inform the PATH given by `env` command

- rust: extract out separate "rustup" task

- rust: extract out separate "rustc" task

- use values from `env` command for child processes

### Fixed

- ssh: support "TIME FORMAT" values besides raw integers (seconds)

## [0.22.2] - 2018-11-13

### Changed

- golang: extract out separate "goget" task for better flow / status

### Fixed

- golang: hunt for old `dep` in the correct directory

- npm: ensure global links for package executables

## [0.22.1] - 2018-11-07

### Fixed

- fixed build on macOS (again) :S

## [0.22.0] - 2018-11-05

### Added

- new `all` sub-command that both synchronises and updates everything

### Changed

- all tasks now output a typed `Status` result

- use colorful text when displaying `Status` output

- run GitHub Release tasks in parallel with all other tasks

- nodejs: extract out separate "npm" task for better flow / status

### Removed

- no longer display progress dots during archive extraction

- remove separate `sync` and `update` commands

## [0.21.0] - 2018-11-03

### Added

- atlantis: keep [atlantis](https://github.com/runatlantis/atlantis) `sync`ed and `update`d

- bazel: keep [bazel](https://github.com/bazelbuild/bazel) `sync`ed and `update`d

### Fixed

- ssh: properly parse and honour the `Hostname` directive in SSH settings

## [0.20.0] - 2018-10-09

### Added

- new `jokeyrhyme env` command to export generated environment variables

- vim: export EDITOR set to `nvim` (preferred) or `vim` as detected

- atom+nodejs: detect and configure path to Python 2.x executable

### Fixed

- gitleaks: use "--version" to determine version

- Windows: fix builds again (oops)

## [0.19.0] - 2018-08-26

### Added

- bash: keep [bash-it](https://github.com/Bash-it/bash-it) `sync`ed and `update`d

- zsh: keep [oh-my-zsh](https://github.com/robbyrussell/oh-my-zsh) `sync`ed and `update`d

- zsh: keep [pure](https://github.com/sindresorhus/pure) theme `sync`ed and `update`d

### Fixed

- minikube: parse `minikube version` output better

- filter out unstable versions (fixes #85)

## [0.18.0] -2018-07-28

### Added

- git-sizer: keep [git-sizer](https://github.com/github/git-sizer) `sync`ed and `update`d

- vale: keep [vale](https://github.com/errata-ai/vale) `sync`ed and `update`d

## [0.17.0] - 2018-07-20

### Added

- gitleaks: keep [gitleaks](https://github.com/zricethezav/gitleaks) `sync`ed and `update`d

- hadolint: keep [hadolint](https://github.com/hadolint/hadolint) `sync`ed and `update`d

- minikube: keep [minikube](https://github.com/kubernetes/minikube) `sync`ed and `update`d

- vim: keep [neovim](https://github.com/neovim/neovim) plugins and settings `sync`ed and `update`d if detected

## [0.16.0] - 2018-07-13

### Added

- rust: ensure `rustup` is managing `rustfmt`

- rust: ensure `cargo fmt` works

- rust: `update` will update `rustup` itself

### Fixed

- ssh: support cipher/etc-whitelist required by old OpenSSH

### Added

- macOS: wipe Quick Look cache during `sync`

## [0.15.1] - 2018-06-10

### Fixed

- nodejs: Windows: look for node.exe in the correct place

- ssh: create ~/.ssh if missing

## [0.15.0] - 2018-06-06

### Added

- golang: `sync` delete `dep` installed by `go get ...`

- golang: `sync`/`update` grab favourite packages listed in TOML

- golang: `sync`/`update` grab linters with [gometalinter](https://github.com/alecthomas/gometalinter)

### Changed

- drop "pkg: " prefix from log output, etc

- less verbose archive extraction

### Fixed

- fix `.unwrap()` panic when HTTP requests time out

## [0.14.0] - 2018-05-20

### Added

- tmux: `sync` installs [tpm](https://github.com/tmux-plugins/tpm)

### Fixed

- git: fix installation of yarn merge driver

## [0.13.0] - 2018-05-17

### Added

- ssh: `sync` merges ~/.dotfiles/config/ssh into ~/.ssh/config

- ssh: `sync` blacklists weak ciphers / algorithms

### Fixed

- ssh: deterministic declaration block order e.g. Match

- vscode: fix copy-pasta with macOS xattr fix

## [0.12.0] - 2018-05-10

### Added

- [golang](https://golang.org/): `sync` and `update`

- vscode: macOS: added a work-around to fix app auto-update

### Changed

- update dependencies

- HTTP request logs no longer include query string or fragment

### Fixed

- os: Windows: replace `%PATH%` checks with static instructions

## [0.11.0] - 2018-04-26

### Added

- [jq](https://github.com/stedolan/jq): `sync` and `update`

### Fixed

- nodejs: macOS: fix OS mappings

- nodejs: Windows: handle path differences

- nodejs: install/update NPM packages after updating Node.js

- slightly better error handling

## [0.10.0] - 2018-04-22

### Added

- [nodejs](https://nodejs.org): `sync` and `update` install the latest version of Node.js on Linux, macOS, and Windows

- nodejs: `sync` enables metrics in `npm`

### Fixed

- Windows: fix the build again :S

- dep,shfmt,skaffold,yq: Windows needs .exe extension

## [0.9.0] - 2018-04-19

### Added

- [dep](https://github.com/golang/dep): `sync` (into ~/.local/bin) and `update`

### Fixed

- `utils::http::fetch_request()` follows redirects

## [0.8.0] - 2018-04-17

### Added

- [yq](https://github.com/mikefarah/yq): `sync` (into ~/.local/bin) and `update`

### Changed

- install binaries to ~/.local/bin, expect it to be in PATH

### Fixed

- ensure target directory exists when downloading files

- shfmt,skaffold: fix macOS asset detection

## [0.7.0] - 2018-04-11

### Added

- Windows: ensure ~/bin exists in PATH

- [shfmt](https://github.com/mvdan/sh): `sync` (into ~/bin) and `update`

- [skaffold](https://github.com/GoogleCloudPlatform/skaffold): `sync` (into ~/bin) and `update`

- tmux: `sync` and `update` install, clean, and update [tpm plugins](https://github.com/tmux-plugins/tpm/blob/master/docs/managing_plugins_via_cmd_line.md)

## [0.6.1] - 2018-03-14

### Fixed

- nodejs: properly identify installed packages

- vim: Windows: \_vimrc instead of .vimrc

## [0.6.0] - 2018-03-10

### Added

- git: configure [npm-merge-driver](https://www.npmjs.com/package/npm-merge-driver) when possible

- nodejs: `sync` will (un)install `npm` packages as listed in TOML

- nodejs: `update` will update `npm` and global packages

- vim: `sync` will (un)install vim-plug and symlink .vimrc

- vim: `update` will update vim-plug and plugins

### Fixed

- skip symlinking when desired link already exists

## [0.5.0] - 2018-03-07

### Added

- atom: `sync` can now disable packages listed in TOML

- dotfiles: `sync` calls `git pull` in ~/.dotfiles

## [0.4.1] - 2018-03-04

### Fixed

- atom: fix `apm install` bug caused by accidental whitespace

## [0.4.0] - 2018-03-03

### Added

- atom: keep settings and packages `sync`ed and `update`d

### Fixed

- vscode: `update` now works on Windows

- vscode: link settings in the correct macOS-specific place

- vscode: use the correct settings path on Windows

## [0.3.1] - 2018-02-26

### Fixed

- vscode: check for "code.cmd" on Windows

## [0.3.0] - 2018-02-25

### Added

- `sync` now reads from a TOML file and installs / uninstalls desired Visual Studio Code extensions

### Fixed

- fix attribute usage so that Windows can build

## [0.2.0] - 2018-02-22

### Added

- `sync` now reads a list of crates from a TOML file and installs missing crates with `cargo install`

- add `update` command

- `update` now updates Rust with `rustup`

- `update` now updates crates installed by `cargo install`

### Changed

- use values from Cargo.toml for `--help` and `--version`

### Fixed

- `sync` now actually avoids installing desired crates that already exist

- `update` now actually avoids updated installed crates that are already latest

## [0.1.2] - 2018-02-18

### Changed

- add missing package metadata to fix `cargo publish`

## [0.1.1] - 2018-02-18

### Changed

- nothing changed, bumped version to test builds
