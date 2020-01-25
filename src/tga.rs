use std::io;
use std::io::{Read, Seek};
use byteorder::{ReadBytesExt, LittleEndian};
use std::error::Error;

/*------------------------------------
            Image Basics
------------------------------------*/
enum Image_Type {
    no_image_data  = 0,

    // Uncompressed Image
    raw_color_map  = 1,
    raw_true_color = 2,
    raw_gray_scale = 3,

    // Run Length Encoded images
    run_color_map  = 9,
    run_true_color = 10,
    run_gray_scale = 11,

    // Fully Compressed Images
    com1_color_map = 32,
    quad_color_map = 33,
    Unknown,
}

impl Image_Type {
    fn new(img_type: u8) -> Image_Type {
        match img_type {
            0  => Image_Type::No_Image_Data,

            // Raw
            1  => Image_Type::Raw_Color_Map,
            2  => Image_Type::Raw_True_Color,
            3  => Image_Type::Raw_Gray_Scale,

            // Run Length Encoded
            9  => Image_Type::Run_Color_Map,
            10 => Image_Type::Run_True_Color,
            11 => Image_Type::Run_Gray_Scale,

            // Should not be used
            32 => Image_Type::Compressed_Color_Map_Single,
            33 => Image_Type::Compressed_Color_Map_Quad,
x            _  => Image_Type::Unknown,
        }
    }

    // Does the image use colors or grayscale?
    fn is_color(&self) -> bool {
        match *self {
            Image_Type::Raw_Color_Map  |
            Image_Type::Raw_True_Color |
            Image_Type::Run_Color_Map  |
            Image_Type::Run_True_Color => true,
            _ => false,
        }
    }

    // Does the image use a color map or true colors
    fn is_color_mapped(&self) -> bool {
        match *self {
            Image_Type::Raw_Color_Map |
            Image_Type::Run_Color_Map => true,
            _ => false,
        }
    }

    // Is the image encoded
    fn is_encoded(&self) -> bool {
        match *self {
            Image_Type::Run_Color_Map               |
            Image_Type::Run_True_Color              |
            Image_Type::Run_Gray_Scale              |
            Image_Type::Compressed_Color_Map_Single |
            Image_Type::Compressed_Color_Map_Quad   => true,
            _ => false,
        }
    }

    fn is_compressed(&self) -> bool {
        match *self {
            Image_Type::Compressed_Color_Map_Single |
            Image_Type::Compressed_Color_Map_Quad   => true,
            _ => false,
        }
    }
}


// total 18 bytes or 144 bits
// Header for TGA File
#[derive(Debug)]
struct Header {
    id_length        : u8,    // between 0 and 255, the length of imageDescriptor
    color_map_type   : u8,    // either 0 or 1, if 0 ignore bytes 3-7
    image_type       : u8,    // type of image, can be [1, 2, 3, 9, 10, 11, 32, 33] wil probably be 10
    color_map_origin : u16,   // int (lo-hi), index of the first color map entry
    color_map_length : u16,   // int (lo-hi), count of color map entries
    color_map_depth  : u8,    // number of bits in a color map entry. Likely to be either [16, 24, or 32]
    x_origin         : u16,   // int (lo-hi), X-coordinate of the lower left corner
    y_origin         : u16,   // int (lo-hi), Y-coordinate of the lower left corner
    width            : u16,   // int (lo-hi), width of the image in pixels
    height           : u16,   // int (lo-hi), height of the image in pixels
    bits_per_pixel   : u8,    // number of bits per pixel. Likely to be [16, 24, 32]
    image_descriptor : u8,    // 00001000, bits 0-3 are set to 0 for 32 bit image
}

impl Header {
    // construct new Header
    fn new() -> Header {
        Header {
            id_length        : 0,
            color_map_type   : 0,
            image_type       : 0,
            color_map_origin : 0,
            color_map_length : 0,
            color_map_depth  : 0,
            x_origin         : 0,
            y_origin         : 0,
            width            : 0,
            height           : 0,
            bits_per_pixel   : 0,
            image_descriptor : 0,
        }
    }

    // Load Header with values from the reader(?)
    fn from_reader(r: &mut Read) -> Image_Result<Header> {
        Ok(Header {
            id_length        : try!(r.read_u8()),
            color_map_type   : try!(r.read_u8()),
            image_type       : try!(r.read_u8()),
            color_map_origin : try!(r.read_u16::<LittleEndian>()),
            color_map_length : try!(r.read_u16::<LittleEndian>()),
            color_map_depth  : try!(r.read_u8()),
            x_origin         : try!(r.read_u16::<LittleEndian>()),
            y_origin         : try!(r.read_u16::<LittleEndian>()),
            width            : try!(r.read_u16::<LittleEndian>()),
            height           : try!(r.read_u16::<LittleEndian>()),
            bits_per_pixel   : try!(r.read_u8()),
            image_descriptor : try!(r.read_u8()),
        })
    }
}

struct Image {
    width: i32,
    height: i32,
    data: Vec<Color>,
}


// run length encoded rgba image
let rle_rgb = TGA_Header {
    idLength: 0u8,
    colorMapType: 0u8,
    dataTypeCode: 10u8,
    colorMapOrigin: 0u16,
    colorMapLength: 0u16,
    colorMapDepth: 0u8,
    xOrigin: 0u16,
    yOrigin: 0u16,
    width: 640u16,
    height: 480u16,
    bitsPerPixel: 32u8,
    imageDescriptor: 8u8,
};

