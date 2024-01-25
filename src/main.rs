use bevy::prelude::*;
mod camera;
mod map;

use crate::camera::*;
use crate::map::*;


#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorPlugin, MapPlugin))
        .run();
}