mod editor_systems;
mod game_systems;
mod map;
mod player;

mod prelude {
    pub use ::bracket_lib::prelude::*;
    //screen size
    pub const SCREEN_WIDTH: i32 = 19;
    pub const SCREEN_HEIGHT: i32 = 17;
    pub use crate::map::*;
    pub use crate::Crate;
    pub use crate::EditorState;
    pub use crate::Move;
    pub use crate::SokobanState;
    pub use std::collections::HashMap;
}

use prelude::*;

use eframe::egui;
pub struct EditorState {
    map: Map,
    cursor: Point,
    key: Option<VirtualKeyCode>,
    shift: bool,
    control: bool,
    brush_tile: Option<TileType>,
}
impl EditorState {
    fn new() -> Self {
        Self {
            map: Map::new(),
            cursor: Point::new(0, 0),
            key: None,
            shift: false,
            control: false,
            brush_tile: None,
        }
    }
}

impl GameState for EditorState {
    fn tick(&mut self, ctx: &mut BTerm) {
        //clear the screen
        ctx.cls();
        //capture user input
        (self.key, self.shift, self.control) = (ctx.key, ctx.shift, ctx.control);
    }
}

pub struct SokobanState {
    key: Option<VirtualKeyCode>,
    control: bool,
    map: Map,
    player: Point,
    crates: HashMap<Point, Crate>,
    movecount: u32,
    moves: Vec<Move>,
    quitting: bool,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Crate {
    id: u32,
}

pub struct Move {
    player: Point,
    crates: HashMap<Point, Crate>,
    movecount: u32,
}
impl Move {
    fn new(player: Point, crates: HashMap<Point, Crate>, movecount: u32) -> Self {
        Self {
            player,
            crates,
            movecount,
        }
    }
}

impl SokobanState {
    fn from_file(path: String) -> Self {
        let (map, player_spawn, crates) = read_data_from_string(path);
        Self {
            key: None,
            control: false,
            map,
            player: player_spawn,
            crates,
            movecount: 0,
            moves: Vec::new(),
            quitting: false,
        }
    }
}
use std::fs;
fn read_data_from_string(path: String) -> (Map, Point, HashMap<Point, Crate>) {
    //get the raw string from file
    let mut raw_data =
        fs::read_to_string(path).expect("failed to properly read the raw map data string");
    //then trim all the whitespace and make everything uppercase
    raw_data = raw_data.to_uppercase();
    raw_data.retain(|c| !c.is_whitespace());
    let mut map = Map::new();
    let mut player_spawn = Point::new(0, 0);
    let mut crates: HashMap<Point, Crate> = HashMap::new();
    let mut crate_num = 0;
    let mut index = 0;
    for char in raw_data.chars() {
        match char {
            '#' => {
                map.tiles[index as usize] = TileType::Wall;
            }
            '.' => {
                map.tiles[index as usize] = TileType::Floor;
            }
            'X' => {
                map.tiles[index as usize] = TileType::Floor;
                // crates.insert(, )
                //need to get an index to point function working
            }
            'O' => {
                map.tiles[index as usize] = TileType::LoadingSquare;
            }
            '@' => {
                map.tiles[index as usize] = TileType::Floor;
                //set player spawnpoint using index to point function
            }
            _ => {
                println!("Unrecognized character in raw map data")
            }
        }
        index += 1;
    }
    return (map, player_spawn, crates);
}

impl GameState for SokobanState {
    fn tick(&mut self, ctx: &mut BTerm) {
        //clear screen
        ctx.cls();
        //get the current input for the tick
        (self.key, self.control) = (ctx.key, ctx.control);
        //run all the game systems
        game_systems::run_systems(self);
        if self.quitting {
            ctx.quit();
        }
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
                        Gamemode::Editor => launcheditor().unwrap(),
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

    main_loop(ctx, EditorState::new())
}
