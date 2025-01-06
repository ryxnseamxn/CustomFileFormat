use std::io::Cursor;
use image::ImageReader;

let img = ImageReader::open("gavarni.jpg")?.decode()?;

img.save("empty.jpg")?;

