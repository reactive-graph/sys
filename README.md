<div align="center">
  <a href="https://www,reactive-graph.io/"><img src="https://raw.githubusercontent.com/reactive-graph/design/main/public/logo/rendered/malachite/reactive-graph-400x400.png" alt="Reactive Graph"></a>
</div>

<h2 align="center">
    <a href="https://sys.reactive-graph.io/">sys.reactive-graph.io</a>
</h2>

<p align="center">
This repository contains the system library for the <a href="https://github.com/reactive-graph/reactive-graph">Reactive Graph</a>.
</p>

<p align="center">
  <a href="https://github.com/reactive-graph/reactive-graph">Reactive Graph</a> is a <b>reactive runtime</b> based on a <b>graph database</b>, empowering everyone to build reliable and efficient software.
</p>

<hr>

<div align="center" style="text-align: center">

[<img src="https://img.shields.io/badge/book-master-yellow">](https://docs.reactive-graph.io/book/)
[<img src="https://img.shields.io/badge/api-master-yellow">](https://docs.reactive-graph.io/docs/)

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/license/reactive-graph/sys">](https://github.com/reactive-graph/sys/blob/main/LICENSE)

[![Build](https://github.com/reactive-graph/sys/actions/workflows/rust.yml/badge.svg)](https://github.com/reactive-graph/sys/actions/workflows/rust.yml)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

</div>

<hr>

<h2 align="center" style="text-align: center;">List of Plugins</h2>

In this repository you'll find plugins which interacts with the system it runs on.

| Name                                                         | Description                    |
|--------------------------------------------------------------|--------------------------------|
| [Binary](./plugins/binary/README.md)                         | Handles binary data            |
| [Config](./plugins/config/README.md)                         | Load configuration files       |
| [File](./plugins/file/README.md)                             | File representation            |
| [System Environment](./plugins/system-environment/README.md) | Provides environment variables |

<h2 align="center" style="text-align: center;">Local Build + Local Deployment</h2>

#### Setup deployment directory in `.deployment.toml`

```shell
target_dirs = [
  "../reactive-graph/plugins/deploy"
]
```

#### Install a specific plugin

```shell
cargo build
cargo post build --package=reactive-graph-sys-binary
```

#### Install all plugins of this repository at once

```shell
cargo build
cargo post build --package=deployment-all
```
