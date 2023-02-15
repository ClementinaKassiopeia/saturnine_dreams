use bevy::prelude::*;
use bevy::math::f32::Quat;

use std::f32::consts::FRAC_PI_2;

use crate::sprites::create_sprite_bundle;

// pub fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(spawn)
//         .add_system()
//         .run()
// }

pub fn rods(mut cmds: Commands) {
    cmds.spawn_bundle(create_sprite_bundle(0.4, 0.01, 0.5, 0.5, FRAC_PI_2));
    cmds.spawn_bundle(create_sprite_bundle(0.4, 0.01, 1.0, 0.5, FRAC_PI_2));
}

pub fn rotate_transforms(mut trans: Query<&mut Transform, Without<Camera>>) {
    for mut t in trans.iter_mut() {
        // t.translation.x += 0.05;
        t.rotate_z(0.1);
    }
}