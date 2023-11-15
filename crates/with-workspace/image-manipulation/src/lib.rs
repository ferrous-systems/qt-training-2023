use std::io::Cursor;

use rustagram::image::io::Reader;
use rustagram::image::ImageOutputFormat;
use rustagram::{RustagramFilter, FilterType};

pub fn apply_filter(img: &[u8], filter: FilterType) -> Vec<u8> {
    log::debug!("image: {} bytes, filter: {:?}", img.len(), filter);

    let img = Reader::new(Cursor::new(img))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

        let out = img.to_rgba8().apply_filter(filter);
    let mut bytes: Vec<u8> = Vec::new();
    out.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)
        .unwrap();

    bytes
}