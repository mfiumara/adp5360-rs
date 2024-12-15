# ADP5360 Rust Driver

[![Rust](https://github.com/mfiumara/adp5360-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mfiumara/adp5360-rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/mfiumara/adp5360-rs/graph/badge.svg?token=SJPT9NBXCQ)](https://codecov.io/gh/mfiumara/adp5360-rs)

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
adp5360 = "0.2.1"
```

## Usage

See the [examples](examples) directory for usage examples.
