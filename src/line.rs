#[allow(unused_variables)]
extern crate image;
use image::RgbaImage;

struct Line {
    name: String,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: image::Rgba,
    image: RgbaImage;
    draw_behavior: Box<dyn DrawBehavior>
}

trait DrawBehavior {
    fn draw(&self);
}

struct Naive0{}
struct Naive1{}
struct Bresenham{}
struct Wu{}

impl DrawBehavior for Naive0 {
    fn draw(&self) {
        for t in 0..100 {
            let t = t as f32 * 0.01;
            let x: u32 = x0 + (x1 - x0) * t;
            let y: u32 = y0 + (y1 - y0) * t;
            image
        }
    }
}
