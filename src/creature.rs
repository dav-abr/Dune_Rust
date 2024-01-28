use bevy::prelude::*;
use crate::settings::*;
use crate::components::*;
use std::collections::*;
use std::f32::consts::*;
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
            .add_systems(Update, (
                move_creature,
                bound_creatures,
                reposition_creatures,
                (rotate_to_angle, rotate_creature).chain()
            ).chain());
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
            movable: Movable {
                angle: 0.0,
                to_angle: 0.0,
            },
            dimensions: Dimensions {
                width: 1,
                height: 1,
            },
            position: Position {
                i: 0,
                j: 0,
            },
            sprite: SpriteBundle {
                texture: asset_server.load("moto_straight_right.png").into(),
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
            movable: Movable {
                angle: 0.0,
                to_angle: 0.0,
            },
            dimensions: Dimensions {
                width: 1,
                height: 1,
            },
            position: Position {
                i: 1,
                j: 1,
            },
            sprite: SpriteBundle {
                texture: asset_server.load("moto_straight_right.png").into(),
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
    mut creature_query: Query<(&mut Position, &mut Transform, &mut PathToGo, &mut Movable), With<PathToGo>>,
) {
    for (
        mut creature_position,
        mut creature_transform,
        mut creature_path,
        mut creature_movable
    ) in creature_query.iter_mut() {
        if normalize_angle(creature_movable.angle) != creature_movable.to_angle {
            continue;
        }

        if let Some(next_cell) = creature_path.path.get(0) {
            let x = next_cell.j as f32 * CELL_SIZE;
            let y = next_cell.i as f32 * CELL_SIZE;

            let angle = normalize_angle(creature_movable.angle);
            let angle_to_next = ((next_cell.i - creature_position.i) as f32)
                .atan2((next_cell.j - creature_position.j) as f32)
                * (180.0 / PI);

            if angle != angle_to_next {
                creature_movable.to_angle = angle_to_next;
                continue;
            }

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

fn angle_change_factor(from: f32, to: f32) -> f32 {
    let mut left = (360.0 - from) + to;
    let mut right = from - to;

    if from < to {
        if to > 0.0 {
            left = to - from;
            right = (360.0 - to) + from;
        } else {
            left = (360.0 - to) + from;
            right = to - from;
        }
    }

    if left <= right {
        1.0
    } else {
        -1.0
    }
}

fn rotate_to_angle(
    mut creature_query: Query<&mut Movable, With<Movable>>,
) {
    for mut creature_movable in creature_query.iter_mut() {
        let angle = normalize_angle(creature_movable.angle);
        let angle_to = creature_movable.to_angle;

        if angle != angle_to {
            creature_movable.angle += angle_change_factor(angle, angle_to);
        }
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

fn normalize_angle(angle: f32) -> f32 {
    if angle >= 0.0 {
        return angle % 360.0;
    }

    return angle + (angle / 360.0) * 360.0;
}

fn rotate_creature(
    mut creature_query: Query<(&mut Handle<Image>, &Movable), Changed<Movable>>,
    asset_server: Res<AssetServer>,
) {
    for (mut creature_image, creature_movable) in creature_query.iter_mut() {
        let angle = normalize_angle(creature_movable.angle);

        if angle >= 348.75 && angle <= 360.0 || angle >= 0.0 && angle <= 11.25 {
            *creature_image = asset_server.load("moto_straight_right.png").into();
        }

        if angle >= 11.25 && angle <= 33.75 {
            *creature_image = asset_server.load("moto_angle_horizontal_right_down.png").into();
        }

        if angle >= 33.75 && angle <= 56.25 {
            *creature_image = asset_server.load("moto_diagonal_up_right.png").into();
        }

        if angle >= 56.25 && angle <= 78.75 {
            *creature_image = asset_server.load("moto_angle_vertical_up_right.png").into();
        }

        if angle >= 78.75 && angle <= 101.25  {
            *creature_image = asset_server.load("moto_straight_up.png").into();
        }

        if angle >= 101.25 && angle <= 123.75 {
            *creature_image = asset_server.load("moto_angle_vertical_up_left.png").into();
        }

        if angle >= 123.75 && angle <= 146.25 {
            *creature_image = asset_server.load("moto_diagonal_up_left.png").into();
        }

        if angle >= 146.25 && angle <= 168.75 {
            *creature_image = asset_server.load("moto_angle_horizontal_left_down.png").into();
        }

        if angle >= 168.75 && angle <= 191.25 {
            *creature_image = asset_server.load("moto_straight_left.png").into();
        }

        if angle >= 191.25 && angle <= 213.75 {
            *creature_image = asset_server.load("moto_angle_horizontal_left_up.png").into();
        }

        if angle >= 213.75 && angle <= 236.25 {
            *creature_image = asset_server.load("moto_diagonal_down_left.png").into();
        }

        if angle >= 236.25 && angle <= 258.75 {
            *creature_image = asset_server.load("moto_angle_vertical_down_left.png").into();
        }

        if angle >= 258.75 && angle <= 281.25 {
            *creature_image = asset_server.load("moto_straight_down.png").into();
        }

        if angle >= 281.25 && angle <= 303.75 {
            *creature_image = asset_server.load("moto_angle_vertical_down_right.png").into();
        }

        if angle >= 303.75 && angle <= 326.25 {
            *creature_image = asset_server.load("moto_diagonal_down_right.png").into();
        }

        if angle >= 326.25 && angle <= 348.75 {
            *creature_image = asset_server.load("moto_angle_horizontal_right_up.png").into();
        }
    }
}
