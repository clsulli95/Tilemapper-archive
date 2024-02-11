use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(AssetsBeingLoaded::default())
        .add_systems(Startup, setup)
        .add_systems(Update, check_assets_ready)
        .add_systems(Update, ui_example_system)
        .run();
}

#[derive(Default, Resource)]
struct AssetsBeingLoaded {
    images: Vec<Handle<Image>>,
}

#[derive(Default, Resource)]
struct AssetHandleManager {
    images: Vec<Handle<Image>>,
}

fn setup(server: Res<AssetServer>, mut loading: ResMut<AssetsBeingLoaded>) {
    let tilemap = server.load("tilemap.png");
    loading.images.push(tilemap);
}

fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: ResMut<AssetsBeingLoaded>,
    mut handle_mgr: ResMut<AssetHandleManager>,
) {
    loading
        .images
        .iter()
        .map(|x| match server.get_load_state(x.id()) {
            Some(LoadState::Failed) => println!("Asset {x:?} failed to load!"),
            Some(LoadState::Loaded) => handle_mgr.images.push(Handle::from(x.clone())),
            Some(LoadState::NotLoaded) => println!("Asset {x:?} has not been loaded!"),
            Some(LoadState::Loading) => println!("Asset {x:?} is loading!"),
            _ => println!("Ahh!"),
        });
}

fn ui_example_system(mut contexts: EguiContexts) {}
