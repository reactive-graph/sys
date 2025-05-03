# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

- Refactored into new mono repo `reactive-graph/sys`
- Moved plugins binary, config, file and system-environment from `reactive-graph/std` -> `reactive-graph/sys`
- Prefix plugins with `sys` (e.g. `libreactive_graph_sys_binary`)
- Refactored usages of `std` to its new namespace
- Configure lints on workspace level

### Fixed

### Distribution

### Infrastructure

- CI: Synchronize labels from config file
- CI: Automatically merge successful dependabot PR's
