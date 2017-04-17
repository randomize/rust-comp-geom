
extern crate rand;
extern crate std;

use self::rand::Rng;
use std::fmt;

pub struct Vector2D
{
    x: f64,
    y: f64
}

impl Vector2D
{

    fn len(self) -> f64 
    {
        f64::sqrt(self.x*self.x + self.y*self.y)
    }

}

impl rand::Rand for Vector2D
{
    fn rand<R: Rng>(rng: &mut R) -> Vector2D
    {
        Vector2D{ x: rng.gen::<f64>(), y: rng.gen::<f64>() }
    }
}

impl std::fmt::Display for Vector2D
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn gen_random_points(rng: &mut rand::ThreadRng ,n: usize) -> Vec<Vector2D> {

    let mut v = Vec::with_capacity(n);
    for _ in 0..n  {
        v.push(rng.gen::<Vector2D>());
    }
    v
}
