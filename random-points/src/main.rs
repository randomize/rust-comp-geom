// simple example that just draws random points in range 0..1

extern crate piston_window;
extern crate rand;
extern crate rmath;

use rmath::alg::gen_random_points;
use piston_window::*;



fn main() {

    let black = [0.0, 0.0, 0.0, 1.0];
    let height = 500;
    let width = 500;
    let dot_size = 3.0;
    let count = 500;

    let mut rng = rand::thread_rng();

    let ar = gen_random_points(&mut rng, count);


    let opengl = OpenGL::V3_3;
    let mut window: PistonWindow = WindowSettings::new("Random dots", [width, height])
        .exit_on_esc(true)
        .opengl(opengl)
        .samples(4)
        .vsync(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            for p in &ar {
                let e = [p.x * width as f64, p.y * width as f64, dot_size, dot_size];
                ellipse(black, e, c.transform, g);
            }
        });
    }


}
