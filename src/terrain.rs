use crate::fixed_int::FixedInt10;
use lodepng::{Bitmap, ColorType, Grey};
use rgb::{RGBA, RGBA8};

pub struct HeightMap(Bitmap<Grey<u8>>);

pub struct Texture(Bitmap<RGBA8>);

// We hardcode a size of 1024 for the heightmap / texture

impl HeightMap {
    pub fn get(&self, i: FixedInt10, j: FixedInt10) -> FixedInt10 {
        let i = Into::<usize>::into(i.floor()) & 1023;
        let j = Into::<usize>::into(j.floor()) & 1023;
        self.0.buffer[i + self.width() * j].0.into()
    }

    pub fn get_interpolate(&self, i: FixedInt10, j: FixedInt10) -> FixedInt10 {
        let i0 = Into::<usize>::into(i.floor()) & 1023;
        let i1 = (i0 + 1) & 1023;
        let i: FixedInt10 = i.fract();
        let ic = FixedInt10::from(1) - i;

        let j0 = Into::<usize>::into(j.floor()) & 1023;
        let j1 = (j0 + 1) & 1023;
        let j: FixedInt10 = j.fract();
        let jc = FixedInt10::from(1) - j;

        let f00: FixedInt10 = self.0.buffer[i0 + self.width() * j0].0.into();
        let f10: FixedInt10 = self.0.buffer[i1 + self.width() * j0].0.into();
        let f01: FixedInt10 = self.0.buffer[i0 + self.width() * j1].0.into();
        let f11: FixedInt10 = self.0.buffer[i1 + self.width() * j1].0.into();

        // See https://en.wikipedia.org/wiki/Bilinear_interpolation#Unit_square
        f00 * ic * jc + f10 * i * jc + f01 * ic * j + f11 * i * j
    }

    pub fn from_file(path: &str) -> Result<HeightMap, String> {
        match lodepng::decode_file(path, ColorType::GREY, 8) {
            Err(e) => Err(format!("Error opening the file {} ({})", path, e)),
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

    pub fn width(&self) -> usize {
        1024
    }
}

#[cfg(test)]
mod tests {
    use crate::fixed_int::FixedInt10;
    use crate::terrain::HeightMap;
    use lodepng::Bitmap;
    use rgb::alt::Gray;

    fn sample_map() -> HeightMap {
        let mut data = vec![Gray(0u8); 1024 * 1024];
        data[0 + 1024 * 0] = Gray(1u8);
        data[1 + 1024 * 0] = Gray(2u8);
        data[0 + 1024 * 1] = Gray(3u8);
        data[1 + 1024 * 1] = Gray(4u8);

        data[1023 + 1024 * 1023] = Gray(5u8);

        HeightMap::from(Bitmap {
            height: 2,
            width: 2,
            buffer: data,
        })
    }

    #[test]
    fn get_boundaries() {
        let map = sample_map();
        assert_eq!(map.get_interpolate(0.into(), 0.into()), 1.into());
        assert_eq!(map.get_interpolate(1.into(), 0.into()), 2.into());
        assert_eq!(map.get_interpolate(0.into(), 1.into()), 3.into());
        assert_eq!(map.get_interpolate(1.into(), 1.into()), 4.into());

        assert_eq!(map.get_interpolate((1024 + 1).into(), 1.into()), 4.into());
        assert_eq!(
            map.get_interpolate((1024 + 1).into(), (1024 + 1).into()),
            4.into()
        );
        assert_eq!(map.get_interpolate((-1).into(), (-1).into()), 5.into());
    }

    #[test]
    fn get_interpolation() {
        let map = sample_map();
        assert_eq!(
            map.get_interpolate((0.5f32).into(), (0.5f32).into()),
            FixedInt10::from(1 + 2 + 3 + 4) / 4
        );
    }
}

impl Texture {
    pub fn get(&self, i: FixedInt10, j: FixedInt10) -> RGBA<u8> {
        let i = Into::<usize>::into(i) & 1023;
        let j = Into::<usize>::into(j) & 1023;
        self.0.buffer[i + self.width() * j]
    }

    fn u8_to_fixed_int_rgb(val: RGBA<u8>) -> RGBA<FixedInt10> {
        RGBA {
            r: val.r.into(),
            g: val.g.into(),
            b: val.b.into(),
            a: val.a.into(),
        }
    }

    pub fn get_interpolate(&self, i: FixedInt10, j: FixedInt10) -> RGBA<u8> {
        let i0 = Into::<usize>::into(i.floor()) & 1023;
        let i1 = (i0 + 1) & 1023;
        let i: FixedInt10 = i.fract();
        let ic = FixedInt10::from(1) - i;

        let j0 = Into::<usize>::into(j.floor()) & 1023;
        let j1 = (j0 + 1) & 1023;
        let j: FixedInt10 = j.fract();
        let jc = FixedInt10::from(1) - j;

        let f00 = Self::u8_to_fixed_int_rgb(self.0.buffer[i0 + self.width() * j0]);
        let f10 = Self::u8_to_fixed_int_rgb(self.0.buffer[i1 + self.width() * j0]);
        let f01 = Self::u8_to_fixed_int_rgb(self.0.buffer[i0 + self.width() * j1]);
        let f11 = Self::u8_to_fixed_int_rgb(self.0.buffer[i1 + self.width() * j1]);

        // See https://en.wikipedia.org/wiki/Bilinear_interpolation#Unit_square
        RGBA {
            r: (f00.r * ic * jc + f10.r * i * jc + f01.r * ic * j + f11.r * i * j).into(),
            g: (f00.g * ic * jc + f10.g * i * jc + f01.g * ic * j + f11.g * i * j).into(),
            b: (f00.b * ic * jc + f10.b * i * jc + f01.b * ic * j + f11.b * i * j).into(),
            a: (f00.a).into(),
        }
    }

    pub fn from_file(path: &str) -> Result<Texture, String> {
        match lodepng::decode_file(path, ColorType::RGBA, 8) {
            Err(e) => Err(format!("Error opening the file {} ({})", path, e)),
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

    pub fn width(&self) -> usize {
        1024
    }
}
