mod editor_systems;
mod game_systems;
mod map;
mod sokoban_state;

mod prelude {
    pub use ::bracket_lib::prelude::*;
    //screen size
    pub const SCREEN_WIDTH: i32 = 19;
    pub const SCREEN_HEIGHT: i32 = 17;
    pub use crate::game_systems::*;
    pub use crate::map::*;
    pub use crate::sokoban_state::Crate;
    pub use crate::sokoban_state::Move;
    pub use crate::sokoban_state::SokobanState;
    pub use std::collections::HashMap;
}

use prelude::*;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sokoban Soup",
        options,
        Box::new(|cc| Box::<SokobanSoupLauncher>::default()),
    )
}
#[derive(PartialEq)]
enum Gamemode {
    InGame,
    Editor,
}
struct SokobanSoupLauncher {
    gamemode: Gamemode,
    launched: bool,
    name: String,
}

impl Default for SokobanSoupLauncher {
    fn default() -> Self {
        Self {
            gamemode: Gamemode::InGame,
            launched: false,
            name: "".to_string(),
        }
    }
}

impl eframe::App for SokobanSoupLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select the game or the level editor.");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            let gamemode_label = ui.label("Select Gamemode");
            if ui
                .add(egui::RadioButton::new(
                    self.gamemode == Gamemode::InGame,
                    "Sokoban Game",
                ))
                .clicked()
            {
                self.gamemode = Gamemode::InGame
            }
            if ui
                .add(egui::RadioButton::new(
                    self.gamemode == Gamemode::Editor,
                    "Level Editor",
                ))
                .clicked()
            {
                self.gamemode = Gamemode::Editor
            }
            if ui.button("Launch").clicked() {
                if !self.launched {
                    match self.gamemode {
                        Gamemode::InGame => launchgame().unwrap(),
                        Gamemode::Editor => println!("launching sokoban level editor"), //launcheditor().unwrap(),
                    }
                    println!("{} has launched Sokoban Soup!", self.name);
                }
                self.launched = true;
            }
        });
    }
}

fn launchgame() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("Sokoban Soup")
        .with_fps_cap(30.0)
        .with_fitscreen(true)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("tileset.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "tileset.png")
        .build()?;

    main_loop(ctx, SokobanState::from_file("levels/test.txt".to_string()))
}

/*
fn launcheditor() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("Level Edtor")
        .with_fps_cap(30.0)
        .with_fitscreen(true)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("tileset.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "tileset.png")
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "tileset.png")
        .build()?;

    // main_loop(ctx, EditorState::new())
}*/
