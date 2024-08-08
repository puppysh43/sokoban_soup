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
pub fn run_systems(state: &mut SokobanState, ctx: &mut BTerm) {
    //get player input as a message of intent
    let moi = input::player_input(state);
    //process message of intent
    match moi {
        MessageOfIntent::None => do_nothing(state),
        MessageOfIntent::MovePlayer(delta) => process_move(state, delta),
        MessageOfIntent::Quit => quit_game(state),
    }
    render(state, ctx);

    //check gamestate for victory condition if so do victory state
}
fn do_nothing(state: &mut SokobanState) {
    //do nothing
}
fn process_move(state: &mut SokobanState, delta: Point) {
    //get player position
    let player_pos = state.player;
    //calculate the player's new position w/ the delta
    let new_player_pos = Point::new(player_pos.x + delta.x, player_pos.y + delta.y);
    //make a variable to be filled once it checks if the move collides with any crates.
    let mut moving_crate: Option<Crate> = None;
    //make a temp variable to check if the player has moved
    let mut has_moved = false;
    //if there is a crate in the spot the player is moving to mark it as being present
    if state.crates.contains_key(&new_player_pos) {
        moving_crate = Some(state.crates.remove(&new_player_pos).unwrap());
    }
    //if there is a crate where the player is apply the delta to the crate position and see if
    //the crate can also safely move to where it would go
    if state.map.can_enter_tile(new_player_pos) && moving_crate.is_some() {
        if state.map.can_enter_tile(Point::new(
            new_player_pos.x + delta.x,
            new_player_pos.y + delta.y,
        )) {
            //if so move the player and adjust the crate position accordingly
            state.player = new_player_pos;
            state.crates.insert(
                Point::new(new_player_pos.x + delta.x, new_player_pos.y + delta.y),
                moving_crate.unwrap(),
            );
            has_moved = true;
        }
        //if there is no crate being moved just make sure the player can move and move them!
    } else if state.map.can_enter_tile(new_player_pos) && moving_crate.is_none() {
        state.player = new_player_pos;
        has_moved = true;
    }
    //then take a snapshot of the state as if
    if has_moved {
        let move_made = Move::new(state.player, state.crates.clone(), state.movecount);
        state.moves.push(move_made);
    }
}
fn quit_game(state: &mut SokobanState) {
    state.quitting = true;
}
//player input function will have movement w/ arrow keys
//restart level w/ ctrl+r
//quit w/ ctrl+q
//go forward and backwards in turn w/ ctrl+arrow keys
//(for compatability issues make sure to "erase" the rest of the "timeline")
fn render(state: &mut SokobanState, ctx: &mut BTerm) {
    ctx.set_active_console(0);
    //first render the game map
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let pt = Point::new(x, y);
            let idx = map_idx(x, y);
            if state.map.in_bounds(pt) {
                match state.map.tiles[idx] {
                    TileType::Wall => {
                        ctx.set(x, y, GRAY, BLACK, to_cp437('#'));
                    }
                    TileType::Floor => {
                        // draw_batch.set(pt, ColorPair::new(LIGHT_GRAY, BLACK), to_cp437('.'));
                        ctx.set(x, y, LIGHT_GRAY, BLACK, to_cp437('.'));
                    }
                    TileType::LoadingSquare => {
                        // draw_batch.set(pt, ColorPair::new(ORANGE, BLACK), to_cp437('O'));
                        ctx.set(x, y, ORANGE, BLACK, to_cp437('O'));
                    }
                }
            }
        }
    }
    //then render the crates
    for crate_pos in state.crates.keys() {
        ctx.set(crate_pos.x, crate_pos.y, YELLOW, BLACK, to_cp437('X'));
    }
    //then render the player
    ctx.set(state.player.x, state.player.y, GREEN, BLACK, to_cp437('@'));
}
