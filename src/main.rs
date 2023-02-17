use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResizeConstraints,
};
use bevy_pixels::prelude::*;
use rand::prelude::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

#[derive(Component, Debug)]
struct Point {
    x: u32,
    y: u32,
    speed_x: f32,
    speed_y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Hello Bevy Pixels".to_string(),
                width: WIDTH as f32,
                height: HEIGHT as f32,
                resize_constraints: WindowResizeConstraints {
                    min_width: WIDTH as f32,
                    min_height: HEIGHT as f32,
                    ..default()
                },
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(PixelsPlugin {
            width: WIDTH,
            height: HEIGHT,
            ..default()
        })
        .add_startup_system(setup)
        .add_system(exit_on_escape)
        .add_system(movement)
        .add_system(draw)
        .run();
}
fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        commands.spawn(Point{
            x: rng.gen_range(0..WIDTH as u32),
            y: rng.gen_range(0..WIDTH as u32),
            speed_x: rng.gen_range(0..100) as f32 / 80.0,
            speed_y: rng.gen_range(0..100) as f32 / 80.0,

        });
    }

}

fn movement(mut query: Query<&mut Point>) {
    for mut point in query.iter_mut() {
        point.x = (point.x as f32 + point.speed_x) as u32;
        point.y = (point.y as f32 + point.speed_y) as u32;
    }
}

fn exit_on_escape(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn draw(mut pixels_resource: ResMut<PixelsResource>, query: Query<&Point>) {
    let frame = pixels_resource.pixels.get_frame_mut().chunks_exact_mut(4);

    let mut points = query.iter().map(|p| (p.x, p.y)).collect::<Vec<(u32, u32)>>();


    for (i, pixel) in frame.enumerate() {
        let x = (i % WIDTH as usize) as u32;
        let y = (i / WIDTH as usize) as u32;

        let mut distances: Vec<u32> = points.iter().map(|p| euclidean_dist(&(p.0 as i32, p.1 as i32), &(x as i32,y as i32)) as u32).collect();
        distances.sort();

        //generate a random color from white to black
        let r: u8 = 0xff;
        let g: u8 = 0xff;
        let b: u8 = 0xff;
        let a: u8 = distances[0] as u8;
        let color = [r, g, b, map_range((0.0, 50.0), (0.0, 255.0), a as f64) as u8];

        //println!("{} {} {} {}", r, g, b, a);

        pixel.copy_from_slice(&color);
    }
}
fn euclidean_dist(p: &(i32, i32), q: &(i32, i32)) -> f64 {
    (((p.0 - q.0) as f64).powi(2) + ((p.1 - q.1) as f64).powi(2)).sqrt()
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}