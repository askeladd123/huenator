use image::{Pixel, Rgb, RgbaImage};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use screenshots::Screen;

use crate::shared::Color;

pub fn mean(colors: &Vec<Color>) -> Vec<Color> {
    let (mut r, mut g, mut b) = (0, 0, 0);
    for color in colors.iter() {
        r += color.r as u32;
        g += color.g as u32;
        b += color.b as u32;
    }

    let n = colors.len() as u32;
    r /= n;
    g /= n;
    b /= n;

    vec![Color {
        r: r as u8,
        g: g as u8,
        b: b as u8,
    }]
}

pub fn screenshot(factor: f32) -> RgbaImage {
    assert!((0. ..=1.0).contains(&factor));
    let mut display_info = Screen::all().unwrap().first().unwrap().display_info;
    display_info.scale_factor = factor;
    let screen = Screen::new(&display_info);
    screen.capture().unwrap()
}

pub fn sample(image: &RgbaImage, samples: u64) -> Vec<Color> {
    let mut sampled = Vec::with_capacity(samples as usize);
    let (w, h) = (image.width(), image.height());

    let (uniform_x, uniform_y) = (
        rand::distributions::Uniform::new(0, w),
        rand::distributions::Uniform::new(0, h),
    );

    let mut rng = rand::thread_rng();

    for _ in 0..samples {
        let x = uniform_x.sample(&mut rng);
        let y = uniform_y.sample(&mut rng);

        let Rgb([r, g, b]) = image.get_pixel(x, y).to_rgb();

        sampled.push(Color { r, g, b });
    }

    sampled
}
