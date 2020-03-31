use std::mem;
use image::RgbaImage;
use crate::backend::images::set;

/*------------------------------------------------------------------------------
                    LINE METHODS (STRATEGY PATTERN)
------------------------------------------------------------------------------*/
struct Line <'a> {
    x0   : i32,
    y0   : i32,
    x1   : i32,
    y1   : i32,
    color: [u8; 4],
    img  : &'a mut RgbaImage,
    draw_behavior: Box<dyn DrawBehavior>,
}

trait DrawBehavior {
    fn draw(&self,
            x0   : i32,
            y0   : i32,
            x1   : i32,
            y1   : i32,
            color: [u8; 4],
            img  : &mut RgbaImage);
}

enum LineMethodEnum {
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
            x0   : i32,
            y0   : i32,
            x1   : i32,
            y1   : i32,
            color: [u8; 4],
            img  : &mut RgbaImage) {
        for t in 0..100 {
            let t = t as f32 * 0.01;
            let x: i32 = (x0 as f32 + (x1 - x0) as f32 * t) as i32;
            let y: i32 = (y0 as f32 + (y1 - y0) as f32 * t) as i32;
            set(img, x, y, color);
        }
    }
}

impl DrawBehavior for Naive1 {
    fn draw(&self,
            x0   : i32,
            y0   : i32,
            x1   : i32,
            y1   : i32,
            color: [u8; 4],
            img  : &mut RgbaImage) {

        let mut steep: bool = false;
        let mut x0t = x0;
        let mut x1t = x1;
        let mut y0t = y0;
        let mut y1t = y1;

        if (x0-x1).abs() < (y0-y1).abs() { //if the line is steep, transpose
            mem::swap(&mut x0t, &mut y0t);
            mem::swap(&mut x1t, &mut y1t);
            steep = true;
        }

        if x0 > x1 { // make it left to right
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
            x0   : i32,
            y0   : i32,
            x1   : i32,
            y1   : i32,
            color: [u8; 4],
            img  : &mut RgbaImage) {

        let mut steep: bool = false;
        let mut x0t  : i32 = x0;
        let mut x1t  : i32 = x1;
        let mut y0t  : i32 = y0;
        let mut y1t  : i32 = y1;

        if (x0-x1).abs() < (y0-y1).abs() { //if the line is steep, transpose
            mem::swap(&mut x0t, &mut y0t);
            mem::swap(&mut x1t, &mut y1t);
            steep = true;
        }

        if x0 > x1 { // make it left to right
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
            x0   : i32,
            y0   : i32,
            x1   : i32,
            y1   : i32,
            color: [u8; 4],
            img  : &mut RgbaImage) {
        // Needed for mutability and protection of user input
        let mut steep: bool = false;
        let mut x0t  : i32 = x0;
        let mut x1t  : i32 = x1;
        let mut y0t  : i32 = y0;
        let mut y1t  : i32 = y1;

        if (x0-x1).abs() < (y0-y1).abs() { //if the line is steep, transpose
            mem::swap(&mut x0t, &mut y0t);
            mem::swap(&mut x1t, &mut y1t);
            steep = true;
        }

        if x0 > x1 { // make it left to right
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
    fn new(x0   : i32,
           y0   : i32,
           x1   : i32,
           y1   : i32,
           color: [u8; 4],
           img  : &mut RgbaImage,
           line_method: LineMethodEnum)
        -> Line<'_> {
        match line_method {
            LineMethodEnum::NAIVE0 => Line{x0,
                                           y0,
                                           x1,
                                           y1,
                                           color,
                                           img,
                                           draw_behavior: Box::new(Naive0{})
                                          },
                                          
            LineMethodEnum::NAIVE1 => Line{x0,
                                           y0,
                                           x1,
                                           y1,
                                           color,
                                           img,
                                           draw_behavior: Box::new(Naive1{})
                                          },

            LineMethodEnum::NAIVE2 => Line{x0,
                                           y0,
                                           x1,
                                           y1,
                                           color,
                                           img,
                                           draw_behavior: Box::new(Naive2{})
                                          },

            LineMethodEnum::BRESENHAM => Line{x0,
                                           y0,
                                           x1,
                                           y1,
                                           color,
                                           img,
                                           draw_behavior: Box::new(Bresenham{})
                                          },
        }
    }

    fn draw(self) {
        self.draw_behavior.draw(self.x0,
                                self.y0,
                                self.x1,
                                self.y1,
                                self.color,
                                self.img);
    }
}

fn line(x0t   : i32,
        y0t   : i32,
        x1t   : i32,
        y1t   : i32,
        colort: [u8; 4],
        imaget: &mut RgbaImage,
        line_method: LineMethodEnum)
{
    let mut temp = Line::new(x0t,
                             y0t,
                             x1t,
                             y1t,
                             colort,
                             imaget,
                             line_method);
    temp.draw();
}