//! An example of generating julia fractals.
//!
use std::io::BufWriter;

use data_encoding::BASE64_MIME;
use image::{ColorType, ImageEncoder};

use rand::prelude::*;

use core::num::complex::Complex;

  fn mandelbrot_at_point(cx: f64, cy: f64, iters: usize) -> usize {
      let mut z = Complex { re: 0.0, im: 0.0 } ;
      let c = Complex::new(cx, cy);
  
      for i in 0..=iters { 
          // if z.re > 2.0 || z.im > 2.0 {
          if z.norm() > 2.0 {
  
              return i;
          }
          z = z*z + c;
      }
      return iters;
  }
  
  fn calculate_mandelbrot(iters: usize, x_min: f64, x_max: f64,  y_min: f64, y_max: f64, width: usize, height: usize) -> Vec<Vec<usize>> {
  
      let mut rows = Vec::<Vec<usize>>::with_capacity(width);
      for img_y in 0..height {
          let mut row = Vec::<usize>::with_capacity(height);
          for img_x in 0..width {
              let width_ = width as f64;
              let height_ = height as f64;
              let img_x_ = img_x as f64;
              let img_y_ = img_y as f64;
              // convert between pixel space and Mandelbrot space
              let cx = x_min + (x_max - x_min) * (img_x_ / width_);
              let cy = y_min + (y_max - y_min) * (img_y_ / height_);
              let escaped_at = mandelbrot_at_point(cx, cy, 2000);
              row.push(escaped_at);
          }
          rows.push(row);
  
      }
      rows
  }
  
  fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
      let mut line = String::with_capacity(row.len());
      for column in row {
        let val = match column {
          0 ..= 2 => ' ',
          2 ..= 5 => '.',
          5 ..= 10 => '•',
          11 ..= 30 => '*',
          30 ..= 100 => 'x',
          100 ..= 200 => '%',
          200 ..= 400 => '$',
          400 ..= 700 => '#',
          _ => '@',
      };
      
      line.push(val);
      }
      println!("{}", line);
    }
  }
  
  fn main() {
      let mandelbrot = calculate_mandelbrot(
        1000, 
        -2.5,
        1.0,
        -1.1,
        1.1,
  
        80,
        40, 
      );
  
      render_mandelbrot(mandelbrot);
  }

// fn main() {
  

//   return;;
//     const imgx: u32 = 32;
//     const imgy: u32 = 32;

//     // let scalex = 3.0 / imgx as f32;
//     // let scaley = 3.0 / imgy as f32;

//     // // Create a new ImgBuf with width: imgx and height: imgy
//     let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
//     let v = [0; (imgx * imgy * 3) as _].to_vec();
//     // // Iterate over the coordinates and pixels of the image
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let r = (0.3 * x as f32) as u8;
//         let b = (0.3 * y as f32) as u8;
//         *pixel = image::Rgb([r, 0, b]);
//         // v.push(r);
//         // v.push(0);
//         // v.push(b);
//     }
//     // let mut rng = rand::thread_rng();
//     // let v = [0; (imgx * imgy * 3) as _].to_vec();
//     // let light = [127; 3];
//     // let dark = [0; 3];
//     // let v = v
//     //     .into_iter()
//     //     .enumerate()
//     //     .map(|(i, x)| if rng.gen_bool() {
//     //     )
//     //     .collect::<Vec<u8>>();
//     // let v = imgbuf.fl();
//     let mut writer = BufWriter::with_capacity(imgbuf.len(), v.clone());
//     image::codecs::png::PngEncoder::new(&mut writer)
//         .write_image(&v, imgx, imgy, ColorType::Rgb8)
//         .unwrap();
//     // .unwrap();
//     // let res = imgbuf.write_to(&mut writer, image::ImageOutputFormat::Png);
//     let base64 = BASE64_MIME.encode(&writer.buffer());

//     // // Save the image as “fractal.png”, the format is deduced from the path
//     // imgbuf.save("fractal.png").unwrap();
//     // let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
//     let url = format!("data:image/png;base64,{}", base64.replace("\r\n", ""));

//     println!("{}", url);
// }
