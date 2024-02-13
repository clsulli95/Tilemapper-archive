use crate::asset_loader::AssetManager;
use crate::AppState;

use bevy::{prelude::*, render::render_resource::Texture};
use bevy_egui::{egui, render_systems::EguiTextureId, EguiContexts, EguiPlugin, EguiSettings};

pub struct TilemapLoaderPlugin;

impl Plugin for TilemapLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SidePanelInputHandler>()
            .add_systems(OnEnter(AppState::Run), setup)
            .add_systems(Update, draw.run_if(in_state(AppState::Run)));
    }
}

#[derive(Default, Resource)]
struct SidePanelInputHandler {
    label: String,
    image_texture: Option<egui::TextureHandle>,
}

fn setup(
    mut input_handler: ResMut<SidePanelInputHandler>,
    asset_mgr: Res<AssetManager>,
    mut contexts: EguiContexts,
) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });

    input_handler.image_texture.get_or_insert_with(|| {
        contexts.ctx_mut().load_texture(
            "example-image",
            egui::ColorImage::example(),
            Default::default(),
        )
    });
}

fn draw(
    mut input_handler: ResMut<SidePanelInputHandler>,
    asset_mgr: Res<AssetManager>,
    mut rendered_texture_id: Local<Option<egui::TextureId>>,
    mut contexts: EguiContexts,
) {
    let handle = input_handler.image_texture.as_ref().unwrap().clone();

    *rendered_texture_id
        .get_or_insert_with(|| contexts.add_image(asset_mgr.images.first().unwrap().clone_weak()));

    let ctx = contexts.ctx_mut();
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut input_handler.label);
            });

            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                handle.id(),
                handle.size_vec2(),
            )));

            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                rendered_texture_id.unwrap(),
                [256.0, 256.0],
            )));
        });
}
