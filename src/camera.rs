use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_cursor).chain())
            .add_systems(Update, move_cursor);
    }
}

#[derive(Component)]
struct GameCursor {}

fn setup_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;
    let cursor_spawn: Vec3 = Vec3::ZERO;

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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn move_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<GameCursor>>) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_style = cursor.single_mut();
        img_style.left = Val::Px(position.x);
        img_style.top = Val::Px(position.y);
    }
}