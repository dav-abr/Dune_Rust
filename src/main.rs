use bevy::prelude::*;
mod camera;
use crate::camera::*;


#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorPlugin))
        .run();
}