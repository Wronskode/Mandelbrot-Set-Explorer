extern crate sdl2;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use std::thread;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let nb_iterations = 1000; // Adjust here the number of iterations (precision)
    let mut x_min = -2.0;
    let mut x_max = 0.5;
    let mut y_min = -1.12;
    let mut y_max = 1.12;
    let rapport = (x_max - x_min) / (y_max - y_min);
    let height = 1000.0;
    let width = height * rapport;
    let window = video_subsystem
        .window("Mandelbrot Set", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 2.0;
    let mut modified = true;
    let video = true;
    let mut x_target = get_position(-0.5541669757014586, x_min, x_max, 0.0, width);
    let mut y_target = get_position(0.6312605869248036, y_min, y_max, 0.0, height);
    // println!("{}", compute_area(x_min, x_max, y_min, y_max));
    let mut s = 1.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    modified = true;
                    canvas.clear();
                    let result = compute_zoom(
                        width, height, x as f64, y as f64, x_min, x_max, y_min, y_max, i,
                    );
                    x_min = result.0;
                    x_max = result.1;
                    y_min = result.2;
                    y_max = result.3;
                    x_target = result.4;
                    y_target = result.5;
                    i += 1.0;
                }
                _ => {}
            }
        }
        if video {
            canvas.clear();
            s += 0.0001;
            let result = compute_zoom(
                width, height, x_target, y_target, x_min, x_max, y_min, y_max, s,
            );
            x_min = result.0;
            x_max = result.1;
            y_min = result.2;
            y_max = result.3;
            modified = true;
        }
        if modified {
            draw_mandelbrot_set(
                &mut canvas,
                nb_iterations,
                x_min,
                x_max,
                y_min,
                y_max,
                (width, height),
            );
        }
        modified = false;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn get_position(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    (x - a) * (d - c) / (b - a) + c
}

fn compute_zoom(
    width: f64,
    height: f64,
    x: f64,
    y: f64,
    mut x_min: f64,
    mut x_max: f64,
    mut y_min: f64,
    mut y_max: f64,
    zoom_ratio: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let new_width = width / zoom_ratio;
    let new_height = height / zoom_ratio;
    let r1 = x / width;
    let r2 = y / height;
    let xmin = x_min;
    let xmax = x_max;
    let ymin = y_min;
    let ymax = y_max;
    let x_target = x - r1 * new_width;
    let y_target = y - r2 * new_height;
    x_min = get_position(x_target, 0.0, width, x_min, x_max);
    y_min = get_position(y_target, 0.0, height, y_min, y_max);
    x_max = get_position(x - r1 * new_width + new_width, 0.0, width, xmin, xmax);
    y_max = get_position(y - r2 * new_height + new_height, 0.0, height, ymin, ymax);
    (x_min, x_max, y_min, y_max, x_target, y_target)
}

fn draw_mandelbrot_set(
    canvas: &mut Canvas<sdl2::video::Window>,
    iterations: u32,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    (w, h): (f64, f64),
) {
    let t1 = thread::spawn(move || {
        let mut local_vec = vec![];
        for j in 0..=(w as i32) {
            for k in 0..(h as i32) / 4 {
                let x = get_position(j as f64, 0.0, w, x1, x2);
                let y = get_position(k as f64, 0.0, h, y1, y2);
                let c = est_bornee(x, y, iterations);
                if !c.0 {
                    local_vec.push((j, k, 255 as u8, (c.1 * 15) as u8, 0_u8));
                }
            }
        }
        local_vec
    });
    let t2 = thread::spawn(move || {
        let mut local_vec = vec![];
        for j in 0..=(w as i32) {
            for k in (h as i32) / 4..(h as i32) / 2 {
                let x = get_position(j as f64, 0.0, w, x1, x2);
                let y = get_position(k as f64, 0.0, h, y1, y2);
                let c = est_bornee(x, y, iterations);
                if !c.0 {
                    local_vec.push((j, k, 255 as u8, (c.1 * 15) as u8, 0_u8));
                }
            }
        }
        local_vec
    });
    let t3 = thread::spawn(move || {
        let mut local_vec = vec![];
        for j in 0..=(w as i32) {
            for k in (h as i32) / 2..3 * (h as i32) / 4 {
                let x = get_position(j as f64, 0.0, w, x1, x2);
                let y = get_position(k as f64, 0.0, h, y1, y2);
                let c = est_bornee(x, y, iterations);
                if !c.0 {
                    local_vec.push((j, k, 255 as u8, (c.1 * 15) as u8, 0_u8));
                }
            }
        }
        local_vec
    });
    let t4 = thread::spawn(move || {
        let mut local_vec = vec![];
        for j in 0..=(w as i32) {
            for k in 3 * (h as i32) / 4..=(h as i32) {
                let x = get_position(j as f64, 0.0, w, x1, x2);
                let y = get_position(k as f64, 0.0, h, y1, y2);
                let c = est_bornee(x, y, iterations);
                if !c.0 {
                    local_vec.push((j, k, 255 as u8, (c.1 * 15) as u8, 0_u8));
                }
            }
        }
        local_vec
    });
    let mut v1 = t1.join().unwrap();
    v1.extend(t2.join().unwrap());
    v1.extend(t3.join().unwrap());
    v1.extend(t4.join().unwrap());
    for element in v1 {
        canvas.set_draw_color(Color::RGB(element.2, element.3, element.4));
        let _ = canvas.draw_point(Point::new(element.0, element.1));
    }
    canvas.present();
}

fn est_bornee(a: f64, b: f64, iterations: u32) -> (bool, u32) {
    let p = (a - 1.0 / 4.0).powi(2) + b.powi(2);
    if a < p.sqrt() - 2.0 * p + 1.0 / 4.0 || (a + 1.0).powi(2) + b.powi(2) < 1.0 / 16.0 {
        return (true, 0);
    }
    let mut x = 0.0;
    let mut y = 0.0;
    for i in 0..iterations {
        let y1 = 2.0 * x * y;
        x = x * x - y * y + a;
        y = y1 + b;
        if (x * x + y * y) > 4.0 {
            return (false, i);
        }
    }
    (true, 0)
}

fn compute_area(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let total_area = (x_max - x_min) * (y_max - y_min);
    let mut i = 0;
    let mut c = 0;
    while i < 1000000 {
        let x = rng.gen_range(x_min..=x_max);
        let y = rng.gen_range(y_min..=y_max);
        if est_bornee(x, y, 100000).0 {
            c += 1;
        }
        i += 1;
    }
    return ((c as f64) / (i as f64)) * total_area;
}
