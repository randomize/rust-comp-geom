
extern crate rand;
extern crate std;



use self::rand::Rng;
use super::vector::Vector2D;
use std::f64;

pub fn gen_random_points(rng: &mut rand::ThreadRng ,n: usize) -> Vec<Vector2D> {

    let mut v = Vec::with_capacity(n);
    for _ in 0..n  {
        v.push(rng.gen::<Vector2D>());
    }
    v
}

pub fn gen_random_points_unit_circle(rng: &mut rand::ThreadRng ,n: usize) -> Vec<Vector2D> {

    let mut v = Vec::with_capacity(n);
    for _ in 0..n  {
        let r = rng.gen_range(0.0, 1.0) * 0.5;
        let a = rng.gen_range(0.0, f64::consts::PI * 2.0);
        let t = Vector2D {
            x: a.sin() * r + 0.5,
            y: a.cos() * r + 0.5
        };
        v.push(t);
    }
    v
}

pub fn ccw(a: &Vector2D, b: &Vector2D, c: &Vector2D) -> bool
{
    let x = Vector2D::sub(a, b);
    let y = Vector2D::sub(c, b);
    Vector2D::det(&x, &y) > 0.0
}


fn convex_half_hull(sorted_points: &Vec<Vector2D>, up: bool ) -> Vec<Vector2D> {

    let l = sorted_points.len();
    let mut hull : Vec<Vector2D> = Vec::with_capacity(l);
    hull.push(sorted_points[0].clone());

    for i in 1..l {
        let p = sorted_points[i].clone();

        while hull.len() >= 2 && ccw(&hull[hull.len()-2], &hull[hull.len()-1], &p) == up
        {
            hull.pop();
        }

        hull.push(p);
    }

    hull
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
    sorted_points.sort_by( |a, b| a.x.partial_cmp(&b.x).unwrap()); // TODO: sort by y too

    let mut upper_hull = convex_half_hull(&sorted_points, true);
    let lower_hull = convex_half_hull(&sorted_points, false);

    upper_hull.extend(lower_hull);

    return Some(upper_hull);
}




