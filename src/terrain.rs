use lodepng::{Bitmap, ColorType, Grey};
use rgb::{RGBA, RGBA8};

pub struct HeightMap(Bitmap<Grey<u8>>);

pub struct Texture(Bitmap<RGBA8>);

// We hardcode a size of 1024 for the heightmap / texture

impl HeightMap {
    pub fn get(self: &Self, i: i32, j: i32) -> u8 {
        let i = (i as usize) & 1023;
        let j = (j as usize) & 1023;
        self.0.buffer[(i + self.width() * j)].0
    }

    pub fn from_file(path: &str) -> Result<HeightMap, String> {
        match lodepng::decode_file(path, ColorType::GREY, 8) {
            Err(e) => Err(format!("Error opening the file {} ({})", path, e.0)),
            Ok(image) => match image {
                lodepng::Image::Grey(im) => {
                    if (im.width, im.height) != (1024, 1024) {
                        Err(format!(
                            "Texture size must be exactly 1024x1024, is {}x{})",
                            im.width, im.height
                        ))
                    } else {
                        Ok(HeightMap::from(im))
                    }
                }
                _ => Err(format!(
                    "Not the right format, expect grayscale 8 bits. Was {:?}",
                    image
                )),
            },
        }
    }

    pub fn from(data: Bitmap<Grey<u8>>) -> HeightMap {
        HeightMap(data)
    }

    pub fn width(self: &Self) -> usize {
        1024
    }
}

impl Texture {
    pub fn get(self: &Self, i: i32, j: i32) -> RGBA8 {
        let i = (i as usize) & 1023;
        let j = (j as usize) & 1023;
        self.0.buffer[i + self.width() * j]
    }

    pub fn from_file(path: &str) -> Result<Texture, String> {
        match lodepng::decode_file(path, ColorType::RGBA, 8) {
            Err(e) => Err(format!("Error opening the file {} ({})", path, e.0)),
            Ok(image) => match image {
                lodepng::Image::RGBA(im) => {
                    if (im.width, im.height) != (1024, 1024) {
                        Err(format!(
                            "Texture size must be exactly 1024x1024, is {}x{})",
                            im.width, im.height
                        ))
                    } else {
                        Ok(Texture::from(im))
                    }
                }
                _ => Err(format!(
                    "Not the right format, expect grayscale 8 bits. Was {:?}",
                    image
                )),
            },
        }
    }

    pub fn from(data: Bitmap<RGBA<u8>>) -> Texture {
        Texture(data)
    }

    pub fn width(self: &Self) -> usize {
        1024
    }
}
