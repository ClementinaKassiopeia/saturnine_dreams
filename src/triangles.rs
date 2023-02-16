use std::{
    f32::consts::FRAC_PI_2,
    collections::{HashMap, HashSet},
    hash::Hash
};

use rand::prelude::*;
use bevy::math::Vec2;

use crate::math::angle;

type BranchMap = HashMap<[u32; 2], HashSet<[u32; 2]>>;

// creates a line between two points, and adds that line to the vector
fn push_joined_edge(
  v: &mut Vec<(Vec2, Vec2)>,
  branches: &mut BranchMap,
  p0: Vec2,
  p1: Vec2
) {
    v.push((p0.clone(), p1.clone()));
    let p0: [u32; 2] = [p0.x.to_bits(), p0.y.to_bits()];
    let p1: [u32; 2] = [p1.x.to_bits(), p1.y.to_bits()];
    add_branch(branches, p0, p1);
    add_branch(branches, p1, p0);
}

fn add_branch<K, T>(branches: &mut HashMap<K, HashSet<T>>, key: K, val: T)
where K: Eq + Hash,
      T: Eq + Hash {
    branches.entry(key).or_insert_with(HashSet::new).insert(val);
}

pub fn generate_mesh() -> impl Iterator<Item=(Vec2, Vec2)> {
    let mut edges:      Vec<(Vec2, Vec2)> = Vec::new();
    let mut free_edges: Vec<(Vec2, Vec2)> = Vec::with_capacity(3);
    let mut branches = HashMap::new();
    // let (p0, p1, p2) = (Vec2::new(0.6, 0.4), Vec2::new(0.8, 0.45), Vec2::new(0.7, 0.5));
    let (p0, p1, p2) = (Vec2::new(0.7, 0.3), Vec2::new(0.8, 0.45), Vec2::new(0.6, 0.4));

    push_joined_edge(&mut free_edges, &mut branches, p0, p1);
    push_joined_edge(&mut free_edges, &mut branches, p1, p2);
    push_joined_edge(&mut free_edges, &mut branches, p2, p0);
    edges.extend_from_slice(free_edges.as_slice());

    let rng = &mut thread_rng();
    for gen in 0..2 {
        let mut new_edges: Vec<(Vec2, Vec2)> = Vec::with_capacity(3_usize.pow(gen+1));
        for edge in free_edges.into_iter() {
            add_triangle(rng, &mut new_edges, &mut branches, edge.0, edge.1);
        }
        edges.extend_from_slice(new_edges.as_slice());
        free_edges = new_edges;
    }

    edges.into_iter()
}

fn add_triangle(
  rng: &mut ThreadRng,
  new_edges: &mut Vec<(Vec2, Vec2)>,
  branches: &mut BranchMap,
  p0: Vec2,
  p1: Vec2
) {
    let edge_len = Vec2::distance(p1, p0);
    let new_triangle_height = rng.gen_range(edge_len*0.5 .. edge_len*1.25);
    let perp_angle = angle(p0, p1) - FRAC_PI_2;
    let new_point = Vec2::new(
        rand_between(rng, p0.x, p1.x) + f32::cos(perp_angle) * new_triangle_height,
        rand_between(rng, p0.y, p1.y) + f32::sin(perp_angle) * new_triangle_height
    );

    push_joined_edge(new_edges, branches, p0, new_point);
    push_joined_edge(new_edges, branches, new_point, p1);
}

fn rand_between(rng: &mut ThreadRng, start: f32, stop: f32) -> f32 {
    let mid = 0.5; //rng.gen::<f32>();
    start + (mid * (stop-start))
}