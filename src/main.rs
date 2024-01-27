use bevy::prelude::*;
mod camera;
mod map;
mod images;
mod creature;
mod settings;
mod components;
mod controls;
mod hud;
mod state;

use crate::camera::*;
use crate::controls::ControlsPlugin;
use crate::creature::CreaturesPlugin;
use crate::hud::HUDPlugin;
use crate::images::ImagesPlugin;
use crate::map::*;


#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ImagesPlugin, CursorPlugin, HUDPlugin, ControlsPlugin, MapPlugin, CreaturesPlugin))
        .run();
}
