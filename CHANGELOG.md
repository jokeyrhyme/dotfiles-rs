# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
