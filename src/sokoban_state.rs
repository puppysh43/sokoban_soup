use crate::prelude::*;

pub struct SokobanState {
    pub key: Option<VirtualKeyCode>,
    pub control: bool,
    pub map: Map,
    pub player: Point,
    pub crates: HashMap<Point, Crate>,
    pub movecount: u32,
    pub moves: Vec<Move>,
    pub quitting: bool,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Crate {
    pub id: u32,
}

pub struct Move {
    pub player: Point,
    pub crates: HashMap<Point, Crate>,
    pub movecount: u32,
}
impl Move {
    pub fn new(player: Point, crates: HashMap<Point, Crate>, movecount: u32) -> Self {
        Self {
            player,
            crates,
            movecount,
        }
    }
}

impl SokobanState {
    pub fn from_file(path: String) -> Self {
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
        crate::game_systems::run_systems(self);
        if self.quitting {
            ctx.quit();
        }
    }
}
