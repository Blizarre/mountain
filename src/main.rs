use std::thread::sleep;
use std::time::Duration;

use lodepng;
use lodepng::{Bitmap, Grey};
use sdl;
use sdl::{get_ticks, InitFlag, quit};
use sdl::event::{Event, poll_event};
use sdl::event::Key::Escape;
use sdl::mouse::set_cursor_visible;
use sdl::video::{set_video_mode, Surface, SurfaceFlag, VideoFlag};

mod stats;

fn get_pitch(surface: &Surface) -> u16 {
    unsafe { (*surface.raw).pitch }
}

mod others {
    #[link(name = "SDL")]
    #[link(name = "asound")]
    extern "C" {}
}

type Image = Bitmap<Grey<u8>>;

fn load_map(path: &str) -> Result<Image, String> {
    match lodepng::Decoder::new().decode_file(path) {
        Err(e) => Err(format!("Error opening the file {} ({})", path, e.0)),
        Ok(image) => match image {
            lodepng::Image::Grey(im) => Ok(im),
            _ => Err("Not the right format, expect grayscale".to_string()),
        },
    }
}

fn main() {
    println!("Starting");
    sdl::init([InitFlag::Video].as_ref());
    let screen = set_video_mode(
        320,
        240,
        32,
        [SurfaceFlag::SWSurface].as_ref(),
        [VideoFlag::Fullscreen].as_ref(),
    )
        .unwrap();
    set_cursor_visible(false);

    let mut request_exit = false;
    let mut frame_ctr = stats::Stats::default();
    let mut draw_ctr = stats::Stats::default();

    let map = load_map("map.png").expect("Cannot open the map");

    while !request_exit {
        let tick_start_frame = get_ticks();

        // Process the events
        loop {
            let evt = poll_event();
            match evt {
                Event::None => break,
                Event::Quit => request_exit = true,
                Event::Key(k, true, _, _) => {
                    println!("keypress: {:?}", k as usize);
                    if let Escape = k {
                        request_exit = true
                    }
                }
                _ => (),
            }
        }
        let tick_start_draw = get_ticks();
        draw(&screen, &map);
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

fn set_color(image: &mut [u8], i: usize, j: usize, pitch: usize, value: u8) {
    let pixel_offset = i * 4 + j * pitch;
    image[pixel_offset] = value;
    image[pixel_offset + 1] = value;
    image[pixel_offset + 2] = value;
}

fn draw(screen: &Surface, _map: &Image) {
    screen.with_lock(|f| {
        let (w, h) = screen.get_size();
        let pitch = get_pitch(&screen) as usize;

        for j in 0..h as usize {
            for i in 0..w as usize {
                set_color(f, i, j, pitch, (i + j) as u8);
            }
        }
        true
    });
}
