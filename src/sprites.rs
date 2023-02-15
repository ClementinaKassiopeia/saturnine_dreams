use bevy::prelude::*;

use std::f32::consts::FRAC_PI_2;

pub fn create_sprite_bundle(width: f32, height: f32, x: f32, y: f32, rot: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(width, height)),
            ..Sprite::default()
        },
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            rotation: Quat::from_rotation_z(rot),
            scale: Vec3::ONE
        },
        ..SpriteBundle::default()
    }
}

pub fn _create_point(x: f32, y: f32) -> SpriteBundle {
    create_sprite_bundle(0.01, 0.01, x, y, 0.0)
}

pub fn line_between(p1: &Vec2, p2: &Vec2) -> SpriteBundle {
    let x_dist = p2.x - p1.x;
    let y_dist = p2.y - p1.y;
    let dist = f32::sqrt(x_dist.powi(2) + y_dist.powi(2));
    
    create_sprite_bundle(
        dist as f32, 0.0075,
        p1.x + x_dist/2.,
        p1.y + y_dist/2.,
        if x_dist == 0.0 { FRAC_PI_2 } else { f32::atan(y_dist/x_dist) }
    )
}