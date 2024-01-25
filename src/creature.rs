use bevy::prelude::*;
use crate::settings::*;

#[derive(Component)]
struct Creature {}

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_creatures)
            .add_systems(Update, update_creatures);
    }
}

fn setup_creatures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
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
        crate::map::Cell {
            i: 0,
            j: 0,
        },
        Creature {}
    ));
}

fn update_creatures() {

}
