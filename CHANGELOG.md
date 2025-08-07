# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

### Fixed

### Distribution

### Infrastructure

## [0.10.0-alpha-1] - 2025-08-07

### Added

- Documentation: Added Book
- Type System: Added JSON schema identifier to all types

### Changed

- Refactored into new mono repo `reactive-graph/sys`
- Moved plugins binary, config, file and system-environment from `reactive-graph/std` -> `reactive-graph/sys`
- Prefix plugins with `sys` (e.g. `libreactive_graph_sys_binary`)
- Refactored usages of `std` to its new namespace
- Configure lints on workspace level
- Replaced lazy_static with LazyLock
- Build: Bump MSRV 1.85 -> 1.87
- Upgrade log4rs to 1.4.0

### Fixed

### Distribution

- CI: Use ubuntu-22.04 instead of ubuntu-20.04 for building debian packages
- CI: Publish dynamic link libraries as compressed archive
- CI: Publish debian packages

### Infrastructure

- CI: Synchronize labels from config file
- CI: Automatically merge successful dependabot PR's
- CI: Update rust nightly toolchain to nightly-2025-08-05
