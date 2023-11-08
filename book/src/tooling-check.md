# Tooling check

## Setup check

✅ Fully restart your terminal (not just open a fresh tab).

✅ Let's check that you have installed Rust.

```console
$ rustc --version
rustc 1.73.0 (cc66ad468 2023-10-03)
```

```console
$ cargo --version
cargo 1.73.0 (9c4383fb5 2023-08-26)
```

✅ In a work directory, run:

```console
$ cargo new --bin hello-world
$ cd hello-world
$ cargo run --release
   Compiling hello-world v0.1.0 (C:\Code\ferrous\hello-world)
    Finished release [optimized] target(s) in 0.99s
     Running `target\release\hello-world.exe`
Hello, world!
```

This ensures that the whole toolchain works correctly and finds the system linker. This should work at all times, if it doesn't, immediately ask for a trainer.

# Qt specifics

TODO tool check for Qt, compiling a hello world app.
