use crate::camera::Camera;
use crate::config::RendererConfig;
use crate::fixed_int::FixedInt10;
use crate::terrain;
use crate::vector::Vector2;
use rgb::RGBA8;
use sdl::video::{Color, Surface};
use std::cmp::{max, min};

fn get_pitch(surface: &Surface) -> u16 {
    unsafe { (*surface.raw).pitch }
}

fn set_color(image: &mut [u8], i: usize, j: usize, pitch: usize, value: RGBA8) {
    let pixel_offset = i * 4 + j * pitch;
    image[pixel_offset] = value.b;
    image[pixel_offset + 1] = value.g;
    image[pixel_offset + 2] = value.r;
}

fn draw_line(
    image: &mut [u8],
    i: usize,
    jmin: usize,
    jmax: usize,
    image_h: usize,
    pitch: usize,
    value: RGBA8,
) {
    for height in jmin..jmax {
        set_color(image, i, image_h - height - 1, pitch, value);
    }
}

pub fn draw(
    screen: &Surface,
    map: &terrain::HeightMap,
    texture: &terrain::Texture,
    camera: &Camera,
    config: &RendererConfig,
) {
    let screen_w = screen.get_width() as i32;
    let screen_h = screen.get_height() as i32;

    let pitch = get_pitch(&screen) as usize;
    let sky = RGBA8::new(80, 120, 250, 0);

    screen.fill(Color::RGB(sky.r, sky.g, sky.b));

    let horizon = FixedInt10::from(camera.horizon);
    let scale_height = screen_h;

    screen.with_lock(|screen_pixels| {
        let mut max_height = Vec::new();
        max_height.resize(screen_w as usize, 0);

        for z in 1..config.distance_max {
            let zf = z as f32;
            let left = Vector2 {
                x: FixedInt10::from((-camera.cos_angle * zf - camera.sin_angle * zf) + camera.x),
                y: FixedInt10::from((camera.sin_angle * zf - camera.cos_angle * zf) + camera.y),
            };

            let right = Vector2 {
                x: FixedInt10::from((camera.cos_angle * zf - camera.sin_angle * zf) + camera.x),
                y: FixedInt10::from((-camera.sin_angle * zf - camera.cos_angle * zf) + camera.y),
            };

            let stride = Vector2 {
                x: (right.x - left.x) / screen_w,
                y: (right.y - left.y) / screen_w,
            };

            for i in 0..screen_w as i32 {
                let height_on_hm = map.get(
                    (left.x + stride.x * i).into(),
                    (left.y + stride.y * i).into(),
                );

                let real_height: FixedInt10 = ((FixedInt10::from(height_on_hm) - camera.z)
                    // trick here: scale_height AND z should be brought to fixed float, however
                    // the (<< PRECISION) cancel each other
                    * scale_height)
                    / z
                    + horizon;

                let real_height: i32 = max(0, real_height.into());

                if real_height > max_height[i as usize] {
                    let texture_value = texture.get(
                        (left.x + stride.x * i).into(),
                        (left.y + stride.y * i).into(),
                    );

                    let texture_value = if config.fog && z > config.fog_start {
                        let sky_weight = FixedInt10::from(z - config.fog_start)
                            / (config.distance_max - config.fog_start);
                        let texture_weight = FixedInt10::from(1) - sky_weight;

                        RGBA8 {
                            r: (texture_weight * texture_value.r + sky_weight * sky.r).into(),
                            g: (texture_weight * texture_value.g + sky_weight * sky.g).into(),
                            b: (texture_weight * texture_value.b + sky_weight * sky.b).into(),
                            a: 0,
                        }
                    } else {
                        texture_value
                    };

                    draw_line(
                        screen_pixels,
                        i as usize,
                        max_height[i as usize] as usize,
                        min(real_height, screen_h) as usize,
                        screen_h as usize,
                        pitch,
                        texture_value,
                    );
                    max_height[i as usize] = real_height
                }
            }
        }
        true
    });
}
