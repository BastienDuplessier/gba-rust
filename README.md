GBA Games Dev
--------------------

# Setup


## Install Rust
```shell
curl https://sh.rustup.rs -sSf | sh
```

## Configure Rust
```shell
rustup toolchain install nightly
rustup default nightly
rustup target add arm-unknown-linux-gnueabi
```

## Install Xargo
```shell
rustup component add rust-src
cargo install xargo
```

# Build a ROM

```shell
cd rom-example
make
```

Generated rom will be named `out/game.gba`

# Misc

> Base was taken form this github : https://github.com/tbelaire/rusty-TONC