
extern crate rand;
extern crate std;

use self::rand::Rng;
use std::fmt;

#[derive(Clone)]
pub struct Vector2D
{
    pub x: f64,
    pub y: f64
}

impl Vector2D
{

    pub fn len(&self) -> f64 
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


pub fn convex_hull(points: &Vec<Vector2D> ) -> Option<Vec<Vector2D>> {

    if points.len() == 0 {
        return None;
    }

    if points.len() <= 3 {
        return Some(points.clone());
    }

    return None;
}




