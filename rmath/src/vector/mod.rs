
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

    pub fn normal(&self) -> Vector2D
    {
        Vector2D { x:-self.y, y:self.x }
    }

    pub fn normalized(&self) -> Vector2D
    {
        let mut v = self.clone();
        v.normalize();
        v
    }

    pub fn normalize(&mut self)
    {
        let l = self.len();
        self.x /= l;
        self.y /= l;
    }

    pub fn scale(&mut self, s: f64) 
    {
        self.x *= s;
        self.y *= s;
    }

    pub fn dot(a: &Vector2D, b: &Vector2D) -> f64
    {
        a.x*b.x + a.y*b.y
    }

    pub fn det(a: &Vector2D, b: &Vector2D) -> f64
    {
        a.x*b.y - a.y*b.x
    }

    pub fn sub(a: &Vector2D, b: &Vector2D) -> Vector2D
    {
        Vector2D{ x: a.x - b.x, y: a.y - b.y }
    }
}

/*
impl std::ops::Add for Vector2D
{
    type Output = Vector2D;

    fn add(mut self, _rhs: Vector2D) -> Vector2D {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self
    }

} */

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





