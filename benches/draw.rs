#[macro_use]
extern crate criterion;
extern crate sdl;

use mountain::{camera, draw, terrain};

use criterion::{black_box, Criterion};
use mountain::config::{Config, Screen};
use sdl::video::SurfaceFlag;
use std::time::Duration;

const BENCH_FAST: Config = Config {
    fog_start: 300,
    fog: true,
    distance_max: 350,
    screen: Screen {
        width: 320,
        height: 240,
    },
};

const BENCH_SLOW: Config = Config {
    fog_start: 1000,
    fog: true,
    distance_max: 1100,
    screen: Screen {
        width: 1920,
        height: 1080,
    },
};

// TODO: Refactor for slow/fast processor
pub fn draw_bench(c: &mut Criterion) {
    let map = match terrain::HeightMap::from_file("hm.png") {
        Err(e) => {
            println!("Cannot open the map: {}", e);
            return;
        }
        Ok(im) => im,
    };

    let texture = match terrain::Texture::from_file("tx.png") {
        Err(e) => {
            println!("Cannot open the texture: {}", e);
            return;
        }
        Ok(im) => im,
    };

    let screen_fast = sdl::video::Surface::new(
        &[SurfaceFlag::SWSurface],
        BENCH_FAST.screen.width as isize,
        BENCH_FAST.screen.height as isize,
        32,
        0xff0000,
        0x00ff00,
        0x0000ff,
        0x000000ff,
    )
    .unwrap();

    let screen_slow = sdl::video::Surface::new(
        &[SurfaceFlag::SWSurface],
        BENCH_SLOW.screen.width as isize,
        BENCH_SLOW.screen.height as isize,
        32,
        0xff0000,
        0x00ff00,
        0x0000ff,
        0x000000ff,
    )
    .unwrap();

    let camera_slow = camera::Camera::new(500., 400., 200, 2 * screen_slow.get_height() as i32 / 3);
    let camera_fast = camera::Camera::new(500., 400., 200, 2 * screen_fast.get_height() as i32 / 3);

    c.bench_function("draw_fast", |b| {
        b.iter(|| {
            draw::draw(
                black_box(&screen_fast),
                &map,
                &texture,
                black_box(&camera_fast),
                black_box(&BENCH_FAST),
            )
        })
    });

    c.bench_function("draw_slow", |b| {
        b.iter(|| {
            draw::draw(
                black_box(&screen_slow),
                &map,
                &texture,
                black_box(&camera_slow),
                black_box(&BENCH_SLOW),
            )
        })
    });
}

criterion_group!(
    name = benches;
     config = Criterion::default()
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(30));
    targets = draw_bench
);
criterion_main!(benches);
