use image::{GrayImage, ImageBuffer, Luma};
use num::complex::Complex;
use std::convert::TryInto;

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            //println!("{} | {}", Complex::new(cx, cy), escaped_at);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    //println!("{}, {}", cx, cy);
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        //println!("{}", z.norm());
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>, max_iters: usize, width: u32, height: u32) {
    let mut img: GrayImage = ImageBuffer::new(width, height);
    let mut y = 0;
    for row in escape_vals {
        let mut x = 0;
        for column in row {
            let val: u8 = (column * 255 / max_iters).try_into().unwrap();
            img.put_pixel(x, y, Luma([val]));
            x += 1;
        }
        y += 1;
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
