use crate::{terrain, Camera};
use rgb::RGBA8;
use sdl::video::Surface;
use std::cmp::{max, min};

fn get_pitch(surface: &Surface) -> u16 {
    unsafe { (*surface.raw).pitch }
}

fn set_color(image: &mut [u8], i: usize, j: usize, pitch: usize, value: RGBA8) {
    let pixel_offset = i * 4 + j * pitch;
    image[pixel_offset] = value.r;
    image[pixel_offset + 1] = value.g;
    image[pixel_offset + 2] = value.b;
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
) {
    let (screen_w, screen_h) = screen.get_size();
    let screen_w = screen_w as i32;
    let screen_h = screen_h as i32;

    let pitch = get_pitch(&screen) as usize;

    let fixed_precision = 11;
    let precision_multipier = 2048.;

    let fixed_horizon = camera.horizon << fixed_precision;
    let scale_height_shift = 7;
    let distance_max = 500;

    screen.with_lock(|screen_pixels| {
        let mut max_height = Vec::new();
        max_height.resize(screen_w as usize, 0);

        for z in 1..distance_max {
            let zf = z as f32;
            let left = (
                (-camera.cos_angle * zf - camera.sin_angle * zf) + camera.x,
                (camera.sin_angle * zf - camera.cos_angle * zf) + camera.y,
            );

            let right = (
                (camera.cos_angle * zf - camera.sin_angle * zf) + camera.x,
                (-camera.sin_angle * zf - camera.cos_angle * zf) + camera.y,
            );

            let stride = (
                (right.0 - left.0) / screen_w as f32,
                (right.1 - left.1) / screen_w as f32,
            );

            let fixed_left = (
                (left.0 * precision_multipier) as i32,
                (left.1 * precision_multipier) as i32,
            );
            let fixed_stride = (
                (stride.0 * precision_multipier) as i32,
                (stride.1 * precision_multipier) as i32,
            );

            for i in 0..screen_w as i32 {
                let height_on_hm = map.get(
                    (fixed_left.0 + i * fixed_stride.0) >> fixed_precision,
                    (fixed_left.1 + i * fixed_stride.1) >> fixed_precision,
                ) as i32;
                let fixed_height_on_hm = height_on_hm << fixed_precision;

                let texture_value = texture.get(
                    (fixed_left.0 + i * fixed_stride.0) >> fixed_precision,
                    (fixed_left.1 + i * fixed_stride.1) >> fixed_precision,
                );

                let fixed_real_height = ((fixed_height_on_hm - (camera.z << fixed_precision))
                    // trick here: scale_height AND z should be brought to fixed float, however
                    // the (<< PRECISION) cancel each other
                    << scale_height_shift)
                    / z
                    + fixed_horizon;

                let real_height = max(0, fixed_real_height >> fixed_precision);

                if real_height > max_height[i as usize] {
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
