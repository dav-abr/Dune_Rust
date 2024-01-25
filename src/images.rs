use bevy::prelude::*;

#[derive(Resource)]
pub struct ImagesResource {
    pub cursor: UiImage,
    pub concrete: UiImage,
}

pub struct ImagesPlugin;

impl Plugin for ImagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_images);
    }
}

fn setup_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ImagesResource {
        cursor: asset_server.load("cursor.png").into(),
        concrete: asset_server.load("concrete.png").into(),
    });
}
