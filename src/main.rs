use image::{GrayImage, ImageBuffer, Luma};
use num::complex::Complex;
use rayon::prelude::*;

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

fn render_mandelbrot(pixels: Vec<usize>, iters: usize, width: usize, height: usize, frame: u32) {
    let mut img: GrayImage = ImageBuffer::new(width as u32, height as u32);
    let mut i = 0;
    for pixel in pixels {
        let y = i / width;
        let x = i - (y * width);
        let val = pixel * 255 / iters;
        img.put_pixel(x as u32, y as u32, Luma([val as u8]));
        i += 1;
    }
    std::fs::create_dir_all(format!("images/{}x{}/", width, height)).unwrap();
    img.save(format!("images/{}x{}/{}.png", width, height, frame))
        .unwrap();
}

fn main() {
    let width = 1920;
    let max_frames = 480;
    let zoom_x = -1.0067581019642513;
    let zoom_y = 0.3112899872556565;
    let rad_mult = 1.03; // Controls speed of zoom based on framerate

    let mut rad_x = 2.0;
    let mut rad_y = 1.0;

    let height = width / 16 * 9;

    for frame in 0..=max_frames {
        let x_min = zoom_x - rad_x;
        let x_max = zoom_x + rad_x;
        let y_min = zoom_y - rad_y;
        let y_max = zoom_y + rad_y;
        let power = (2f64 / rad_x).log(2.0);

        let scale = width as f64 / (y_max - y_min);
        let max_iters = ((2.0 * (1.0 - (5.0 * scale).sqrt()).abs().sqrt()).sqrt() * 66.5) as usize;
        println!(
            "Frame {} | Power {} | Iters {} | x scale {} | y scale {}",
            frame,
            power,
            max_iters,
            x_max - x_min,
            y_max - y_min
        );
        let mandelbrot = calculate_mandelbrot(max_iters, x_min, x_max, y_min, y_max, width, height);
        render_mandelbrot(mandelbrot, max_iters, width, height, frame);

        rad_x /= rad_mult;
        rad_y /= rad_mult;
    }
}
