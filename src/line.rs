
// for coordinate selection
struct Point {
    x: f32,
    y: f32,
}

struct Line <'a> {
    x0   : f32,
    y0   : f32,
    x1   : f32,
    y1   : f32,
    color: [u8; 4],
    img  : &'a mut RgbaImage,
    draw_behavior: Box<dyn DrawBehavior>,
}

trait DrawBehavior {
    fn draw(&mut self);
}

struct Naive0{
    line: Line,
}
struct Naive1{}
struct Bresenham{}
struct Wu{}


impl DrawBehavior for Naive0 {
    fn draw(self) {
        for t in 0..100 {
            let t = t as f32 * 0.01;
            let x: u32 = (self.line.x0 + (self.line.x1 - self.line.x0) * t) as u32;
            let y: u32 = (self.line.y0 + (self.line.y1 - self.line.y0) * t) as u32;
            set(self.line.image, x, y, self.line.color);
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
                                           draw_behavior: Box::new(Naive0{line: self})
                                          },
        }
    }

    fn drop(&mut self){}

    fn draw(&mut self) {
        self.draw_behavior.draw();
    }
}

fn line(x0t   : f32,
        y0t   : f32,
        x1t   : f32,
        y1t   : f32,
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





/*------------------------------------------------------------------------------
                    Simple Working Temp
------------------------------------------------------------------------------*/

fn lineNaive0(x0: f32,
              y0: f32,
              x1: f32,
              y1: f32,
              color: [u8; 4],
              image: &mut RgbaImage) {
    for t in 0..100 {
        let t = t as f32 * 0.01;
        let x: u32 = (x0 + (x1 - x0) * t) as u32;
        let y: u32 = (y0 + (y1 - y0) * t) as u32;
        set(image, x, y, color);
     }
 }