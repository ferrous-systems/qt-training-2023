# Setup

This section describes how to set up the toolchain
for compiling Rust programs to WebAssembly
and integrate them with the different environments we will look at.

## The Rust Toolchain

You will need the standard Rust toolchain, including `rustup`, `rustc`, and
`cargo`.

[Follow these instructions to install the Rust toolchain.][rust-install]

This will install the Rust toolchain and the targets for the host platform.
Windows users need to choose between the `msvc` and `mingw` targets.
We recommend the `msvc` target, unless you already have `mingw` installed.

The course will only use the stable compiler, we don't require any experimental feature flags.
The latest Rust should work best, though older versions have no known issues.

[rust-install]: https://www.rust-lang.org/tools/install

### Additional tooling

For those interested in following the Qt section of the course,
installation of Qt is needed.

TODO: fill Qt installation instructions


