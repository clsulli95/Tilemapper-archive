pub mod asset_loader;
pub mod tilemap_loader;

use crate::asset_loader::AssetManager;
use bevy_egui::EguiPlugin;
use log;
use pretty_env_logger;

use bevy::prelude::*;
//use bevy_egui::{egui, EguiContexts, EguiPlugin};

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Setup,
    Run,
}

fn main() {
    std::env::set_var("TILEMAPPER_LOG_LEVEL", "info");
    pretty_env_logger::init_custom_env("TILEMAPPER_LOG_LEVEL");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_state::<AppState>()
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(tilemap_loader::TilemapLoaderPlugin)
        .add_systems(Update, update.run_if(in_state(AppState::Run)))
        .run();
}

fn update() {
    info!("[UpdatePlugin] Update!");
}
