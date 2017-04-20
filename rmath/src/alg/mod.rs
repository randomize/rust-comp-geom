
extern crate rand;
extern crate std;



use self::rand::Rng;
use std::fmt;
use super::vector::Vector2D;

pub fn gen_random_points(rng: &mut rand::ThreadRng ,n: usize) -> Vec<Vector2D> {

    let mut v = Vec::with_capacity(n);
    for _ in 0..n  {
        v.push(rng.gen::<Vector2D>());
    }
    v
}

pub fn ccw(a: &Vector2D, b: &Vector2D, c: &Vector2D) -> bool
{
    let x = Vector2D::sub(a, b);
    let y = Vector2D::sub(c, b);
    Vector2D::dot(&x, &y) < 0.0
}


pub fn convex_hull(points: &Vec<Vector2D> ) -> Option<Vec<Vector2D>> {

    let l = points.len();

    if l == 0 {
        return None;
    }

    if l <= 3 {
        return Some(points.clone());
    }

    let mut sorted_points = points.clone();
    // sorted_points.sort_by_key(|k| k.x);
    sorted_points.sort_by( |a, b| a.x.partial_cmp(&b.x).unwrap()); // TODO: why so ?

    let mut hull : Vec<Vector2D> = Vec::with_capacity(l);
    hull.push(sorted_points[0].clone());


    for i in 1..l {
        let p = sorted_points[i].clone();

        while hull.len() >= 2 && ccw(&hull[hull.len()-1], &hull[hull.len()-2], &p)
        {
            hull.pop();
        }

        hull.push(p);
    }

    return Some(hull);
}




