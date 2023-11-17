use std::io::Cursor;

use rustagram::image::io::Reader;
use rustagram::image::ImageOutputFormat;
use rustagram::{FilterType, RustagramFilter};

pub fn apply_filter(img: &[u8], filter: FilterType) -> Vec<u8> {
    log::debug!("image: {} bytes, filter: {:?}", img.len(), filter);

    let cursor = Cursor::new(img);
    let img = Reader::new(cursor)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let out = img.to_rgba8().apply_filter(filter);

    let mut bytes: Vec<u8> = Vec::new();
    let mut write_cursor = Cursor::new(&mut bytes);
    out.write_to(&mut write_cursor, ImageOutputFormat::Png)
        .unwrap();

    bytes
}

