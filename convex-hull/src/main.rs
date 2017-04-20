
extern crate piston_window;
extern crate rand;
extern crate rmath;

use rmath::vector::Vector2D;
use rmath::alg::gen_random_points;
use rmath::alg::convex_hull;
use rand::Rng;
use piston_window::*;



fn main() {

    let black = [0.0, 0.0, 0.0, 1.0];
    let red = [1.0, 0.0, 0.0, 1.0];
    let mut rng = rand::thread_rng();

    let ar = gen_random_points(&mut rng, 100);
    let hull = convex_hull(&ar);


    let height = 500;
    let width = 500;
    let dot_size = 3.0;

    let mut window: PistonWindow =
        WindowSettings::new("Random dots", [width, height])
        .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            for p in hull.iter() {
                ellipse(red, [p.x * width as f64, p.y * width as f64, dot_size*2.0, dot_size*2.0],
                        c.transform, g);
            }
            for p in ar.iter() {
                ellipse(black, [p.x * width as f64, p.y * width as f64, dot_size, dot_size],
                        c.transform, g);
            }
        });
    }


}
