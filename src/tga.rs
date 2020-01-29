luse std::io;
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


let 

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

struct Color_Map {
    // sizes in bytes
    start_offset: usize,
    entry_size:   usize,
    bytes:        Vec<u8>,
}

impl Color_Map {
    pub fn from_reader(r: &mut Read, start_offset: u16, num_entries: u16, bits_per_entry: u8)
    -> ImageResult<ColorMap> {
        let bytes_per_entry = (bits_per_entry as usize + 7) / 8; //Why?
        let mut bytes = vec![0; bytes_per_entry * num_entries as usize];
        try!(r.read_exact(&mut bytes));
        Ok(Color_Map {
            entry_size: bytes_per_entry,
            start_offset: start_offset as usize,
            bytes: bytes,
        })
    }

    // Get an entry from the color map
    pub fn get(&self, index: usize) -> &[u8] {
        let entry = self.start_offset + self.entry_size * index;
        &self.bytes[entry..entry + self.entry_size]
    }
}

pub struct TGA_Decoder<R> {
    r: R,

    width:               usize,
    height:              usize,
    bytes_per_pixel:     usize,
    has_loaded_metadata: bool,

    image_type: Image_Type,
    color_type: Color_Type,

    header: Header,
    color_map: Option<ColorMap>,
}

impl<R: Read + Seek> TGA_Decoder<R> {
    //create a new decoder that decodes from the stream r
    pub fn new(r: R) -> TGA_Decoder<R> {
        TGA_Decoder {
            r: r,

            width:  0,
            height: 0,
            bytes_per_pixel: 0,
            has_loaded_metadata: false,

            image_type: Image_Type::Unknown,
            color_type: Color_Type::Gray(1),

            header: Header::new(),
            color_map: None,
        }
    }

    fn read_header(&mut self) -> Image_Result<()> {
        self.header = try!(Header::from_reader(&mut self.r));
        self.image_type = Image_Type::new(self.header.image_type);
        self.width = self.header.image_width as usize;
        self.height = self.header.image_height as usize;
        self.byttes_per_pixel = (self.header.pixel_depth as usize + 7) /8;
        Ok(())
    }

    fn read_metadata(&mut self) -> ImageResult<()> {
        if !self.has_loaded_metadata {
            try!(self.read_header());
            try!(self.read_image_id());
            try!(self.read_color_map());
            try!(self.read_color_information());
            self.has_loaded_metadata = true;
        }
        Ok(())
    }

    // loads color information
    //currently won't handle depths not divisible by 8 or greater than 32
    fn read_color_information(&mut self) -> Image_Result<()> {
        if self.header.pixel_depth % 8 != 0 {
            return Err(Image_Error::Unsupported_Error("\ Bit depth must be divisible by 8".to_string()));
        }

        if self.header.pixel_depth >32 {
            return Err(Image_Error::Unsupported_Error("\ Bit depth must be less than 32".to_string()));
        }

        let num_alpha_bits = self.header.image_desc & 0b1111;

        let other_channel_bits = if self.header.map_type != 0 {
            self.header.map_entry_size
        } else {
            if num_alpha_bits > self.header.pixel_depth {
                return Err(Image_Error::Unsupported_Error(format!("\
                    Color format not supported. Alpha bits: {}", num_alpha_bits).to_string()))
            }
            self.header.pixel_depth - num_alpha_bits
        };

        let color = self.image_type.is_color();

        match (num_alpha_bits, other_channel_bits, color) {

        }
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

