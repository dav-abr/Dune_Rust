use bevy::prelude::*;
use crate::settings::*;
use crate::components::*;

#[derive(Bundle)]
struct CreatureBundle {
    movable: Movable,
    sprite: SpriteBundle,
    position: Position,
    dimensions: Dimensions
}

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_creatures)
            .add_systems(Update, (move_creature, bound_creatures, reposition_creatures).chain());
    }
}

fn setup_creatures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        CreatureBundle {
            movable: Movable {},
            dimensions: Dimensions {
                width: 1,
                height: 1
            },
            position: Position {
                i: 0,
                j: 0,
                x: 0.0,
                y: 0.0,
            },
            sprite: SpriteBundle {
                texture: asset_server.load("moto_straight_up.png").into(),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0 * CELL_SIZE,
                        y: 0.0 * CELL_SIZE,
                        z: 2.0,
                    },
                    scale: Vec3::splat(CELL_SIZE / 96.0),
                    ..default()
                },
                ..default()
            },
        }
    );
}

fn move_creature(
    mut creature_query: Query<&mut Position, With<Movable>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::D) {
        let mut creature_position = creature_query.get_single_mut().unwrap();

        creature_position.j += 1;
    }
}

fn bound_creatures(mut creature_query: Query<&mut Position, Changed<Position>>) {
    let max_i: i8 = MAP_WIDTH - 1;
    let min_i: i8 = 0;
    let max_j: i8 = MAP_HEIGHT - 1;
    let min_j: i8 = 0;


    for mut position in creature_query.iter_mut() {

        if position.i > max_i {
            position.i = max_i;
        }

        if position.i < min_i {
            position.i = min_i;
        }

        if position.j > max_j {
            position.j = max_j;
        }

        if position.j < min_j {
            position.j = min_j;
        }
    }
}

fn reposition_creatures(
    mut creature_query: Query<(&mut Transform, &Position), Changed<Position>>,
) {
    for (mut transform, position) in creature_query.iter_mut() {
        transform.translation.x = (position.j as f32) * CELL_SIZE;
        transform.translation.y = (position.i as f32) * CELL_SIZE;
    }
}
