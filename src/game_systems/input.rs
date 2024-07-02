use crate::game_systems::MessageOfIntent;
use crate::prelude::*;
use crate::SokobanState;

pub fn player_input(state: &mut SokobanState) -> MessageOfIntent {
    let key = state.key;
    let control = state.control;
    if key.is_some() {
        let player_delta = match key.unwrap() {
            //arrow key player movement
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        return MessageOfIntent::MovePlayer(player_delta);
    }
    return MessageOfIntent::None;
}
