use std::cmp::Reverse;
use std::collections::HashMap;

use image::{Pixel, RgbaImage};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use screenshots::Screen;

use crate::shared::*;

pub fn mean_rgb(colors: &[color::Rgb]) -> Vec<color::Rgb> {
    let (mut r, mut g, mut b) = (0, 0, 0);
    let n = colors.len() as u32;

    for i in colors.iter() {
        r += i.0[0] as u32;
        g += i.0[1] as u32;
        b += i.0[2] as u32;
    }

    vec![color::Rgb([(r / n) as u8, (g / n) as u8, (b / n) as u8])]
}

pub fn k_means_clustring_cielab(colors: &[color::Rgb]) -> Vec<color::Rgb> {
    todo!()
}

pub fn k_means_clustring_rgb(colors: &[color::Rgb]) -> Vec<color::Rgb> {
    todo!()
}

pub fn median_cut_cielab(colors: &[color::Rgb]) -> Vec<color::Rgb> {
    todo!()
}

pub fn median_cut_rgb(colors: &[color::Rgb]) -> Vec<color::Rgb> {
    todo!()
}

pub fn histogram_rgb(colors: &[color::Rgb], splits: u8, results: u32) -> Vec<color::Rgb> {
    use color::Rgb;

    let bucket_amount = u8::MAX / splits;

    let funnel = |x: u8| x / bucket_amount;

    let mut buckets = HashMap::with_capacity((splits * 3) as usize);
    for rgb in colors.iter() {
        buckets
            .entry((funnel(rgb.0[0]), funnel(rgb.0[1]), funnel(rgb.0[2])))
            .or_insert(Vec::new())
            .push(*rgb);
    }

    let mut buckets = buckets.into_values().collect::<Vec<_>>();
    buckets.sort_unstable_by_key(|v| Reverse(v.len()));
    buckets.truncate(results as usize);

    let mean = |v: Vec<Rgb>| {
        let (mut r, mut g, mut b) = (0, 0, 0);
        let n = v.len() as u32;

        for i in v.iter() {
            r += i.0[0] as u32;
            g += i.0[1] as u32;
            b += i.0[2] as u32;
        }

        Rgb([(r / n) as u8, (g / n) as u8, (b / n) as u8])
    };
    buckets.into_iter().map(mean).collect()
}

pub fn population(colors: &[color::Rgb]) -> Vec<color::Rgb> {
    todo!()
}

pub fn screenshot(factor: f32) -> RgbaImage {
    assert!((0. ..=1.0).contains(&factor));
    let mut display_info = Screen::all().unwrap().first().unwrap().display_info;
    display_info.scale_factor = factor;
    let screen = Screen::new(&display_info);
    screen.capture().unwrap()
}

pub fn sample(image: &RgbaImage, samples: u64) -> Vec<color::Rgb> {
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

        let image::Rgb([r, g, b]) = image.get_pixel(x, y).to_rgb();

        sampled.push(color::Rgb([r, g, b]));
    }

    sampled
}
