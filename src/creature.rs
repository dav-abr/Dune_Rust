use std::array;
use bevy::prelude::*;
use crate::settings::*;
use crate::components::*;
use std::collections::*;
use pathfinding::prelude::*;

#[derive(Bundle)]
struct CreatureBundle {
    movable: Movable,
    sprite: SpriteBundle,
    position: Position,
    dimensions: Dimensions,
}

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_creatures)
            .add_systems(Update, (move_creature, bound_creatures, reposition_creatures).chain());
    }
}

fn setup_creatures(mut commands: Commands, asset_server: Res<AssetServer>, creatures_query: Query<&Position, With<Movable>>) {
    let mut positions: Vec<Position> = vec![];

    for position in creatures_query.iter() {
        positions.push(position.clone());
    }

    let GOAL: Position = Position::new(4, 6);
    let path = astar(&Position::new(0, 0), |p| p.successors(&positions), |p| p.distance(&GOAL) / 3, |p| *p == GOAL);

    commands.spawn(
        CreatureBundle {
            movable: Movable {},
            dimensions: Dimensions {
                width: 1,
                height: 1,
            },
            position: Position {
                i: 0,
                j: 0,
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
    ).insert(PathToGo {
        path: VecDeque::from_iter(
            path.unwrap().0
        )
    });


    commands.spawn(
        CreatureBundle {
            movable: Movable {},
            dimensions: Dimensions {
                width: 1,
                height: 1,
            },
            position: Position {
                i: 1,
                j: 1,
            },
            sprite: SpriteBundle {
                texture: asset_server.load("moto_straight_up.png").into(),
                transform: Transform {
                    translation: Vec3 {
                        x: 1.0 * CELL_SIZE,
                        y: 1.0 * CELL_SIZE,
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
    mut creature_query: Query<(&mut Position, &mut Transform, &mut PathToGo), With<PathToGo>>,
) {
    for (mut creature_position, mut creature_transform, mut creature_path) in creature_query.iter_mut() {
        if let Some(next_cell) = creature_path.path.get(0) {
            let x = next_cell.j as f32 * CELL_SIZE;
            let y = next_cell.i as f32 * CELL_SIZE;

            if creature_transform.translation.x == x && creature_transform.translation.y == y {
                creature_position.i = next_cell.i;
                creature_position.j = next_cell.j;

                creature_path.path.pop_front();
            } else {
                creature_transform.translation.x += 1.0 * (next_cell.j - creature_position.j) as f32;
                creature_transform.translation.y += 1.0 * (next_cell.i - creature_position.i) as f32;
            }
        }
    }
}

fn animate_to_cell() {}

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
