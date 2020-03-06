use std::thread::sleep;
use std::time::Duration;

use sdl;
use sdl::{get_ticks, InitFlag, quit};
use sdl::event::{Event, poll_event};
use sdl::event::Key::Escape;
use sdl::video::{set_video_mode, Surface, SurfaceFlag, VideoFlag};

mod stats;

fn get_pitch(surface: &Surface) -> u16 {
    unsafe { (*surface.raw).pitch }
}

mod others {
    #[link(name = "SDL")]
    #[link(name = "asound")]
    extern {}
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
    ).unwrap();


    let mut request_exit = false;
    let mut frame_ctr = stats::Stats::default();

    while !request_exit {
        let tick = get_ticks();

        // Process the events
        loop {
            let evt = poll_event();
            match evt {
                Event::None => break,
                Event::Quit => request_exit = true,
                Event::Key(k, true, _, _) => {
                    println!("keypress: {:?}", k as usize);
                    match k {
                        Escape => request_exit = true,
                        _ => ()
                    }
                }
                _ => ()
            }
        }
        draw(&screen);
        screen.flip();
        let ms_elapsed = (get_ticks() - tick) as u32;
        frame_ctr.add(ms_elapsed);

        if ms_elapsed < 16 {
            sleep(Duration::from_millis(16 - ms_elapsed as u64));
        }
    }
    println!("Frame stats: {}", frame_ctr);
    quit();
}


fn draw(screen: &Surface) {
    screen.with_lock(|f| {
        let (w, h) = screen.get_size();
        let pitch = get_pitch(&screen) as usize;

        for j in 0..h as usize {
            for i in 0..w as usize {
                f[i * 4 + j * pitch + 0] = (j % 255) as u8;
                f[i * 4 + j * pitch + 1] = (i % 255) as u8;
                f[i * 4 + j * pitch + 2] = ((i + j) % 255) as u8;
                f[i * 4 + j * pitch + 3] = 0;
            }
        }
        true
    });
}