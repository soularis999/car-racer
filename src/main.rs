use std::error::Error;

use car_racer::init;
use car_racer::state::GameState;

fn main() -> Result<(), Box<dyn Error>>{
    let game_state = GameState::new();
    let mut game = init::new(&game_state)?;
    game.add_logic(car_racer::game::game_logic);
    // setup game
    game.run(game_state);

    Ok(())
}


