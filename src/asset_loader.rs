use crate::AppState;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetManager>()
            .add_systems(OnEnter(AppState::Setup), setup)
            .add_systems(Update, load_assets.run_if(in_state(AppState::Setup)));
    }
}

#[derive(Default, Resource)]
pub struct AssetManager {
    pub images: Vec<Handle<Image>>,
}

impl AssetManager {
    pub fn log_all_states(&self, server: &Res<AssetServer>) {
        self.log_asset_state(server, LoadState::NotLoaded);
        self.log_asset_state(server, LoadState::Loading);
        self.log_asset_state(server, LoadState::Loaded);
        self.log_asset_state(server, LoadState::Failed);
    }

    pub fn log_asset_state(&self, server: &Res<AssetServer>, state: LoadState) {
        self.images
            .iter()
            .filter(|x| server.get_load_state(x.id()) == Some(state))
            .for_each(|x| {
                let path = x.path();
                info!("[AssetLoaderPlugin] {path:?} is {state:#?}!")
            });
    }
}

fn setup(server: Res<AssetServer>, mut loading: ResMut<AssetManager>) {
    let tilemap = server.load("tilemap.png");
    loading.images.push(tilemap);
    info!("[AssetLoaderPlugin] Setup!");
}

fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: ResMut<AssetManager>,
    mut state: ResMut<NextState<AppState>>,
) {
    let loaded_all_assets = loading
        .images
        .iter()
        .all(|x| match server.get_load_state(x.id()) {
            Some(LoadState::Loaded) => true,
            _ => false,
        });

    if loaded_all_assets {
        info!("[AssetLoaderPlugin] Loaded all assets!");
        state.set(AppState::Run);
    } else {
        info!("[AssetLoaderPlugin] All assets not loaded!");
        loading.log_all_states(&server);
    }
}
