use image::ImageReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("gavarni.jpg")?.decode()?;
    img.save("empty.jpg")?;
    Ok(())
}
