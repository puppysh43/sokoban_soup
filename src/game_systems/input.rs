use crate::game_systems::MessageOfIntent;
use crate::prelude::*;
use crate::SokobanState;

pub fn player_input(state: &mut SokobanState) -> MessageOfIntent {
    let key = state.key;
    let control = state.control;
    if key.is_some() {
        match key.unwrap() {
            VirtualKeyCode::Left => {
                return MessageOfIntent::MovePlayer(Point::new(-1, 0));
            }
            VirtualKeyCode::Right => {
                return MessageOfIntent::MovePlayer(Point::new(1, 0));
            }
            VirtualKeyCode::Up => {
                return MessageOfIntent::MovePlayer(Point::new(0, -1));
            }
            VirtualKeyCode::Down => {
                return MessageOfIntent::MovePlayer(Point::new(0, 1));
            }
            VirtualKeyCode::Q => {
                return MessageOfIntent::Quit;
            }
            _ => {
                return MessageOfIntent::None;
            }
        }
    } else {
        return MessageOfIntent::None;
    }
}
