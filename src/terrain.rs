use lodepng::{Bitmap, ColorType, Grey};
use rgb::{RGBA, RGBA8};

pub struct HeightMap(Bitmap<Grey<u8>>);

pub struct Texture(Bitmap<RGBA<u8>>);

impl HeightMap {
    pub fn get(self: &Self, i: i32, j: i32) -> u8 {
        let i = i.rem_euclid(self.width());
        let j = j.rem_euclid(self.height());
        self.0.buffer[(i + self.width() * j) as usize].0
    }

    pub fn from_file(path: &str) -> Result<HeightMap, String> {
        match lodepng::decode_file(path, ColorType::GREY, 8) {
            Err(e) => Err(format!("Error opening the file {} ({})", path, e.0)),
            Ok(image) => match image {
                lodepng::Image::Grey(im) => Ok(HeightMap::from(im)),
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

    pub fn width(self: &Self) -> i32 {
        self.0.width as i32
    }

    pub fn height(self: &Self) -> i32 {
        self.0.height as i32
    }
}

impl Texture {
    pub(crate) fn get(self: &Self, i: i32, j: i32) -> RGBA8 {
        let i = i.rem_euclid(self.width());
        let j = j.rem_euclid(self.height());
        self.0.buffer[(i + self.width() * j) as usize]
    }

    pub fn from_file(path: &str) -> Result<Texture, String> {
        match lodepng::decode_file(path, ColorType::RGBA, 8) {
            Err(e) => Err(format!("Error opening the file {} ({})", path, e.0)),
            Ok(image) => match image {
                lodepng::Image::RGBA(im) => Ok(Texture::from(im)),
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

    pub fn width(self: &Self) -> i32 {
        self.0.width as i32
    }

    pub fn height(self: &Self) -> i32 {
        self.0.height as i32
    }
}
