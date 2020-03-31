#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]

use image::RgbaImage;

/*------------------------------------------------------------------------------
                    IMAGE OPERATIONS
------------------------------------------------------------------------------*/

pub fn set(image: &mut RgbaImage,
           x    : i32,
           y    : i32,
           color: [u8; 4])
    -> bool {
    image[(x as u32, y as u32)] = image::Rgba([color[0], color[1], color[2], color[3]]);
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
