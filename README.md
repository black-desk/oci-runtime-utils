# OCI runtime utilities

A simple command line tool contains some utilities for
testing, benchmarking and analysing behaviors of
OCI runtime CLI implementations.

## Usage

Check [help.md](./docs/help.md).

## Build

This program should be statically linked,
as it going to be executed in minimal container environment
in where even `libc` is unavailable.

So you should build to `*-linux-musl` target.

### x86_64

```bash
cargo build --target x86_64-unknown-linux-musl
```

### aarch64

```bash
cargo build --target aarch64-unknown-linux-musl
```

### loongarch64

```bash
cargo build --target loongarch64-unknown-linux-musl
```
