use image::{save_buffer, GenericImageView, ImageBuffer, ImageReader, Rgb};
use std::{io::Cursor, os::macos::raw}; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("gavarni.jpg")?.decode()?;

    let (width, height) = img.dimensions(); 
    let raw_pixels = img.to_rgb8().into_raw(); 
    let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, raw_pixels).expect("failed to create"); 
    for pixel in buffer.iter() {
        println!("{:?}", pixel); 
    }
    Ok(())
}
