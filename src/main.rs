#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]

pub mod line;
pub mod images;

use std::path::Path;
use std::fs::File;
use image::{RgbaImage, ImageBuffer, imageops};

const WIDTH : u32 = 100;
const HEIGHT: u32 = 100;

const WHITE : [u8; 4] = [  0,   0,   0,   0];
const BLACK : [u8; 4] = [  0,   0,   0, 255];
const RED   : [u8; 4] = [255,   0,   0, 255];
const GREEN : [u8; 4] = [  0, 255,   0, 255];
const BLUE  : [u8; 4] = [  0,   0, 255, 255];

fn main() {
    // make a black image
    let mut img: RgbaImage = ImageBuffer::new(WIDTH, HEIGHT);
    images::set_all(&mut img, BLACK);
    images::set(&mut img, 52, 41, GREEN);
    //line::line(0, 0, 50, 50, BLUE, &mut img, NAIVE0);
    imageops::flip_vertical(&img);


    //save image
    let path = Path::new("out/pixel.png");
    let _file = match File::create(&path) {
        Err(e) =>{panic!("there was a problem creating the file: {:?}", e);}
        Ok(_file) => {let _ = img.save(&path).unwrap();}
    };
}
