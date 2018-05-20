# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.0] - 2018-05-20

### Added

* pkg: tmux: `sync` installs [tpm](https://github.com/tmux-plugins/tpm)

### Fixed

* pkg: git: fix installation of yarn merge driver

## [0.13.0] - 2018-05-17

### Added

* pkg: ssh: `sync` merges ~/.dotfiles/config/ssh into ~/.ssh/config

* pkg: ssh: `sync` blacklists weak ciphers / algorithms

### Fixed

* pkg: ssh: deterministic Host and Match section order

* pkg: vscode: fix copy-pasta with macOS xattr fix

## [0.12.0] - 2018-05-10

### Added

* pkg: [golang](https://golang.org/): `sync` and `update`

* pkg: vscode: macOS: added a work-around to fix app auto-update

### Changed

* update dependencies

* HTTP request logs no longer include query string or fragment

### Fixed

* os: Windows: replace `%PATH%` checks with static instructions

## [0.11.0] - 2018-04-26

### Added

* pkg: [jq](https://github.com/stedolan/jq): `sync` and `update`

### Fixed

* pkg: nodejs: macOS: fix OS mappings

* pkg: nodejs: Windows: handle path differences

* pkg: nodejs: install/update NPM packages after updating Node.js

* slightly better error handling

## [0.10.0] - 2018-04-22

### Added

* pkg: [nodejs](https://nodejs.org): `sync` and `update` install the latest version of Node.js on Linux, macOS, and Windows

* pkg: nodejs: `sync` enables metrics in `npm`

### Fixed

* Windows: fix the build again :S

* pkg: dep,shfmt,skaffold,yq: Windows needs .exe extension

## [0.9.0] - 2018-04-19

### Added

* pkg: [dep](https://github.com/golang/dep): `sync` (into ~/.local/bin) and `update`

### Fixed

* `utils::http::fetch_request()` follows redirects

## [0.8.0] - 2018-04-17

### Added

* pkg: [yq](https://github.com/mikefarah/yq): `sync` (into ~/.local/bin) and `update`

### Changed

* install binaries to ~/.local/bin, expect it to be in PATH

### Fixed

* ensure target directory exists when downloading files

* pkg: shfmt,skaffold: fix macOS asset detection

## [0.7.0] - 2018-04-11

### Added

* Windows: ensure ~/bin exists in PATH

* pkg: [shfmt](https://github.com/mvdan/sh): `sync` (into ~/bin) and `update`

* pkg: [skaffold](https://github.com/GoogleCloudPlatform/skaffold): `sync` (into ~/bin) and `update`

* pkg: tmux: `sync` and `update` install, clean, and update [tpm plugins](https://github.com/tmux-plugins/tpm/blob/master/docs/managing_plugins_via_cmd_line.md)

## [0.6.1] - 2018-03-14

### Fixed

* pkg: nodejs: properly identify installed packages

* pkg: vim: Windows: \_vimrc instead of .vimrc

## [0.6.0] - 2018-03-10

### Added

* pkg: git: configure [npm-merge-driver](https://www.npmjs.com/package/npm-merge-driver) when possible

* pkg: nodejs: `sync` will (un)install `npm` packages as listed in TOML

* pkg: nodejs: `update` will update `npm` and global packages

* pkg: vim: `sync` will (un)install vim-plug and symlink .vimrc

* pkg: vim: `update` will update vim-plug and plugins

### Fixed

* skip symlinking when desired link already exists

## [0.5.0] - 2018-03-07

### Added

* pkg: atom: `sync` can now disable packages listed in TOML

* pkg: dotfiles: `sync` calls `git pull` in ~/.dotfiles

## [0.4.1] - 2018-03-04

### Fixed

* pkg: atom: fix `apm install` bug caused by accidental whitespace

## [0.4.0] - 2018-03-03

### Added

* pkg: atom: implement `sync` and `update`

### Fixed

* pkg: vscode: `update` now works on Windows

* pkg: vscode: link settings in the correct macOS-specific place

* pkg: vscode: use the correct settings path on Windows

## [0.3.1] - 2018-02-26

### Fixed

* pkg: vscode: check for "code.cmd" on Windows

## [0.3.0] - 2018-02-25

### Added

* `sync` now reads from a TOML file and installs / uninstalls desired Visual Studio Code extensions

### Fixed

* fix attribute usage so that Windows can successfully build

## [0.2.0] - 2018-02-22

### Added

* `sync` now reads a list of crates from a TOML file and installs missing crates with `cargo install`

* add `update` command

* `update` now updates Rust with `rustup`

* `update` now updates crates installed by `cargo install`

### Changed

* use values from Cargo.toml for `--help` and `--version`

### Fixed

* `sync` now actually avoids installing desired crates that already exist

* `update` now actually avoids updated installed crates that are already latest

## [0.1.2] - 2018-02-18

### Changed

* add missing package metadata to fix `cargo publish`

## [0.1.1] - 2018-02-18

### Changed

* nothing changed, bumped version to test builds
