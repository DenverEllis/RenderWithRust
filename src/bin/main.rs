#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use std::path::Path;
use std::fs::File;
use image::{RgbaImage, ImageBuffer, imageops};
use render_with_rust::{
    line::{line, LineMethodEnum::*},
    backend::{images::*, geometry::*},
    model::*
};

const WIDTH : u32 = 480;
const HEIGHT: u32 = 620;

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

    let v0: Vec2i = Vec2i::new(Scalar::<i32>{value: 13}, Scalar::<i32>{value: 20});
    let v1: Vec2i = Vec2i::new(Scalar::<i32>{value: 80}, Scalar::<i32>{value: 40});

    line(v0, v1, WHITE, &mut img, BRESENHAM);


    // Vector Tests
    let testVec2: Vec2<i16> = Vec2::new(Scalar::<i16>{value: 1}, Scalar::<i16>{value: 2});
    let tempVec2: Vec2<i16> = Vec2::new(Scalar::<i16>{value: 3}, Scalar::<i16>{value: 4});
    let tempVec3: Vec3<i16> = Vec3::new(Scalar::<i16>{value: 5},
                                        Scalar::<i16>{value: 6},
                                        Scalar::<i16>{value: 7});
    let tempVec4: Vec4<i16> = Vec4::new(Scalar::<i16>{value: 8},
                                        Scalar::<i16>{value: 9},
                                        Scalar::<i16>{value: 10},
                                        Scalar::<i16>{value: 11});
    let testScal: Scalar<i16> = Scalar::new(10);

    println!("Vec2: {:?}", tempVec2 + testVec2);
    println!("Vec2: {:?}", tempVec2 - testVec2);
    println!("Vec2: {:?}", tempVec2 * testVec2);
    println!("Vec2: {:?}", tempVec2 / testVec2);
    println!("Vec2: {:?}", tempVec2 * testScal);


    // Triangle Tests
    triangle(Vec2i{x: Scalar::<i32>{value: 10}, y: Scalar::<i32>{value: 70}},
             Vec2i{x: Scalar::<i32>{value: 50}, y: Scalar::<i32>{value: 160}},
             Vec2i{x: Scalar::<i32>{value: 70}, y: Scalar::<i32>{value: 80}}, RED, &mut img);


    imageops::flip_vertical_in_place(&mut img);
    //save image
    let path = Path::new("out/triangleTest.png");
    let _file = match File::create(&path) {
        Err(e) =>{panic!("there was a problem creating the file: {:?}", e);}
        Ok(_file) => {let _ = img.save(&path).unwrap();}
    };
}
