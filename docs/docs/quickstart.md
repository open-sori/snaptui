---
sidebar_position: 1
title: Quickstart
---

# Quickstart

This guide will help you get started with `snaptui`.

## Download

You can download the latest release of `snaptui` from the [GitHub Releases page](https://github.com/open-sori/snaptui/releases).

## Build from Source

To build `snaptui` from source, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.

1. Clone the repository:

    ```bash
    git clone https://github.com/open-sori/snaptui.git
    cd snaptui
    ```

2. Build the project in release mode:

    ```bash
    cargo build --release
    ```

## Usage

After building, you can run `snaptui` from the `target/release` directory:

```bash
./target/release/snaptui --host 127.0.0.1 --port 1780
```

Replace `127.0.0.1` and `1780` with your Mumble server's IP address and port.
