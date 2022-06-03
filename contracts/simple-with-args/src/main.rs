//! An example of generating julia fractals.
//!
use std::io::BufWriter;

use data_encoding::BASE64_MIME;
use image::{ColorType, ImageEncoder};

use rand::prelude::*;

fn main() {
    const imgx: u32 = 200;
    const imgy: u32 = 200;

    // let scalex = 3.0 / imgx as f32;
    // let scaley = 3.0 / imgy as f32;

    // // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
    let mut rng = rand::thread_rng();
    let v = [0; (imgx * imgy * 3) as _].to_vec();
    let v = v
        .into_iter()
        .enumerate()
        .map(|(i, x)| match i % 3 {
            0 => ((rng.gen_range(0..127) + x)) as u8,
            1 => rng.gen_range(64..127) + x,
            _ => (rng.gen_range(0..127) as f32 * 0.3) as u8,
        })
        .collect::<Vec<u8>>();
    let mut writer = BufWriter::with_capacity(imgbuf.len(), v.clone());
    image::codecs::png::PngEncoder::new(&mut writer)
        .write_image(&v, imgx, imgy, ColorType::Rgb8)
        .unwrap();
    // .unwrap();
    // let res = imgbuf.write_to(&mut writer, image::ImageOutputFormat::Png);
    let base64 = BASE64_MIME.encode(&writer.buffer());

    // // Save the image as “fractal.png”, the format is deduced from the path
    // imgbuf.save("fractal.png").unwrap();
    // let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let url = format!("data:image/png;base64,{}", base64.replace("\r\n", ""));

    println!("{}", url);
}
