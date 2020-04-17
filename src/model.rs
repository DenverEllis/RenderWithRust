use std::mem::swap;
use image::RgbaImage;
use crate::{line::*,
            backend::{images::set, geometry::*}};

/*------------------------------------------------------------------------------
                    TRIANGLE METHODS (STRATEGY PATTERN)
------------------------------------------------------------------------------*/
pub fn triangle (t0   : Vec2i,
                 t1   : Vec2i,
                 t2   : Vec2i,
                 color: [u8; 4],
                 image: &mut RgbaImage) {
    let mut t0t: Vec2f = Vec2::new(Scalar::<f32>{value: t0.x.value as f32},
                                   Scalar::<f32>{value: t0.y.value as f32});
    let mut t1t: Vec2f = Vec2::new(Scalar::<f32>{value: t1.x.value as f32},
                                   Scalar::<f32>{value: t1.y.value as f32});
    let mut t2t: Vec2f = Vec2::new(Scalar::<f32>{value: t2.x.value as f32},
                                   Scalar::<f32>{value: t2.y.value as f32});

    if (t0t.y.value == t1t.y.value) && (t0t.y.value == t2t.y.value) {return;}
    if t0t.y.value > t1t.y.value {swap(&mut t0t, &mut t1t);}
    if t0t.y.value > t2t.y.value {swap(&mut t0t, &mut t2t);}
    if t1t.y.value > t2t.y.value {swap(&mut t1t, &mut t2t);}

    let total_height: i32 = (t2t.y.value - t0t.y.value) as i32;
    for k in 0..total_height {
        let i = k as f32;
        let second_half: bool  = i > (t1t.y.value - t0t.y.value) || t1t.y.value == t0t.y.value;
        let segment_height: f32 = if second_half {(t2t.y.value - t1t.y.value)}
                                  else {(t1t.y.value - t0t.y.value)};

        let alpha: f32 = i / total_height as f32;
        let beta : f32 = i - (if second_half {t1t.y.value - t0t.y.value} else {0.0}) / segment_height;

        let mut A: Vec2f = t0t + (t2t - t0t) * Scalar::<f32>{value: alpha};
        let mut B: Vec2f = if second_half {t1t + (t2t - t1t) * Scalar::<f32>{value: beta}}
                       else {t0t + (t1t - t0t) * Scalar::<f32>{value: beta}};

        if A.x.value > B.x.value {swap(&mut A, &mut B)};
        for j in A.x.value as i32..B.y.value as i32 {
            //println!("{:?}", j)
            set(image, j, (t0t.y.value + i) as i32, color);
        }
    }
}
/*
pub fn triangle (t0   : Vec2i,
                 t1   : Vec2i,
                 t2   : Vec2i,
                 color: [u8; 4],
                 image: &mut RgbaImage) {
    // Sort the vectors
    if t1.y - t0.y > 0
}*/