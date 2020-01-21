use std::mem;
use std::slice;

struct Color(u8, u8, u8, u8) // red, green, blue, alpha. Each collor is 32 bits.

// total 18 bytes or 144 bits
struct TGA_Header {
    idLength: u8,          // between 0 and 255, the length of imageDescriptor
    colorMapType: u8,      // either 0 or 1, if 0 ignore bytes 3-7
    dataTypeCode: u8,      // type of image, can be [1, 2, 3, 9, 10, 11, 32, 33] wil probably be 10
    colorMapOrigin: u16,   // int (lo-hi), index of the first color map entry
    colorMapLength: u16,   // int (lo-hi), count of color map entries
    colorMapDepth: u8,     // number of bits in a color map entry. Likely to be either [16, 24, or 32]
    xOrigin: u16,          // int (lo-hi), X-coordinate of the lower left corner
    yOrigin: u16,          // int (lo-hi), Y-coordinate of the lower left corner
    width: u16,            // int (lo-hi), width of the image in pixels
    height: u16,           // int (lo-hi), height of the image in pixels
    bitsPerPixel: u8,      // number of bits per pixel. Likely to be [16, 24, 32]
    imageDescriptor: u8,   // 00001000, bits 0-3 are set to 0 for 32 bit image
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

struct Image {
    width: i32,
    height: i32,
    data: Vec<Color>,
}
