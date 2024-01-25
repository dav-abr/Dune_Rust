use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_cursor).chain())
            .add_systems(Update, (move_cursor, move_camera).chain());
    }
}

#[derive(Component)]
struct GameCursor {}

fn setup_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
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

fn move_camera(window: Query<&Window>, mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let window: &Window = window.single();
    let mut transform = camera_query.get_single_mut().unwrap();

    if let Some(position) = window.cursor_position() {
        if (position.x > window.width() - 60.0) & (position.x < window.width()) {
            transform.translation.x += 5.0;
        }

        if (position.x < 60.0) & (position.x > 0.0) {
            transform.translation.x -= 5.0;
        }

        if (position.y > window.height() - 60.0) & (position.y < window.height()) {
            transform.translation.y -= 5.0;
        }

        if (position.y < 60.0) & (position.y > 0.0) {
            transform.translation.y += 5.0;
        }
    }
}
