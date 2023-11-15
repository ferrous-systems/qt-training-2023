use image_manipulation::apply_filter;
use rustagram::FilterType;

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next().expect("INPUT required");
    let filter = args.next().expect("FILTER required");
    let output = args.next().unwrap_or_else(|| "output.jpg".to_string());

    let filter_type: FilterType = filter.parse().expect("can't parse filter name");

    let bytes: Vec<u8> = std::fs::read(input).unwrap();

    let manipulated_image = apply_filter(&bytes, filter_type);

    std::fs::write(output, manipulated_image).unwrap();
}
