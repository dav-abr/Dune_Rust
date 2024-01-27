use bevy::{window::PrimaryWindow, prelude::*};
use crate::components::*;
use crate::settings::CELL_SIZE;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_controls);
    }
}

fn update_controls(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    creatures_query: Query<&Position, With<Movable>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, transform) = camera_query.single();

        if let Some(cursor_position) = q_windows.single().cursor_position().and_then(|cursor| camera.viewport_to_world_2d(transform, cursor)) {
            let (i, j) = (
                ((cursor_position.x + CELL_SIZE / 2.0) / CELL_SIZE) as i8,
                ((cursor_position.y + CELL_SIZE / 2.0) / CELL_SIZE) as i8
            );

            let creatures = creatures_query.iter().filter(|position| position.i == i && position.j == j).collect::<Vec<_>>();
            let creature_position = creatures.get(0);

            match creature_position {
                Some(position) => {
                    println!("{:?}", position);
                },

                None => {
                    println!("None");
                }
            }
        }
    }
}
