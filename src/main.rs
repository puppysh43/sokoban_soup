#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod map;
mod player;

mod prelude {
    pub use ::bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

use eframe::egui;

struct State {
    map: Map,
    player: Player,
    //world: World,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

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
            if ui.button("Play Game").clicked() {
                if !self.launched {
                    println!("{} has launched Sokoban Soup!", self.name);
                    launchgame().unwrap();
                }
                self.launched = true;
            }
        });
    }
}

fn launchgame() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Sokoban Soup")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(ctx, State::new())
}
