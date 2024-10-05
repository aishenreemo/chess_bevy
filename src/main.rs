mod chess;
mod pages;
mod ui;

use bevy::prelude::*;
use bevy::winit::WinitSettings;
use pages::CorePlugins;

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

    app.add_plugins(DefaultPlugins.set(plugin));
    app.add_plugins(CorePlugins);

    app.add_systems(PreStartup, setup);

    #[cfg(feature = "world-inspector")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.insert_resource(ClearColor(Color::hsl(0., 1., 0.)))
        .insert_resource(WinitSettings::desktop_app());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
