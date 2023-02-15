use std::rc::Rc;
use std::cell::RefCell;
use std::f32::consts::FRAC_PI_2;

use rand::prelude::*;
use bevy::math::Vec2;

use crate::math::angle;

struct Point {
    coords: Vec2,
    neighbours: Vec<PointRef>
}

type PointRef = Rc<RefCell<Point>>;

fn wrap(x: f32, y: f32) -> PointRef {
    Rc::new(RefCell::new(
        Point { coords: Vec2{x, y}, neighbours: Vec::new() }
    ))
}

// creates a line between two Points, and adds that line to the vector
fn push_joined_edge(v: &mut Vec<[PointRef; 2]>, p1: &PointRef, p2: &PointRef) {
    p1.borrow_mut().neighbours.push(Rc::clone(p2));
    p2.borrow_mut().neighbours.push(Rc::clone(p1));
    v.push([Rc::clone(p1), Rc::clone(p2)]);
}

pub fn generate_mesh() -> impl Iterator<Item=[Vec2; 2]> {
    let mut edges:      Vec<[PointRef; 2]> = Vec::new();
    let mut free_edges: Vec<[PointRef; 2]> = Vec::with_capacity(3);
    // let (p1, p2, p3) = (&wrap(0.6, 0.4), &wrap(0.8, 0.45), &wrap(0.7, 0.5));
    let (p1, p2, p3) = (&wrap(0.7, 0.3), &wrap(0.8, 0.45), &wrap(0.6, 0.4));

    push_joined_edge(&mut free_edges, p1, p2);
    push_joined_edge(&mut free_edges, p2, p3);
    push_joined_edge(&mut free_edges, p3, p1);
    edges.extend_from_slice(free_edges.as_slice());

    let rng = &mut thread_rng();
    for _ in 0..2 {
        let mut new_edges: Vec<[PointRef; 2]> = Vec::with_capacity(3);
        for [prf1, prf2] in free_edges.iter() {
            add_triangle(rng, &mut new_edges, prf1, prf2);
        }
        edges.extend_from_slice(new_edges.as_slice());
        free_edges = new_edges;
    }

    edges.into_iter().map(|[prf1, prf2]| {
        [prf1.borrow().coords,
         prf2.borrow().coords]
    })
}

fn add_triangle(
  rng: &mut ThreadRng,
  new_edges: &mut Vec<[PointRef; 2]>,
  prf1: &PointRef,
  prf2: &PointRef
) {
    let p1 = prf1.borrow().coords;
    let p2 = prf2.borrow().coords;

    let edge_len = Vec2::distance(p2, p1);
    let new_triangle_height = rng.gen_range(edge_len*0.5 .. edge_len*1.25);
    let perp_angle = angle(&p1, &p2) - FRAC_PI_2;
    let new_point = &wrap(
        rand_between(rng, p1.x, p2.x) + f32::cos(perp_angle) * new_triangle_height,
        rand_between(rng, p1.y, p2.y) + f32::sin(perp_angle) * new_triangle_height
    );
    drop(p1); drop(p2);

    push_joined_edge(new_edges, prf1, new_point);
    push_joined_edge(new_edges, new_point, prf2);
}

fn rand_between(rng: &mut ThreadRng, start: f32, stop: f32) -> f32 {
    let mid = 0.5; //rng.gen::<f32>();
    start + (mid * (stop-start))
}