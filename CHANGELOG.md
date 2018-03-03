# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
