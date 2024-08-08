/*
use crate::editor_systems::Command;
use crate::prelude::*;
//will maybe need to add control state variable later on?
pub fn parse_input(state: &mut EditorState, command: &mut Command) {
    let key = state.key;
    let shift = state.shift;
    let control = state.control;
    let cursor_pos = state.cursor;
    let mut cursor_delta = Point::new(0, 0);
    if key.is_some() {
        cursor_delta = match key.unwrap() {
            //arrow key cursor movement
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            //numpad cursor movement including diagonals
            VirtualKeyCode::Numpad4 => Point::new(-1, 0), //move west
            VirtualKeyCode::Numpad6 => Point::new(1, 0),  //move east
            VirtualKeyCode::Numpad8 => Point::new(0, -1), //move north
            VirtualKeyCode::Numpad2 => Point::new(0, 1),  //move south
            VirtualKeyCode::Numpad7 => Point::new(-1, -1), //move northwest
            VirtualKeyCode::Numpad9 => Point::new(1, -1), //move northeast
            VirtualKeyCode::Numpad3 => Point::new(1, 1),  //move southeast
            VirtualKeyCode::Numpad1 => Point::new(-1, 1), //move southwest
            //selecting the type of tile to paint w/ the brush
            VirtualKeyCode::Key1 => {
                state.brush_tile = Some(TileType::Wall);
                Point::new(0, 0)
            }
            VirtualKeyCode::Key2 => {
                state.brush_tile = Some(TileType::Floor);
                Point::new(0, 0)
            }
            // VirtualKeyCode::Key3 => {
            // state.brush_tile = Some(TileType::Crate);
            // Point::new(0, 0)
            // }
            VirtualKeyCode::Key4 => {
                state.brush_tile = Some(TileType::LoadingSquare);
                Point::new(0, 0)
            }
            //pressing return sends the command to actually place a tile at that point
            VirtualKeyCode::Return => {
                // command = Command::PlaceTile(cursor_pos);
                Point::new(0, 0)
            }
            _ => Point::new(0, 0),
        }
    };
}
*/
