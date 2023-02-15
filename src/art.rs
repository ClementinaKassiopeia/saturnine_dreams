use delaunator::{Point, triangulate};

use sprites::*;

fn delaunay(mut cmds: Commands) {
    let vertices = 10;
    let mut points = Vec::with_capacity(vertices);
    let mut rng = thread_rng();
    for _ in 0..vertices {
        let (x, y) = (0.05 + randrange(&mut rng, 0., 1.77, 0.1),
                      0.05 + randrange(&mut rng, 0., 1.00, 0.1));
        points.push(Point {x, y});
        cmds.spawn_bundle(create_point(x as f32, y as f32));
    }

    let result = triangulate(&points);
    for tri in result.triangles.chunks(3) {
        let (a, b, c) = match tri {
            &[a, b, c] => (
                point_to_vec2(&points[a]),
                point_to_vec2(&points[b]),
                point_to_vec2(&points[c])),
            _ => panic!("There should be three elements in a Delaunay slice, \
                        one for each side of the triangle.")
        };
        cmds.spawn_bundle(line_between(a, b));
        cmds.spawn_bundle(line_between(b, c));
        cmds.spawn_bundle(line_between(c, a));
    }
}

fn randrange(rng: &mut ThreadRng, start: f64, stop: f64, step: f64) -> f64 {
    let steps = (stop-start)/step;
    start + (rng.gen::<f64>() * steps).floor() * step
}

fn point_to_vec2(p: &Point) -> Vec2 {
    Vec2 {x: p.x, y: p.y}
}