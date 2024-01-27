use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, move_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::splat(0.7),
            ..default()
        },
        ..default()
    });
}

fn move_camera(window: Query<&Window>, mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let window: &Window = window.single();
    let mut transform = camera_query.get_single_mut().unwrap();

    if let Some(position) = window.cursor_position() {
        if (position.x > window.width() - 60.0) && (position.x < window.width()) {
            transform.translation.x += 5.0;
        }

        if (position.x < 60.0) && (position.x > 0.0) {
            transform.translation.x -= 5.0;
        }

        if (position.y > window.height() - 60.0) && (position.y < window.height()) {
            transform.translation.y -= 5.0;
        }

        if (position.y < 60.0) && (position.y > 0.0) {
            transform.translation.y += 5.0;
        }
    }
}
