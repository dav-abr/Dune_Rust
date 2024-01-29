use bevy::prelude::*;
use crate::settings::*;

#[derive(Component)]
struct Map {}

#[derive(Component)]
pub struct Cell {
    pub i: i8,
    pub j: i8,
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
                            x: j as f32 * CELL_SIZE,
                            y: i as f32 * CELL_SIZE,
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
