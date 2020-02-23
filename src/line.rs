#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]

use images;
use image::RgbaImage;

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
    fn draw(&self);
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
    fn draw(&self) {
        for t in 0..100 {
            let t = t as f32 * 0.01;
            let x: u32 = x0 + (x1 - x0) * t;
            let y: u32 = y0 + (y1 - y0) * t;
            images::set(&mut image, x, y, color)
        }
    }
}

impl Line {
    fn new(x0t   : f32,
           y0t   : f32,
           x1t   : f32,
           y1t   : f32,
           colort: [u8; 4],
           imaget: RgbaImage,
           line_method: LineMethodEnum)
        -> Line {
        match line_method {
            LineTypeEnum::NAIVE0 => Line{x0: x0t,
                                         y0: y1t,
                                         x1: x1t,
                                         y1: y1t,
                                         color: colort,
                                         image: imaget,
                                         draw_behavior: Box::new(Naive0{})},

            LineTypeEnum::NAIVE1 => Line{x0: x0t,
                                         y0: y1t,
                                         x1: x1t,
                                         y1: y1t,
                                         color: colort,
                                         image: imaget,
                                         draw_behavior: Box::new(Naive1{})},

            LineTypeEnum::BRESENHAM => Line{x0: x0t,
                                            y0: y1t,
                                            x1: x1t,
                                            y1: y1t,
                                            color: colort,
                                            image: imaget,
                                            draw_behavior: Box::new(Bresenham{})},

            LineTypeEnum::Wu => Line{x0: x0t,
                                     y0: y1t,
                                     x1: x1t,
                                     y1: y1t,
                                     color: colort,
                                     image: imaget,
                                     draw_behavior: Box::new(Wu{})},
        }
    }

    fn draw(&self) {
        self.draw_behavior.draw();
    }
}

pub fn line (x0t   : f32,
             y0t   : f32,
             x1t   : f32,
             y1t   : f32,
             colort: [u8; 4],
             imaget: RgbaImage,
             draw_type: DrawType)
    -> bool {
    temp = Line::new(x0t   : f32,
                     y0t   : f32,
                     x1t   : f32,
                     y1t   : f32,
                     colort: [u8; 4],
                     imaget: RgbaImage,
                     draw_type: DrawType);
    temp.draw();
    return true
}
