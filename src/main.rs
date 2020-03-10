#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]



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


/*------------------------------------------------------------------------------
                    IMAGE OPERATIONS
------------------------------------------------------------------------------*/

pub fn set(image: &mut RgbaImage,
           x    : u32,
           y    : u32,
           color: [u8; 4])
    -> bool {
    image[(x, y)] = image::Rgba([color[0], color[1], color[2], color[3]]);
    return true
}

pub fn set_all (image: &mut RgbaImage,
                color: [u8; 4])
    -> bool {
    for (_x, _y, pixel) in image.enumerate_pixels_mut() {
        *pixel = image::Rgba([color[0], color[1], color[2], color[3]]);
    }
    return true
}


/*------------------------------------------------------------------------------
                    LINE METHODS
------------------------------------------------------------------------------*/

// for coordinate selection
struct Point {
    x: f32,
    y: f32,
}

struct Line {
    x0   : f32,
    y0   : f32,
    x1   : f32,
    y1   : f32,
    color: [u8; 4],
    image: RgbaImage,
    draw_behavior: Box<dyn DrawBehavior>,
}

trait DrawBehavior {
    fn draw(&mut self);
}

struct Naive0{
    x0   : f32,
    y0   : f32,
    x1   : f32,
    y1   : f32,
    color: [u8; 4],
    image: RgbaImage,
}
struct Naive1{}
struct Bresenham{}
struct Wu{}


impl DrawBehavior for Naive0 {
    fn draw(&mut self) {
        for t in 0..100 {
            let t = t as f32 * 0.01;
            let x: u32 = (self.x0 + (self.x1 - self.x0) * t) as u32;
            let y: u32 = (self.y0 + (self.y1 - self.y0) * t) as u32;
            set(&mut self.image, x, y, self.color);
        }
    }
}

enum LineMethodEnum {
    NAIVE0,
}

impl Line {
    fn new(x0   : f32,
           y0   : f32,
           x1   : f32,
           y1   : f32,
           color: [u8; 4],
           image: RgbaImage,
           line_method: LineMethodEnum)
        -> Line {
        match line_method {
            LineMethodEnum::NAIVE0 => Line{x0,
                                           y0,
                                           x1,
                                           y1,
                                           color,
                                           image,
                                           draw_behavior: Box::new(Naive0{x0, y0, x1, y1, color, image})
                                          },
        }
    }

    fn draw(&mut self) {
        self.draw_behavior.draw();
    }
}

fn line (x0t   : f32,
         y0t   : f32,
         x1t   : f32,
         y1t   : f32,
         colort: [u8; 4],
         imaget: RgbaImage,
         line_method: LineMethodEnum)
    -> bool {
    let mut temp = Line::new(x0t,
                         y0t,
                         x1t,
                         y1t,
                         colort,
                         imaget,
                         line_method);
    temp.draw();
    return true
}



/*------------------------------------------------------------------------------
                    MAIN METHOD
------------------------------------------------------------------------------*/

fn main() {
    // make a black image
    let mut img: RgbaImage = ImageBuffer::new(WIDTH, HEIGHT);
    set_all(&mut img, BLACK);
    set(&mut img, 52, 41, GREEN);
    //line::line(0, 0, 50, 50, BLUE, &mut img, NAIVE0);
    imageops::flip_vertical(&img);


    //save image
    let path = Path::new("out/pixel.png");
    let _file = match File::create(&path) {
        Err(e) =>{panic!("there was a problem creating the file: {:?}", e);}
        Ok(_file) => {let _ = img.save(&path).unwrap();}
    };
}
