use bevy::prelude::*;
mod camera;
mod map;
mod images;

use crate::camera::*;
use crate::images::ImagesPlugin;
use crate::map::*;


#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ImagesPlugin, CursorPlugin, MapPlugin))
        .run();
}
