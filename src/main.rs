#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log;
use pretty_env_logger;

use eframe::egui;

// Enum that will be used as a global state for the game
//#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
//pub enum AppState {
//    #[default]
//    Setup,
//    Run,
//}

fn main() -> Result<(), eframe::Error> {
    std::env::set_var("TILEMAPPER_LOG_LEVEL", "info");
    pretty_env_logger::init_custom_env("TILEMAPPER_LOG_LEVEL");
    log::info!("Hello World!");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My GUI",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )

    //App::new()
    //    .add_plugins(DefaultPlugins)
    //    .add_plugins(EguiPlugin)
    //    .add_state::<AppState>()
    //    .add_plugins(asset_loader::AssetLoaderPlugin)
    //    .add_plugins(tilemap_loader::TilemapLoaderPlugin)
    //    .add_systems(Update, update.run_if(in_state(AppState::Run)))
    //    .run();
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            ui.image(egui::include_image!("../assets/tilemap.png"));
        });
    }
}

//fn update() {
//    info!("[UpdatePlugin] Update!");
//}
