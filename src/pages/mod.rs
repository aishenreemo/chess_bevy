mod home;
mod play;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::ui::button::ButtonInteractivePlugin;

#[derive(States, PartialEq, Eq, Clone, Copy, Default, Hash, Debug)]
pub enum GameState {
    #[default]
    Home,
    Play,
}

pub struct PagesPlugin;

impl Plugin for PagesPlugin {
    fn build(&self, app: &mut App) {
        use GameState::*;

        app.init_state::<GameState>();

        app.add_systems(OnEnter(Home), home::setup);
        app.add_systems(Update, home::button_system);
        app.add_systems(OnExit(Home), despawn_ui);

        app.add_systems(OnEnter(Play), play::setup);
        app.add_systems(Update, play::button_system);
        app.add_systems(OnExit(Play), despawn_ui);
    }
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ButtonInteractivePlugin)
            .add(PagesPlugin)
    }
}
