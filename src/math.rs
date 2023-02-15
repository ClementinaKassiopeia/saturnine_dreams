use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::math::Vec2;

pub fn negate_if(cond: bool, x: f32) -> f32 { if cond {-x} else {x} }

// calculates the angle of the vector going from p1 to p2
pub fn angle(p1: &Vec2, p2: &Vec2) -> f32 {
    let x_dist = p2.x - p1.x;
    let y_dist = p2.y - p1.y;
    if y_dist == 0.0 && x_dist < 0.0 { PI } //so that a 180 degree angle won't return 0
    else if x_dist == 0.0            { negate_if(y_dist <= 0.0, FRAC_PI_2)} //to prevent division by 0 for 90 and 270 degree angles
    else                             { f32::atan2(y_dist, x_dist) }
}

pub fn _in_0_to_pi(mut angle: f32) -> bool {
    angle %= TAU;
    if angle < 0.0 { angle += TAU; }
    angle < PI
}