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

pub fn get_image() -> RgbaImage {
    let screens = Screen::all().unwrap();
    screens.first().unwrap().capture().unwrap()
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
        // let x = rng.gen_range(0..w);
        // let y = rng.gen_range(0..h);
        let x = uniform_x.sample(&mut rng);
        let y = uniform_y.sample(&mut rng);

        let Rgb([r, g, b]) = image.get_pixel(x, y).to_rgb();

        sampled.push(Color { r, g, b });
    }

    sampled
}
