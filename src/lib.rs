#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use std::{path::Path, fs::File, mem};
use image::{RgbaImage, ImageBuffer, imageops};

mod line;
mod geometry;
mod model;
mod image;