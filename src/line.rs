use std::mem;
use image::RgbaImage;
use crate::backend::{images::set, geometry::*};

/*------------------------------------------------------------------------------
                    LINE METHODS (STRATEGY PATTERN)
------------------------------------------------------------------------------*/
struct Line <'a> {
    p0: Vec2i,
    p1: Vec2i,
    color: [u8; 4],
    img  : &'a mut RgbaImage,
    draw_behavior: Box<dyn DrawBehavior>,
}

trait DrawBehavior {
    fn draw(&self,
            p0: Vec2i,
            p1: Vec2i,
            color: [u8; 4],
            img  : &mut RgbaImage);
}

pub enum LineMethodEnum {
    NAIVE0,
    NAIVE1,
    NAIVE2,
    BRESENHAM,
}

struct Naive0{}
struct Naive1{}
struct Naive2{}
struct Bresenham{}
struct Wu{}

impl DrawBehavior for Naive0 {
    fn draw(&self,
            p0: Vec2i,
            p1: Vec2i,
            color: [u8; 4],
            img  : &mut RgbaImage) {
        for t in 0..100 {
            let t = t as f32 * 0.01;
            let x: i32 = (p0.x.value as f32 + (p1.x.value - p0.x.value) as f32 * t) as i32;
            let y: i32 = (p0.y.value as f32 + (p1.y.value - p0.y.value) as f32 * t) as i32;
            set(img, x, y, color);
        }
    }
}

impl DrawBehavior for Naive1 {
    fn draw(&self,
            p0   : Vec2i,
            p1   : Vec2i,
            color: [u8; 4],
            img  : &mut RgbaImage) {
        let mut steep: bool = false;
        let mut x0t = p0.x.value;
        let mut x1t = p1.x.value;
        let mut y0t = p0.y.value;
        let mut y1t = p1.y.value;

        if (p0.x.value - p1.x.value).abs() < (p0.y.value - p1.y.value).abs() { //if the line is steep, transpose
            mem::swap(&mut x0t, &mut y0t);
            mem::swap(&mut x1t, &mut y1t);
            steep = true;
        }

        if p0.x.value > p1.x.value { // make it left to right
            mem::swap(&mut x0t, &mut x1t);
            mem::swap(&mut y0t, &mut y1t);
        }

        for x in x0t..x1t  {
            let t: f32 = (x-x0t) as f32 / (x1t-x0t) as f32;
            let y: i32 = (y0t as f32 * (1.0 - t) + y1t as f32 * t) as i32;

            if steep {
                set(img, y, x, color);  //if transposed, de-transpose
            } else {
                set(img, x, y, color);
            }
        }
    }
}

impl DrawBehavior for Naive2 {
    fn draw(&self,
            p0   : Vec2i,
            p1   : Vec2i,
            color: [u8; 4],
            img  : &mut RgbaImage) {

        let mut steep: bool = false;
        let mut x0t = p0.x.value;
        let mut x1t = p1.x.value;
        let mut y0t = p0.y.value;
        let mut y1t = p1.y.value;

        if (p0.x.value - p1.x.value).abs() < (p0.y.value - p1.y.value).abs() { //if the line is steep, transpose
            mem::swap(&mut x0t, &mut y0t);
            mem::swap(&mut x1t, &mut y1t);
            steep = true;
        }

        if p0.x.value > p1.x.value { // make it left to right { // make it left to right
            mem::swap(&mut x0t, &mut x1t);
            mem::swap(&mut y0t, &mut y1t);
        }

        let dx:        i32 = x1t - x0t;
        let dy:        i32 = y1t - y0t;
        let derror:    f32 = (dy as f32/ dx as f32).abs();
        let mut error: f32 = 0.;
        let mut y:     i32 = y0t;

        for x in x0t..x1t  {
            if steep {
                set(img, y, x, color);  //if transposed, de-transpose
            } else {
                set(img, x, y, color);
            }

            error += derror;
            if error > 0.5 {
                y += if y1t > y0t {1} else {-1};
                error -= 1.;
            }
        }
    }
}

impl DrawBehavior for Bresenham {
    fn draw(&self,
            p0   : Vec2i,
            p1   : Vec2i,
            color: [u8; 4],
            img  : &mut RgbaImage) {
        // Needed for mutability and protection of user input
        let mut steep: bool = false;
        let mut x0t = p0.x.value;
        let mut x1t = p1.x.value;
        let mut y0t = p0.y.value;
        let mut y1t = p1.y.value;

        if (p0.x.value - p1.x.value).abs() < (p0.y.value - p1.y.value).abs() { //if the line is steep, transpose
            mem::swap(&mut x0t, &mut y0t);
            mem::swap(&mut x1t, &mut y1t);
            steep = true;
        }

        if p0.x.value > p1.x.value { // make it left to right
            mem::swap(&mut x0t, &mut x1t);
            mem::swap(&mut y0t, &mut y1t);
        }

        let dx:        i32 = x1t - x0t;
        let dy:        i32 = y1t - y0t;
        let derror:    i32 = dy.abs() * 2;
        let mut error: i32 = 0;
        let mut y:     i32 = y0t;

        if steep {
            for x in x0t..x1t  {
                set(img, y, x, color);  //if transposed, de-transpose
                error += derror;
                if error > dx {
                    y += if y1t > y0t {1} else {-1};
                    error -= dx * 2;
                }
            }
        } else {
            for x in x0t..x1t  {
                set(img, x, y, color);
                error += derror;
                if error > dx {
                    y += if y1t > y0t {1} else {-1};
                    error -= dx * 2;
                }
            }
        }
    }
}


impl Line<'_> {
    fn new(p0   : Vec2i,
           p1   : Vec2i,
           color: [u8; 4],
           img  : &mut RgbaImage,
           line_method: LineMethodEnum)
        -> Line<'_> {
        match line_method {
            LineMethodEnum::NAIVE0 => Line{p0,
                                           p1,
                                           color,
                                           img,
                                           draw_behavior: Box::new(Naive0{})
                                          },
                                          
            LineMethodEnum::NAIVE1 => Line{p0,
                                           p1,
                                           color,
                                           img,
                                           draw_behavior: Box::new(Naive1{})
                                          },

            LineMethodEnum::NAIVE2 => Line{p0,
                                           p1,
                                           color,
                                           img,
                                           draw_behavior: Box::new(Naive2{})
                                          },

            LineMethodEnum::BRESENHAM => Line{p0,
                                              p1,
                                              color,
                                              img,
                                              draw_behavior: Box::new(Bresenham{})
                                          },
        }
    }

    fn draw(self) {
        self.draw_behavior.draw(self.p0,
                                self.p1,
                                self.color,
                                self.img);
    }
}

pub fn line(p0t   : Vec2i,
            p1t   : Vec2i,
            colort: [u8; 4],
            imaget: &mut RgbaImage,
            line_method: LineMethodEnum) {
    let mut temp = Line::new(p0t,
                             p1t,
                             colort,
                             imaget,
                             line_method);
    temp.draw();
}