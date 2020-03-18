#[macro_use]
extern crate criterion;
extern crate sdl;

use mountain::{camera, draw, terrain};

use criterion::{black_box, Criterion};
use mountain::draw::Settings;
use sdl::video::SurfaceFlag;
use std::time::Duration;

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

    let screen = sdl::video::Surface::new(
        &[SurfaceFlag::SWSurface],
        800,
        600,
        32,
        0xff0000,
        0x00ff00,
        0x0000ff,
        0x000000ff,
    )
    .unwrap();

    let camera = camera::Camera::new(500., 400., 200, 2 * screen.get_height() as i32 / 3);

    c.bench_function("draw_fog", |b| {
        b.iter(|| {
            draw::draw(
                black_box(&screen),
                &map,
                &texture,
                black_box(&camera),
                &Settings { fog: true },
            )
        })
    });

    c.bench_function("draw_nofog", |b| {
        b.iter(|| {
            draw::draw(
                black_box(&screen),
                &map,
                &texture,
                black_box(&camera),
                &Settings { fog: false },
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
