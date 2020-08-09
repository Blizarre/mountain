extern crate mountain;
extern crate toml;

use std::thread::sleep;
use std::time::Duration;

use sdl::event::Key::Escape;
use sdl::event::{poll_event, Event, Key};
use sdl::mouse::set_cursor_visible;
use sdl::video::{set_video_mode, SurfaceFlag, VideoFlag};
use sdl::{quit, InitFlag};

use mountain::vector::Vector2;

use mountain::camera::Camera;
use mountain::config::{Config, ConfigError};
use mountain::fixed_int::FixedInt10;
use mountain::renderer::draw;
use mountain::stats::Stats;
use mountain::terrain::{HeightMap, Texture};
use sdl::wm::{grab_input, GrabMode};

mod others {
    #[link(name = "SDL")]
    #[link(name = "asound")]
    extern "C" {}
}

#[derive(Default, Copy, Clone)]
struct KeyPressedState {
    pub left_pressed: bool,
    pub back_pressed: bool,
    pub forward_pressed: bool,
    pub right_pressed: bool,
    // During the first PollEvent, SDL will report the current location of the mouse as a relative
    // motion. This is why we need to ignore it as we are really only interested in relative motion
    // from frame to frame
    pub motion_initialized: bool,
}

fn process_events(
    camera: &mut Camera,
    config: &mut Config,
    key_state: &mut KeyPressedState,
) -> bool {
    let mut displacement = Vector2 { x: 0, y: 0 };
    let mut request_exit = false;

    // If we receive a KEYDOWN event followed by a KEYUP event in the same fram, they would cancel
    // each other and we would not register the movement. This State object is only activated
    // on KeyDown and is not kept for the following frame
    let mut single_tap = KeyPressedState::default();

    // SDL will fire multiple Mouse events for each frame, so we add all the motion into this
    // variable to do all the complex computations (sin/cos) once at the end
    let mut mouse_motion: Vector2<i16> = Vector2::default();

    loop {
        let evt = poll_event();
        match evt {
            Event::None => break,
            Event::Quit => request_exit = true,
            Event::MouseMotion(_, _, _, xrel, yrel) => {
                if key_state.motion_initialized {
                    mouse_motion.x += xrel;
                    mouse_motion.y += yrel;
                } else {
                    key_state.motion_initialized = true;
                }
            }
            Event::Key(k, pressed, _, _) => {
                println!("keypress: {:?}, {}", k as usize, pressed);
                match k {
                    Escape => request_exit = true,
                    Key::A => {
                        if pressed {
                            single_tap.left_pressed = true;
                        }
                        key_state.left_pressed = pressed
                    }
                    Key::D => {
                        if pressed {
                            single_tap.right_pressed = true;
                        }
                        key_state.right_pressed = pressed
                    }
                    Key::W => {
                        if pressed {
                            single_tap.forward_pressed = true;
                        }
                        key_state.forward_pressed = pressed
                    }
                    Key::S => {
                        if pressed {
                            single_tap.back_pressed = true;
                        }
                        key_state.back_pressed = pressed
                    }
                    Key::B => {
                        if pressed {
                            config.renderer.enable_filtering =
                                !config.renderer.enable_filtering;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    camera.update_angle(-(config.player.sensitivity_x * mouse_motion.x as f32) / 100.);
    camera.horizon += (config.player.sensitivity_y * mouse_motion.y as f32) as i32;

    if key_state.left_pressed || single_tap.left_pressed {
        displacement.x -= config.player.speed
    }
    if key_state.right_pressed || single_tap.right_pressed {
        displacement.x += config.player.speed
    }
    if key_state.forward_pressed || single_tap.forward_pressed {
        displacement.y -= config.player.speed
    }
    if key_state.back_pressed || single_tap.back_pressed {
        displacement.y += config.player.speed
    }

    camera.x +=
        (displacement.x as f32) * camera.cos_angle + (displacement.y as f32) * camera.sin_angle;
    camera.y +=
        (displacement.x as f32) * camera.sin_angle + (displacement.y as f32) * camera.cos_angle;

    request_exit
}

fn main() {
    println!("Loading configuration");

    let mut config = match mountain::config::Config::from_config("mountain.toml") {
        Ok(c) => c,
        Err(ConfigError { message }) => {
            println!("Cannot read config file mountain.toml: {}", message);
            return;
        }
    };

    println!("Loading textures");

    let map = match HeightMap::from_file(config.map.heightmap.as_str()) {
        Err(e) => {
            println!("Cannot open the map: {}", e);
            return;
        }
        Ok(im) => im,
    };

    let texture = match Texture::from_file(config.map.texture.as_str()) {
        Err(e) => {
            println!("Cannot open the texture: {}", e);
            return;
        }
        Ok(im) => im,
    };

    sdl::init([InitFlag::Video].as_ref());

    let screen = set_video_mode(
        config.screen.width as isize,
        config.screen.height as isize,
        32,
        [SurfaceFlag::SWSurface].as_ref(),
        [VideoFlag::Fullscreen].as_ref(),
    )
    .unwrap();
    set_cursor_visible(false);
    grab_input(GrabMode::On);

    let mut request_exit = false;
    let mut frame_ctr = Stats::default();
    let mut draw_ctr = Stats::default();
    let mut key_pressed = KeyPressedState::default();

    let mut camera = Camera::new(500., 400., 200.into(), screen.get_height() as i32 / 2);

    while !request_exit {
        frame_ctr.start_event();

        if process_events(&mut camera, &mut config, &mut key_pressed) {
            request_exit = true;
        }
        camera.z = FixedInt10::from(config.player.height)
            + map.get_interpolate(camera.x.into(), camera.y.into());

        draw_ctr.time(|| {
            draw(&screen, &map, &texture, &camera, &config.renderer);
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
