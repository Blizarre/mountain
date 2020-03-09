use std::thread::sleep;
use std::time::Duration;

use crate::terrain::{HeightMap, Texture};
use rgb;
use rgb::RGBA8;
use sdl;
use sdl::event::Key::Escape;
use sdl::event::{poll_event, Event, Key, MouseState};
use sdl::mouse::set_cursor_visible;
use sdl::video::{set_video_mode, Surface, SurfaceFlag, VideoFlag};
use sdl::{quit, InitFlag};
use std::cmp::{max, min};

mod stats;
mod terrain;

fn get_pitch(surface: &Surface) -> u16 {
    unsafe { (*surface.raw).pitch }
}

mod others {
    #[link(name = "SDL")]
    #[link(name = "asound")]
    extern "C" {}
}

struct Camera {
    x: f32,
    y: f32,
    z: i32,
    horizon: i32,
    angle: f32,
}

struct Vector2<T> {
    x: T,
    y: T,
}

fn process_events(camera: &mut Camera) -> (bool, f32, f32) {
    let mut displacement = Vector2 { x: 0, y: 0 };
    let mut request_exit = false;

    loop {
        let evt = poll_event();
        match evt {
            Event::None => break,
            Event::Quit => request_exit = true,
            Event::MouseMotion(state, _, _, xrel, yrel) => {
                if state.contains(&MouseState::Left) {
                    camera.angle -= (xrel as f32) / 100.;
                    camera.horizon += yrel as i32;
                } else {
                    displacement.x += xrel;
                    displacement.y += yrel;
                }
            }
            Event::Key(k, false, _, _) => {
                println!("keypress: {:?}", k as usize);
                match k {
                    Escape => request_exit = true,
                    Key::Left => camera.angle += 0.1,
                    Key::Right => camera.angle -= 0.1,
                    Key::Up => displacement.y -= 5,
                    Key::Down => displacement.y += 5,
                    Key::PageUp => camera.z += 5,
                    Key::PageDown => camera.z -= 5,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    let cos_angle = camera.angle.cos();
    let sin_angle = camera.angle.sin();

    camera.x += (displacement.x as f32) * cos_angle - (displacement.y as f32) * sin_angle;
    camera.y += (displacement.x as f32) * sin_angle + (displacement.y as f32) * cos_angle;

    (request_exit, cos_angle, sin_angle)
}

fn main() {
    println!("Starting");
    let map = match HeightMap::from_file("hm.png") {
        Err(e) => {
            println!("Cannot open the map: {}", e);
            return;
        }
        Ok(im) => im,
    };

    let texture = match Texture::from_file("tx.png") {
        Err(e) => {
            println!("Cannot open the texture: {}", e);
            return;
        }
        Ok(im) => im,
    };

    sdl::init([InitFlag::Video].as_ref());
    let screen = set_video_mode(
        1024,
        768,
        32,
        [SurfaceFlag::SWSurface].as_ref(),
        [VideoFlag::Fullscreen].as_ref(),
    )
    .unwrap();
    set_cursor_visible(false);

    let mut request_exit = false;
    let mut frame_ctr = stats::Stats::default();
    let mut draw_ctr = stats::Stats::default();

    let mut camera = Camera {
        x: 240.,
        y: 277.,
        z: 150,
        horizon: 120,
        angle: 0.,
    };

    while !request_exit {
        frame_ctr.start_event();

        let (should_exit, cos_angle, sin_angle) = process_events(&mut camera);

        if should_exit {
            request_exit = true;
        }

        screen.clear();

        draw_ctr.time(|| {
            draw(&screen, &map, &texture, &camera, sin_angle, cos_angle);
        });

        screen.flip();
        let ms_elapsed = frame_ctr.end_event();

        if ms_elapsed < 16 {
            sleep(Duration::from_millis(16 - ms_elapsed as u64));
        }
    }
    println!("Frame stats: {}", frame_ctr);
    println!("Draw stats: {}", draw_ctr);
    quit();
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

fn draw(
    screen: &Surface,
    map: &terrain::HeightMap,
    texture: &terrain::Texture,
    camera: &Camera,
    sin_angle: f32,
    cos_angle: f32,
) {
    let (screen_w, screen_h) = screen.get_size();
    let screen_w = screen_w as i32;
    let screen_h = screen_h as i32;

    let pitch = get_pitch(&screen) as usize;

    let fixed_precision = 11;
    let precision_multipier = 2048.;

    let fixed_horizon = camera.horizon << fixed_precision;
    let scale_height_shift = 7;
    let distance_max = 300;

    screen.with_lock(|screen_pixels| {
        let mut max_height = Vec::new();
        max_height.resize(screen_w as usize, 0);

        for z in 1..distance_max {
            let zf = z as f32;
            let left = (
                (-cos_angle * zf - sin_angle * zf) + camera.x,
                (sin_angle * zf - cos_angle * zf) + camera.y,
            );

            let right = (
                (cos_angle * zf - sin_angle * zf) + camera.x,
                (-sin_angle * zf - cos_angle * zf) + camera.y,
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
