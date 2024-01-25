use bevy::animation::Keyframes::Translation;
use bevy::prelude::*;

const MAP_WIDTH: i8 = 64;
const MAP_HEIGHT: i8 = 64;
const CELL_SIZE: f32 = 50.0_f32;

#[derive(Component)]
struct Map {}

#[derive(Component)]
struct Cell {
    i: i8,
    j: i8,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..MAP_HEIGHT {
        for j in 0..MAP_WIDTH {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("concrete.png").into(),
                    transform: Transform {
                        translation: Vec3 {
                            x: i as f32 * CELL_SIZE,
                            y: j as f32 * CELL_SIZE,
                            z: 1.0,
                        },
                        scale: Vec3::splat(CELL_SIZE / 32.0),
                        ..default()
                    },
                    ..default()
                },
                Cell {
                    i,
                    j,
                })
            );
        }
    }
}
