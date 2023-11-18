# Creating a CLI using the library

Let's now use the library we created in our small CLI app.

✅ In `cli/Cargo.toml`, add:

```toml
[dependencies]
rustagram2 = "2"
image-manipulation = { path = "../image-manipulation" }
```

✅ Recreate parts of the main function:

```rust
use image_manipulation::apply_filter;
use rustagram::FilterType;

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next().expect("INPUT required");
    let filter = args.next().expect("FILTER required");
    let output = args.next().unwrap_or_else(|| "output.jpg".to_string());

    let filter_type: FilterType = filter.parse().expect("unknown filter");
}
```

As our image manipulation library does not handle IO, we will reach for the `std::fs::{read,write}` function.

✅ Read the image as input

```rust
let bytes: Vec<u8> = std::fs::read(input).unwrap();
```

The type annotation here isn't necessary and only for explanation.

✅ Call the `apply_filter` library function

```rust
let manipulated_image = apply_filter(&bytes, filter_type);
```

Note the referencing sigil in front of `bytes`.

✅ Write the output to disk

```rust
std::fs::write(output, manipulated_image).unwrap();
```
---

Some ideas on what to do next:

* Instead of unwrapping all errors, handle `Result` otherwise
