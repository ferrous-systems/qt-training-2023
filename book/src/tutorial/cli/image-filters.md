# Image filter application

Now that you can build and run an application compiled to WebAssembly,
it's time to build some functionality into it.

The goal is:

* Take an input file, a filter name and, optionally, an output file (or "output.jpg" as the default).
* Load the input file, apply the given filter to this image, then write the resulting image to the output file.

You can continue with the previously created project.

✅ Open `src/main.rs` again and replace the `println!` line with code to parse the arguments.

```rust
fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next().expect("INPUT required");
    let filter = args.next().expect("FILTER required");
    let output = args.next().unwrap_or_else(|| "output.jpg".to_string());

    dbg!((input, filter, output));
}
```

✅ Build and run this to make sure it works as expected.

✅ Now add a dependency to handle image manipulation. The image filters are readily available in the [`rustagram2`](https://crates.io/crates/rustagram2) crate.
Add the `rustagram2` crate as a dependency in `rustagram/Cargo.toml`

```toml
[dependencies]
rustagram2 = "2.0.0"
```

The documentation is available on [docs.rs/rustagram2](https://docs.rs/rustagram2/2.0.0/rustagram/).

✅ You need a [`FilterType`](https://docs.rs/rustagram2/2.0.0/rustagram/enum.FilterType.html) to apply later.
`rustagram2` shows the available filters [in the `FilterType` documentation](https://docs.rs/rustagram2/2.0.0/rustagram/enum.FilterType.html).
It also has [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) from the standard library implemented for it, so you can parse strings into the filter type by calling `parse()` on the string.

```rust
let filter_type = filter.parse().expect("can't parse filter name");
```

An unknown filter name would cause an error.
For now you don't need to handle that.
Your application can just panic and exit.

If you compile everything at this point you will probably hit a type annotation error.
You can try to resolve that now.
You can also continue and observe how this error will be resolved once you add more code in the next steps.

Now comes the main part of the application: load the image, apply the filter and save the resulting file.
This is a small challenge for you to write, but the next steps guide you through it.

✅ You need to read the file from disk and turn it into an object you can work with.
[`image::open`](https://docs.rs/image/0.24.4/image/fn.open.html) does that for you easily.
Don't worry about error handling and just `unwrap`.

✅ The image type you get is able to represent a wide variety of image types.
For this tutorial you want an [`RgbaImage`](https://docs.rs/image/0.24.4/image/type.RgbaImage.html). You can convert your image using the [`to_rgba8`](https://docs.rs/image/0.24.4/image/enum.DynamicImage.html#method.to_rgba8) method.

✅ Last but not least you need to apply the selected filter on this image.
The `rustagram2` crate implements that as the [`apply_filter`](https://docs.rs/rustagram2/2.0.0/rustagram/trait.RustagramFilter.html#tymethod.apply_filter) method on a trait.
This trait is automatically implemented for the `RgbaImage` type you got from `to_rgba8`.

✅ Save back to the file `output` by using the [`save`](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.save
) method available on the image.

With the help of the documentation this should be achievable in a couple of lines of code.

Try it for yourself!

✅ Once you wrote the code, build it again and try to run it.

Expected output when you don't pass any arguments:

```console
$ cargo run
thread 'main' panicked at src\main.rs:5:29:
INPUT required
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Expected output when you pass a non-existing file path and a filter name:

```console
$ cargo run -- missing-image.jpg 1977
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target\debug\rustagram.exe kongelige-slott.jpg 1977`
thread 'main' panicked at src\main.rs:10:34:
called `Result::unwrap()` on an `Err` value: IoError(Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Expected output when you pass a file that exists.

```console
$ cargo run -- kongelige-slott.jpg 1977
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target\debug\rustagram.exe kongelige-slott.jpg 1977`
```

The result should then be stored in `output.jpg`.