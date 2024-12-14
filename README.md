# ADP5360 Rust Driver

[![Rust](https://github.com/mfiumara/adp5360-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mfiumara/adp5360-rs/actions/workflows/rust.yml)

An async embedded-hal driver for the ADP5360 Power Management IC.

## Features

- Battery charging control
- Battery voltage reading
- Async I2C communication
- `no_std` compatible

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
adp536x = "0.1.0"
```

## Usage

See the [examples](examples) directory for usage examples.
