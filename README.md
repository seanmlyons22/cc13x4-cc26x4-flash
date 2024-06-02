# Flash Algorithm for cc13x4_cc26x4

This is a flash algorithm for the TI cc13x4_cc26x4 devices. It is intended
for use with `probe-rs`, but since it follows the CMSIS-PACK standard it could
also be used with other debuggers such as PyOCD. That is not tested.

![CI](https://github.com/seanmlyons22/cc13x4-cc26x4-flash/actions/workflows/ci.yml/badge.svg)[![chat](https://img.shields.io/badge/chat-probe--rs%3Amatrix.org-brightgreen)](https://matrix.to/#/#probe-rs:matrix.org)

## Dependencies

Run the following requirements:

```bash
cargo install cargo-generate cargo-binutils target-gen
rustup component add llvm-tools-preview
```

Ensure you have the right toolchain:

```
rustup target add thumbv8m.main-none-eabihf
```

## Setting up sub-modules

This template uses TI's driverlib functions to access the flash.
This is battle tested by TI, so no need to rewrite the lowest level access in
Rust. Instead, we invoke the driverlib functions using the FFI.

To do that, we need to get TI's driverlib. For that, we use TI's SDK
on github. We consume that as a submodule.

For first time clone:

```
git clone --recurse-submodules <repo name>
```

If you have have an existing checkout (and forgot to run with recurse submodules):

```
git submodule update --init --recursive
```

## Build Script

This repo uses a build script to copy driverlib from TI's git based SDK into
the `target` dir. It also renames the library to match LLD's search expectations.

See `build.rs` for more. This runs on the host machine when `cargo build` is run.

## Developing the algorithm

Just run `cargo run`. It spits out the flash algo in the probe-rs YAML format and downloads it onto a target and makes a test run.
You will also be able to see RTT messages.

You can find the generated YAML in `target/definition.yaml`.

# License

This is licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
