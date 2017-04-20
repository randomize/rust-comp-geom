
extern crate piston_window;
extern crate rand;
extern crate rmath;

use rmath::vector::Vector2D;
use rmath::vector::gen_random_points;
use rand::Rng;
use piston_window::*;



fn main() {

    let black = [0.0, 0.0, 0.0, 1.0];
    let mut rng = rand::thread_rng();

    let a: Vector2D = rng.gen();
    let b: Vector2D = rng.gen();

    println!("a: {}, b: {}", a, b);

    let ar = gen_random_points(&mut rng, 100);

    /*
    for (i, p) in ar.iter().enumerate() {
        println!("{}:{}", i, p.x);
    }
    */

    let height = 500;
    let width = 500;
    let dot_size = 3.0;

    let mut window: PistonWindow =
        WindowSettings::new("Random dots", [width, height])
        .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
            for p in ar.iter() {
                let l = p.len();
                ellipse(black, [p.x * width as f64, p.y * width as f64, dot_size, dot_size],
                   c.transform, g);
            }
        });
    }


}
