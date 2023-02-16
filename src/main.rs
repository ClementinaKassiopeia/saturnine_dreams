use bevy::prelude::*;
use bevy::render::camera::{WindowOrigin, ScalingMode};

// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod triangles;
mod math;
mod sprites;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(WorldInspectorPlugin)
        .add_startup_system(camera_setup)
        .add_startup_system(display_points)
        // .add_system(trigger_display_points)
        // .add_startup_system(text_onscreen)
        .run();
}

fn _text_onscreen(mut cmds: Commands, serv: Res<AssetServer>) {
    cmds.spawn(Text2dBundle {
        text: Text::from_section("Text, onscreen!", TextStyle {
            font: serv.load("FreeMono.ttf"),
            font_size: 50.,
            color: Color::PURPLE
        }),
        transform: Transform::from_xyz(0.7, 0.5, 0.5).with_scale(Vec3::new(0.001, 0.001, 1.)),
        ..Text2dBundle::default()
    });
}

fn display_points(mut cmds: Commands, serv: Res<AssetServer>) {
    let mesh = triangles::generate_mesh();
    for (p1, p2) in mesh {
        cmds.spawn(sprites::line_between(&p1, &p2));

        // println!("[{:.2}, {:.2}], [{:.2}, {:.2}]", p1.x, p1.y, p2.x, p2.y);
        cmds.spawn(Text2dBundle {
            text: Text::from_section(format!("[{:.2}, {:.2}]", p1.x, p1.y), TextStyle {
                font: serv.load("FreeMono.ttf"),
                font_size: 25.,
                color: Color::BLACK
            }),
            transform: Transform::from_xyz(p1.x, p1.y, 0.5)
                        .with_scale(Vec3::new(0.001, 0.001, 1.)),
            ..Text2dBundle::default()
        });
    }
}

// fn trigger_display_points(
//     mut cmds: Commands, input: Res<Input<KeyCode>>,
//     sprites: Query<Entity, With<Sprite>>) {
//     if input.just_pressed(KeyCode::Return) {
//         for e in sprites.iter() {
//             cmds.entity(e).despawn();
//         }
//         display_points(cmds);
//     }
// }

fn camera_setup(mut cmds: Commands) {
    let camera_bundle = Camera2dBundle {
        projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            scaling_mode: ScalingMode::FixedVertical(1.),
            ..OrthographicProjection::default()
        },
        ..Camera2dBundle::default()
    };
    cmds.spawn(camera_bundle);
}

/*
TRIANGLE RECURSION

start with central triangle
have a set of generating edges, containing the edges of the starting triangle
have a set of all edges, containing the same
have an empty set of new edges
have an empty set of no longer generating edges

for each edge e in the set of generating edges:
    if e is in the set of no longer generating edges, do nothing
    otherwise for each of the other edges e' which share a vertex with e,
    check if any are within a certain angle of e
    if yes:
        form a triangle between e and e' by placing an edge between the not-shared vertices
        add e' to the set of no longer generating edges
        add the new edge to the set of new edges
    if no:
        generate a line l perpendicular to e, with random length
        and originating from a random point along e
        form a triangle by adding an edge from the two vertices to the end of l,
        and add them to the set of new edges
add the new edges to the set of all edges
replace the set of generating edges with the set of new edges
empty the set of new edges
empty the set of no longer generating edges
repeat for x iterations

data structures:
point: x, y
edge: point, point
*/