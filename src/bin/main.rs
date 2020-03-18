extern crate mountain;

use std::thread::sleep;
use std::time::Duration;

use sdl::event::Key::Escape;
use sdl::event::{poll_event, Event, Key, MouseState};
use sdl::mouse::set_cursor_visible;
use sdl::video::{set_video_mode, SurfaceFlag};
use sdl::{quit, InitFlag};

use mountain::vector::Vector2;

use mountain::camera::Camera;
use mountain::draw::{draw, Settings};
use mountain::stats::Stats;
use mountain::terrain::{HeightMap, Texture};

use sdl::video::VideoFlag::Fullscreen;
use std::f32::consts::PI;

mod others {
    #[link(name = "SDL")]
    #[link(name = "asound")]
    extern "C" {}
}

fn process_events(camera: &mut Camera) -> bool {
    let mut displacement = Vector2 { x: 0, y: 0 };
    let mut request_exit = false;

    loop {
        let evt = poll_event();
        match evt {
            Event::None => break,
            Event::Quit => request_exit = true,
            Event::MouseMotion(state, _, _, xrel, yrel) => {
                if state.contains(&MouseState::Left) {
                    camera.update_angle(-(xrel as f32) / 100.);
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
                    Key::Left => camera.update_angle(0.1),
                    Key::Right => camera.update_angle(-0.1),
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

    camera.x +=
        (displacement.x as f32) * camera.cos_angle - (displacement.y as f32) * camera.sin_angle;
    camera.y +=
        (displacement.x as f32) * camera.sin_angle + (displacement.y as f32) * camera.cos_angle;

    request_exit
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
    let mut frame_ctr = Stats::default();
    let mut draw_ctr = Stats::default();

    let mut camera = Camera::new(500., 400., 200, 2 * screen.get_height() as i32 / 3);

    while !request_exit {
        frame_ctr.start_event();

        if process_events(&mut camera) {
            request_exit = true;
        }

        draw_ctr.time(|| {
            draw::draw(&screen, &map, &texture, &camera);
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
