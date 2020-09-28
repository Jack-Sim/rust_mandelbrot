mod mapping;
extern crate piston_window;

use piston_window::*;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const MAX_ITER: u32 = 1538;
const LIMIT: f32 = 8.;

fn main() {
    let mut pixels: Vec<[f32; 4]> = Vec::new();

    for x in 0..WIDTH {
        for y in 0..HEIGHT{
            let mut a = mapping::map_num(x as f32, 0., WIDTH as f32, -1.105, -1.090);
            let mut b = mapping::map_num(y as f32, 0., HEIGHT as f32, -0.240, -0.230);

            let ca = a.clone();
            let cb = b.clone();

            let mut n: u32 = 0;

            while n < MAX_ITER {
                let aa = a * a - b * b;
                let bb = 2.0 * a * b;

                a = aa + ca;
                b = bb + cb;

                if (a * a + b * b) > LIMIT {
                    break;
                }
                n += 1;
            }

            let mut bright = mapping::map_num(n as f32, 0., MAX_ITER as f32, 0., 1.);
            bright = bright.sqrt();
            if n == MAX_ITER {
                bright = 0.;
            }
            pixels.push([bright, bright, bright, 1.]);
        }
    }
    let mut window: PistonWindow = 
        WindowSettings::new("Mandelbrot Sketch", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();
    
    
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _device| {
                clear([0., 0., 0., 1.0], g);
            
                for x in 0..WIDTH as usize {
                    for y in 0..HEIGHT as usize { 
                    rectangle(pixels[y + x * WIDTH as usize], 
                        [x as f64, y as f64, x as f64, y as f64], 
                        c.transform, g);
                    }
                }
                
            });
        }
    
}

