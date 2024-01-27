use bevy::prelude::*;
use crate::components::*;
use crate::settings::{CELL_SIZE, SELECTED_CURSOR_BLINK};

#[derive(Resource)]
struct SelectedCursorTimer {
    pub timer: Timer,
}

impl Default for SelectedCursorTimer {
    fn default() -> Self {
        SelectedCursorTimer { timer: Timer::from_seconds(SELECTED_CURSOR_BLINK, TimerMode::Repeating) }
    }
}

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCursorTimer>()
            .add_systems(Startup, setup_cursor)
            .add_systems(Update, (move_cursor, change_cursor))
            .add_systems(Startup, setup_select_cursor)
            .add_systems(Update, (update_select_cursor, tick_select_cursor_timer));
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
                width: Val::Px(CELL_SIZE),
                height: Val::Px(CELL_SIZE),
                margin: UiRect {
                    left: -Val::Px(CELL_SIZE / 2.0),
                    top: -Val::Px(CELL_SIZE / 2.0),
                  ..default()
                },
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
        img_style.left = Val::Px(position.x);
        img_style.top = Val::Px(position.y);
    }
}

fn change_cursor(
    selected_creature_query: Query<Entity, With<SelectedCreature>>,
    mut cursor_query: Query<&mut UiImage, With<GameCursor>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut cursor_image) = cursor_query.get_single_mut() {
        let is_creature_selected = selected_creature_query.get_single().is_ok();

        if is_creature_selected {
            *cursor_image = asset_server.load("cursor_selected.png").into();
        } else {
            *cursor_image = asset_server.load("cursor.png").into();
        }
    }
}

fn setup_select_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SelectCursor {},
        SpriteBundle {
            texture: asset_server.load("creature_select.png").into(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 3.0,
                },
                scale: Vec3::splat(CELL_SIZE / 96.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));
}

fn update_select_cursor(
    selected_creature_query: Query<&Transform, (With<SelectedCreature>, Without<SelectCursor>)>,
    mut select_cursor_query: Query<(&mut Visibility, &mut Transform), With<SelectCursor>>,
    select_cursor_blink_timer: Res<SelectedCursorTimer>,
) {
    if let Ok((mut select_cursor_visibility, mut selected_cursor_transform)) = select_cursor_query.get_single_mut() {
        if let Ok(selected_creature_transform) = selected_creature_query.get_single() {
            let translation = selected_creature_transform.translation.clone();

            selected_cursor_transform.translation.x = translation.x;
            selected_cursor_transform.translation.y = translation.y;

            if select_cursor_blink_timer.timer.finished() {
                *select_cursor_visibility = if *select_cursor_visibility == Visibility::Visible {Visibility::Hidden} else {Visibility::Visible};
            }

        } else {
            *select_cursor_visibility = Visibility::Hidden;
        }
    }
}

fn tick_select_cursor_timer(
    mut select_cursor_blink_timer: ResMut<SelectedCursorTimer>,
    time: Res<Time>
) {
    select_cursor_blink_timer.timer.tick(time.delta());
}
