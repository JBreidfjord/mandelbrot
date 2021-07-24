use image::{GrayImage, ImageBuffer, Luma};
use num::complex::Complex;
use rayon::prelude::*;
use std::convert::TryInto;

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<usize> {
    let mut pixels = vec![0; width * height];
    let rows: Vec<(usize, &mut [usize])> = pixels.chunks_mut(width).enumerate().collect();
    rows.into_par_iter().for_each(|(img_y, row)| {
        let y_percent = img_y as f64 / height as f64;
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row[img_x] = escaped_at;
        }
    });
    pixels
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(pixels: Vec<usize>, max_iters: usize, width: u32, height: u32) {
    let mut img: GrayImage = ImageBuffer::new(width, height);
    let mut i = 0;
    for pixel in pixels {
        let y = i / width;
        let x = i - (y * width);
        let val: u8 = (pixel * 255 / max_iters).try_into().unwrap();
        img.put_pixel(x, y, Luma([val]));
        i += 1;
    }
    img.save(format!("mb_{}_{}x{}.png", max_iters, width, height))
        .unwrap();
}

fn main() {
    let max_iters = 100;
    let width = 1920;
    let height = 1080;

    let mandelbrot = calculate_mandelbrot(max_iters, -2.0, 1.0, -1.0, 1.0, width, height);
    render_mandelbrot(
        mandelbrot,
        max_iters,
        width.try_into().unwrap(),
        height.try_into().unwrap(),
    );
}
