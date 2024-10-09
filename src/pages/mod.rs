mod home;
mod play;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::chess::ChessBoard;
use crate::ui::button::ButtonInteractivePlugin;
use crate::ui::palette::ColorPalettePlugin;

#[derive(States, PartialEq, Eq, Clone, Copy, Default, Hash, Debug)]
pub enum GameState {
    Home,
    #[default]
    Play,
}

pub struct PagesPlugin;

impl Plugin for PagesPlugin {
    fn build(&self, app: &mut App) {
        use GameState::*;

        app.init_state::<GameState>();

        app.add_systems(OnEnter(Home), home::setup);
        app.add_systems(Update, home::button_system);
        app.add_systems(OnExit(Home), despawn_all);

        app.add_systems(OnEnter(Play), play::setup);
        app.add_systems(Update, play::button_system);
        app.add_systems(Update, play::drag_system);
        app.add_systems(OnExit(Play), despawn_all);
    }
}

fn despawn_all(
    mut commands: Commands,
    nodes: Query<Entity, With<Node>>,
    chessboards: Query<Entity, With<ChessBoard>>,
) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn();
    }

    for entity in chessboards.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ButtonInteractivePlugin)
            .add(ColorPalettePlugin)
            .add(PagesPlugin)
    }
}
