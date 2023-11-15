# Creating a library

In this section of the tutorial, you will create
a library to handle image manipulation. This library
will have exactly one function. It should:

* Take a field of bytes representing an image
* Take a filter type

It should not:

* Handle any I/O
* Do input parsing (like filter name detection, etc.)

✅ In `image-manipulation/Cargo.toml`, add:

```toml
[dependencies]
rustagram2 = "2"
log = "0.4"
```

We use the `log` crate to get some visibility into what is happening.

✅ In `image-manipulation/src/lib.rs`, add the following imports and function headers

```rust
use std::io::Cursor;

use rustagram::image::io::Reader;
use rustagram::image::ImageOutputFormat;
use rustagram::{RustagramFilter, FilterType};

pub fn apply_filter(img: &[u8], filter: FilterType) -> Vec<u8> {
    log::debug!("image: {} bytes, filter: {:?}", img.len(), filter);
}
```

This communicates three interesting things:

* The function is marked `pub` - this makes it a public function, callable from other libraries. The default in Rust is always private.
* The data of the image is an immutable reference to binary data owned outside of the function
* The result is a binary vector, but passed out owned - so it cannot be the initial data

## Reading the image

Looking at the signature of [Reader](https://docs.rs/image/0.24.3/image/io/struct.Reader.html), we find that it can only be construted using types that implement the `Read` trait. `&[u8]` does not implement the `Read` trait. However, we can use the
standard libraries [Cursor](https://doc.rust-lang.org/std/io/struct.Cursor.html) type
to fix that.

✅ Read the image by inserting the following code

```rust
let read_cursor = Cursor::new(img);
let img = Reader::new(cursor)
    .with_guessed_format()
    .unwrap()
    .decode()
    .unwrap();
```

✅ Apply the filter

```rust
let out = img.to_rgba8().apply_filter(filter);
```

✅ Write to an output buffer

```rust
let mut bytes: Vec<u8> = Vec::new();
let mut write_cursor = Cursor::new(&mut bytes);
out.write_to(&mut write_cursor, ImageOutputFormat::Png)
    .unwrap();

bytes
```




---


Some ideas on what to do next:

* Create further utility functions