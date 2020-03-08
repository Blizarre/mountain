use std::thread::sleep;
use std::time::Duration;

use crate::terrain::{HeightMap, Texture};
use rgb;
use rgb::RGBA8;
use sdl;
use sdl::event::Key::Escape;
use sdl::event::{poll_event, Event, Key};
use sdl::mouse::set_cursor_visible;
use sdl::video::{set_video_mode, Surface, SurfaceFlag};
use sdl::{get_ticks, quit, InitFlag};
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
    let screen =
        set_video_mode(320, 240, 32, [SurfaceFlag::SWSurface].as_ref(), [].as_ref()).unwrap();
    set_cursor_visible(false);

    let mut request_exit = false;
    let mut frame_ctr = stats::Stats::default();
    let mut draw_ctr = stats::Stats::default();

    let mut camera_location = (240., 277., 150.);
    let mut angle = 0.;

    while !request_exit {
        let tick_start_frame = get_ticks();
        // Process the events
        loop {
            let evt = poll_event();
            match evt {
                Event::None => break,
                Event::Quit => request_exit = true,
                Event::Key(k, false, _, _) => {
                    println!("keypress: {:?}", k as usize);
                    match k {
                        Escape => request_exit = true,
                        Key::Left => angle += 0.1,
                        Key::Right => angle -= 0.1,
                        Key::Up => camera_location.1 -= 5.,
                        Key::Down => camera_location.1 += 5.,
                        Key::PageUp => camera_location.2 += 5.,
                        Key::PageDown => camera_location.2 -= 5.,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        screen.clear();
        let tick_start_draw = get_ticks();
        draw(&screen, &map, &texture, camera_location, angle);
        draw_ctr.add(get_ticks() - tick_start_draw);

        screen.flip();
        let ms_elapsed = get_ticks() - tick_start_frame;
        frame_ctr.add(ms_elapsed);

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

fn draw_line(image: &mut [u8], i: usize, jmin: usize, jmax: usize, image_h: usize, pitch: usize, value: RGBA8) {
    for height in jmin..jmax {
        set_color(image, i, image_h - height - 1, pitch, value);
    }
}

fn draw(
    screen: &Surface,
    map: &terrain::HeightMap,
    texture: &terrain::Texture,
    camera_location: (f32, f32, f32),
    angle: f32,
) {
    let (screen_w, screen_h) = screen.get_size();
    let screen_w = screen_w as i32;
    let screen_h = screen_h as usize;

    let pitch = get_pitch(&screen) as usize;

    let horizon = 120.;
    let scale_height = 60.;
    let distance_max = 300;

    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    screen.with_lock(|screen_pixels| {
        let mut max_height = Vec::new();
        max_height.resize(screen_w as usize, 0);

        for z in 1..distance_max {
            let z = z as f32;
            let left = (
                (-cos_angle * z - sin_angle * z) + camera_location.0,
                (sin_angle * z - cos_angle * z) + camera_location.1
            );

            let right = (
                (cos_angle * z - sin_angle * z) + camera_location.0,
                (-sin_angle * z - cos_angle * z) + camera_location.1
            );

            let stride = (
                (right.0 - left.0) / screen_w as f32,
                (right.1 - left.1) / screen_w as f32
            );

            for i in 0..screen_w as usize {
                let i_float = i as f32;
                let height_on_hm = map.get(
                    (left.0 + i_float * stride.0) as i32,
                    (left.1 + i_float * stride.1) as i32,
                );

                let texture_value = texture.get(
                    (left.0 + i_float * stride.0) as i32,
                    (left.1 + i_float * stride.1) as i32,
                );

                let real_height = (height_on_hm as i32 - camera_location.2 as i32) as f32
                    / z as f32
                    * scale_height
                    + horizon;

                let real_height = max(0, real_height as i32) as usize;

                if real_height > max_height[i] {
                    draw_line(
                        screen_pixels,
                        i,
                        max_height[i],
                        min(real_height, screen_h),
                        screen_h,
                        pitch,
                        texture_value,
                    );
                    max_height[i] = real_height
                }
            }
        }
        true
    });
}
