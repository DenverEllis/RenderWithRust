#[allow(unused_variables)]

extern crate image;

use image::{RgbaImage, ImageBuffer, imageops};
use std::path::Path;
use std::fs::File;

const WIDTH : u32 = 100;
const HEIGHT: u32 = 100;

fn main() {
    // make a black image
    let mut img: RgbaImage = ImageBuffer::new(WIDTH, HEIGHT);
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([0, 0, 0, 255]);
    }

    img[(52, 41)] = image::Rgba([255, 0, 0, 255]);
    imageops::flip_vertical(&img);




    //save image
    let path = Path::new("out/image.png");


    let _file = match File::create(&path) {
        Err(e) =>{panic!("there was a problem creating the file: {:?}", e);}
        Ok(_file) => {let _ = img.save_with_format(&path, image::PNG);}
    };
}
