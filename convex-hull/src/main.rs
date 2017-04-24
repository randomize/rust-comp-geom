
extern crate piston_window;
extern crate rand;
extern crate rmath;

// use rmath::vector::Vector2D;
use rmath::alg::gen_random_points_unit_circle;
use rmath::alg::convex_hull;
use piston_window::*;



fn main() {

    let black = [0.0, 0.0, 0.0, 1.0];
    let red = [1.0, 0.0, 0.0, 1.0];

    let height = 500;
    let width = 500;
    let dot_size = 3.0;
    let count = 32;

    let mut rng = rand::thread_rng();
    let ar = gen_random_points_unit_circle(&mut rng, count);
    let hull = convex_hull(&ar).unwrap();


    let mut window: PistonWindow = WindowSettings::new("Random dots", [width, height])
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let line = Line::new(red, 0.5);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            for p in &ar {
                let e = [p.x * width as f64 - dot_size/2., p.y * width as f64 - dot_size/2., dot_size, dot_size];
                ellipse(black, e, c.transform, g);
            }
            for l in hull.windows(2) {
                let p1 = &l[0];
                let p2 = &l[1];
                let p = [p1.x * width as f64,
                         p1.y * width as f64,
                         p2.x * width as f64,
                         p2.y * width as f64];
                line.draw(p, &c.draw_state, c.transform, g);
            }
        });
    }


}
