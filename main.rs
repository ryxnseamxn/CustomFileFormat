use image::{save_buffer, GenericImageView, ImageBuffer, ImageReader, Rgb};
use std::io::Cursor; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("gavarni.jpg")?.decode()?;

    let (width, height) = img.dimensions(); 
    let raw_pixels = img.to_rgb8().into_raw(); 
    let buffer = ImageBuffer::from_raw(width, height, raw_pixels).expect("failed to create"); 
    print!("{:?}", img);
    Ok(())
}
