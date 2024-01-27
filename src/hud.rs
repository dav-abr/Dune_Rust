use bevy::prelude::*;
use crate::components::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor)
            .add_systems(Update, (move_cursor));
    }
}

fn setup_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;

    commands.spawn((
        ImageBundle {
            image: asset_server.load("cursor.png").into(),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Global(15),
            ..default()
        },
        GameCursor {}
    ));
}

fn move_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<GameCursor>>) {
    let window: &Window = window.single();

    if let Some(position) = window.cursor_position() {
        let mut img_style = cursor.single_mut();

        // todo: Change to cursor image width and height
        img_style.left = Val::Px(position.x - 16.0);
        img_style.top = Val::Px(position.y - 16.0);
    }
}
