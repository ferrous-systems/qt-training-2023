# Final application

You should have this file tree layout:

```console
$ tree
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```

To recap your final code should look something like this:

``` rust
{{#include ../../../../crates/cli/src/main.rs}}
```

You can build your code like this:

```
cargo build
```

And run it using cargo:

```
cargo run
```

For an optimized build use:

```
cargo build --release
```

```
cargo run --release
```

---

Some ideas on what to do next:

* Run the application natively: `cargo run`. Any complications or differences?
* Heard of WebAssembly? You can actually run this in WebAssembly - see our [WASM Training](https://github.com/ferrous-systems/wasm-training-2022)
* Try a full command line parser crate - see [blessed.rs for suggestions](https://blessed.rs/crates#section-cli-tools-subsection-argument-parsing)