mod chess;
mod pages;
mod ui;

use bevy::prelude::*;
use pages::CorePlugins;
use ui::palette::ColorPalette;

fn main() {
    let mut app = App::new();

    let window = Window {
        title: "Chess".into(),
        position: WindowPosition::At((0, 0).into()),
        resolution: (800., 600.).into(),
        resizable: false,
        decorations: true,
        ..default()
    };

    let plugin = WindowPlugin {
        primary_window: Some(window),
        ..default()
    };

    app.add_plugins(DefaultPlugins.set(plugin).set(ImagePlugin::default_nearest()));
    app.add_plugins(CorePlugins);

    app.add_systems(PreStartup, setup);

    #[cfg(feature = "world-inspector")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

fn setup(mut commands: Commands, palette: Res<ColorPalette>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: palette.black.into(),
            ..default()
        },
        ..default()
    });
}
