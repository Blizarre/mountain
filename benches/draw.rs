#[macro_use]
extern crate criterion;
extern crate sdl;

use mountain::{camera, renderer, terrain};

use criterion::{black_box, Criterion};
use mountain::config::RendererConfig;
use sdl::video::SurfaceFlag;
use std::time::Duration;

// TODO: Refactor for slow/fast processor
pub fn draw_bench(c: &mut Criterion) {
    let bench_config_fast = RendererConfig {
        fog_start: 300,
        fog: true,
        distance_max: 350,
    };

    let bench_config_slow = RendererConfig {
        fog_start: 1000,
        fog: true,
        distance_max: 1100,
    };

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
        320,
        240,
        32,
        0xff0000,
        0x00ff00,
        0x0000ff,
        0x000000ff,
    )
    .unwrap();

    let screen_slow = sdl::video::Surface::new(
        &[SurfaceFlag::SWSurface],
        1920,
        1080,
        32,
        0xff0000,
        0x00ff00,
        0x0000ff,
        0x000000ff,
    )
    .unwrap();

    let camera_slow = camera::Camera::new(
        500.,
        400.,
        200.into(),
        2 * screen_slow.get_height() as i32 / 3,
    );
    let camera_fast = camera::Camera::new(
        500.,
        400.,
        200.into(),
        2 * screen_fast.get_height() as i32 / 3,
    );

    c.bench_function("draw_fast", |b| {
        b.iter(|| {
            renderer::draw(
                black_box(&screen_fast),
                &map,
                &texture,
                black_box(&camera_fast),
                black_box(&bench_config_fast),
            )
        })
    });

    c.bench_function("draw_slow", |b| {
        b.iter(|| {
            renderer::draw(
                black_box(&screen_slow),
                &map,
                &texture,
                black_box(&camera_slow),
                black_box(&bench_config_slow),
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
