# BadOS Shell System

Bad Technologies' BadOS Shell System, developed in Rust.

## Compilation

### Cargo

BadOS Shell System must be built using the `nightly` branches of Cargo and Rust. Set this project to use `nightly` with the following command:

```shell
rustup override set nightly
```

### Compilation Target

Use the `x84_64-bad_os` compilation target. You can use this with Cargo using the following command:

```shell
cargo build --target x84_64-bad_os.json
```

Copyright (c) 2024 Logan Dhillon.
This software is subject to the Bad Technologies Open Software License. See [LICENSE](LICENSE) for more.