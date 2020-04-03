#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use std::path::Path;
use std::fs::File;
use image::{RgbaImage, ImageBuffer, imageops};
use render_with_rust::{
    line::{line, LineMethodEnum},
    geometry::{Scalar, Vec2},
    backend::images::{set, set_all}
};

const WIDTH : u32 = 100;
const HEIGHT: u32 = 100;

const WHITE : [u8; 4] = [  0,   0,   0,   0];
const BLACK : [u8; 4] = [  0,   0,   0, 255];
const RED   : [u8; 4] = [255,   0,   0, 255];
const GREEN : [u8; 4] = [  0, 255,   0, 255];
const BLUE  : [u8; 4] = [  0,   0, 255, 255];


/*------------------------------------------------------------------------------
                    MAIN METHOD
------------------------------------------------------------------------------*/

fn main() {
    let mut img: RgbaImage = ImageBuffer::new(WIDTH, HEIGHT);
    set_all(&mut img, BLACK);
    set(&mut img, 52, 41, GREEN);

    line(13, 20, 80, 40, WHITE, &mut img, LineMethodEnum::BRESENHAM);
    line(20, 13, 40, 80, RED,   &mut img, LineMethodEnum::BRESENHAM);
    line(80, 40, 13, 20, RED,   &mut img, LineMethodEnum::BRESENHAM);
    imageops::flip_vertical_in_place(&mut img);

    let testVec2: Vec2<i16> = Vec2::new(1, 2);
    let tempVec2: Vec2<i16> = Vec2::new(3, 4);
    let testScal: Scalar<i16> = Scalar::new(10);

    println!("Vec2: {:?}", tempVec2 + testVec2);
    println!("Vec2: {:?}", tempVec2 - testVec2);
    println!("Vec2: {:?}", tempVec2 * testVec2);
    println!("Vec2: {:?}", tempVec2 / testVec2);
    // Does not work
    // println!("Vec2: {:?}", testScal * tempVec2);

    //save image
    let path = Path::new("out/lineBresenham.png");
    let _file = match File::create(&path) {
        Err(e) =>{panic!("there was a problem creating the file: {:?}", e);}
        Ok(_file) => {let _ = img.save(&path).unwrap();}
    };
}
