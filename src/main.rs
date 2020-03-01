use crate::sdl::internal::VideoFlag::Fullscreen;
use crate::sdl::sdl::{init, set_video_mode, quit};
use crate::sdl::internal::SurfaceFlag::SWSurface;

mod sdl;

fn main() {
    println!("Starting");
    init();
    set_video_mode(
        320,
        240,
        32,
        [SWSurface].as_ref(),
        [Fullscreen].as_ref(),
    ).unwrap();
    println!("Quitting");
    quit();
    println!("Thx Bye!");
}