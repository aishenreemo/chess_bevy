#![allow(dead_code)]

mod play;
mod ui_classes;

use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_ui_dsl::*;
use play::play_button_system;
use play::setup_play;
use ui_classes::*;

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum GameState {
    #[default]
    Home,
    Play,
}

#[derive(Component, Debug, PartialEq)]
enum HomeButton {
    Play,
    Quit,
}

fn main() {
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

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(plugin));

    #[cfg(feature = "world-inspector")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.init_state::<GameState>();

    app.add_systems(Startup, setup_camera);
    app.add_systems(OnEnter(GameState::Home), setup_home);
    app.add_systems(OnExit(GameState::Home), destroy_ui);
    app.add_systems(OnEnter(GameState::Play), setup_play);
    app.add_systems(OnExit(GameState::Play), destroy_ui);
    app.add_systems(Update, home_button_system.run_if(in_state(GameState::Home)));
    app.add_systems(Update, play_button_system.run_if(in_state(GameState::Play)));

    app.insert_resource(ClearColor(Color::hsl(0., 1., 0.)))
        .insert_resource(WinitSettings::desktop_app());

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_home(mut commands: Commands, asset_server: Res<AssetServer>) {

    root(c_root, &asset_server, &mut commands, |p| {
        text("Chess", c_text, (c_text_white, c_text_h1), p);
        text_buttoni(
            "Play",
            c_button,
            (c_text_black, c_text_h2),
            HomeButton::Play,
            p,
        );
        text_buttoni(
            "Quit",
            c_button,
            (c_text_black, c_text_h2),
            HomeButton::Quit,
            p,
        );
    });
}

fn home_button_system(
    mut ui_entities: Query<
        (
            &HomeButton,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (id, interaction, mut bg_color, mut border_color, children) in &mut ui_entities {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Pressed if id == &HomeButton::Quit => {
                exit.send(AppExit::Success);
            }
            Interaction::Pressed if id == &HomeButton::Play => {
                next_state.set(GameState::Play);
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::WHITE;
                *bg_color = Color::BLACK.into();
                border_color.0 = Color::WHITE.into();
            }
            _ => {
                text.sections[0].style.color = Color::BLACK;
                border_color.0 = Color::NONE.into();
                *bg_color = Color::WHITE.into();
            }
        }
    }
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
