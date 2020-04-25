use lodepng::{Bitmap, ColorMode, ColorType, Grey};
use rgb::{RGBA, RGBA8};
use serde::export::Vec;

pub struct HeightMap(Bitmap<Grey<u8>>);

pub struct Texture {
    bitmap: Bitmap<u8>,
    palette: Vec<RGBA8>,
}

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
    pub fn get(self: &Self, i: i32, j: i32) -> &RGBA<u8> {
        let i = (i as usize) & 1023;
        let j = (j as usize) & 1023;
        unsafe {
            self.palette
                .get_unchecked(self.bitmap.buffer[i + self.width() * j] as usize)
        }
    }

    pub fn from_file(path: &str) -> Result<Texture, String> {
        let mut state = lodepng::State::default();
        state.info_raw_mut().colortype = ColorType::PALETTE;

        match state.decode_file(path) {
            Err(e) => Err(format!("Error opening the file {} ({})", path, e.0)),
            Ok(image) => match image {
                lodepng::Image::RawData(im) => {
                    if !state.info_png.color.is_palette_type() {
                        Err("Texture must contain a palette".to_string())
                    } else if (im.width, im.height) != (1024, 1024) {
                        Err(format!(
                            "Texture size must be exactly 1024x1024, is {}x{})",
                            im.width, im.height
                        ))
                    } else {
                        Ok(Texture::new(im, &state.info_png.color))
                    }
                }
                _ => Err(format!(
                    "Not the right format, expect RGBA Palette 8 bits. Was {:?}",
                    image
                )),
            },
        }
    }

    pub fn new(data: Bitmap<u8>, color_mode: &ColorMode) -> Texture {
        Texture {
            bitmap: data,
            palette: Vec::from(color_mode.palette()),
        }
    }

    pub fn width(self: &Self) -> usize {
        1024
    }
}
