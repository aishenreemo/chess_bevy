use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Debug)]
pub struct CursorPosition(pub Vec2);

pub struct CursorPositionPlugin;

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition(Vec2::default()));
        app.add_systems(Update, cursor_system);
    }
}

fn cursor_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let Some(mut position) = windows.single().cursor_position() else {
        return;
    };

    position -= windows.single().resolution.size() / 2.;
    position.y = -position.y;
    cursor_position.0 = position;
}
