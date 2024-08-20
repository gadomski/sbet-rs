# sbet-rs

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/gadomski/sbet-rs/ci.yml?branch=main&style=for-the-badge)](https://github.com/gadomski/sbet-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/sbet?style=for-the-badge)](https://crates.io/crates/sbet)
[![docs.rs](https://img.shields.io/docsrs/sbet?style=for-the-badge)](https://docs.rs/sbet/latest/sbet/)
![Crates.io](https://img.shields.io/crates/l/sbet?style=for-the-badge)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](./CODE_OF_CONDUCT)

Micro-crate to read and write Smoothed Best Estimate of Trajectory (SBET) files with Rust.

## Usage

This crate comes with an API and a CLI.

### API

Include **sbet** in your `Cargo.toml`:

```toml
[dependencies]
sbet = "0.1"
```

See [the documentation](https://docs.rs/sbet) for API docs.

### CLI

The Command-Line Interface (CLI) is gated behind the `cli` feature.
To install:

```shell
cargo install sbet -F cli
```

The CLI can fliter points or print an SBET file in CSV format.

```shell
sbet to-csv infile.sbet  # prints to standard output
sbet filter infile.sbet outfile.sbet --start-time 151631 --end-time 151700
```

## License

**sbet-rs** is dual-licensed under both the MIT license and the Apache license (Version 2.0).
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
