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
        app.add_systems(Startup, setup_map)
            .add_systems(Update, update_map);
    }
}

fn setup_map(mut commands: Commands) {
    for i in 0..MAP_HEIGHT {
        for j in 0..MAP_WIDTH {
            commands.spawn(Cell {
                i,
                j,
            });
        }
    }
}

fn update_map(cells: Query<&Cell, With<Cell>>, mut gizmos: Gizmos) {
    for cell in cells.iter() {
        gizmos.rect_2d(
            Vec2 {
                x: cell.i as f32 * CELL_SIZE,
                y: cell.j as f32 * CELL_SIZE,
            },
            0.0_f32,
            Vec2 {
                x: CELL_SIZE,
                y: CELL_SIZE,
            },
            Color::DARK_GREEN
        );
    }
}