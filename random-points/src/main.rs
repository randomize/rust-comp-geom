
extern crate piston_window;
extern crate rand;
extern crate rmath;

use rmath::vector::Vector2D;
use rmath::vector::gen_random_points;
use rand::Rng;

fn main() {

    let mut rng = rand::thread_rng();

    let a: Vector2D = rng.gen();
    let b: Vector2D = rng.gen();

    println!("a: {}, b: {}", a, b);

    let ar = gen_random_points(&mut rng, 10);

    for (i, p) in ar.iter().enumerate() {
        println!("{}:{}", i, p);
    }

}
