use crate::prelude::*;
use crate::SokobanState;
mod input;
pub enum MessageOfIntent {
    None,
    MovePlayer(Point), //point is a delta not the exact location
    Quit,
    // RestartLevel,
    // Rewind,
    // Forward,
}
pub fn run_systems(state: &mut SokobanState) {
    //get player input as a message of intent
    //take snapshot of the gamestate and save it as a move backup
    let moi = input::player_input(state);
    //process message of intent
    match moi {
        MessageOfIntent::None => do_nothing(state),
        MessageOfIntent::MovePlayer(delta) => process_move(state, delta),
        MessageOfIntent::Quit => quit_game(state),
    }
    //check gamestate for victory condition if so do victory state
}
fn do_nothing(state: &mut SokobanState) {
    //nothing
}
fn process_move(state: &mut SokobanState, delta: Point) {
    //check move legality and if so move the player and/or crate
}
fn quit_game(state: &mut SokobanState) {
    //quit the game somehow
}
//player input function will have movement w/ arrow keys
//restart level w/ ctrl+r
//quit w/ ctrl+q
//go forward and backwards in turn w/ ctrl+arrow keys
//(for compatability issues make sure to "erase" the rest of the "timeline")
