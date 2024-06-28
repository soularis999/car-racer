use rusty_engine::game::{Game, Window};
use rusty_engine::NORTH;
use rusty_engine::prelude::{Resource, SpritePreset, Vec2};
use rusty_engine::prelude::bevy::prelude::WindowResizeConstraints;
use rusty_engine::prelude::bevy::utils::thiserror::Error;
use rusty_engine::prelude::bevy::window::EnabledButtons;

use crate::{INIT_POS, LIFE_BOARD, MAX_X, MAX_Y, PLAYER_NAME, SCORE_BOARD};
use crate::state::GameState;

#[derive(Error, Debug)]
pub enum InitErr
{}

pub fn new<S: Resource + Send + Sync + 'static>(game_state: &GameState) -> Result<Game<S>, InitErr> {
    let mut game = Game::new();
    game.window_settings(Window {
        title: "Car Racer!".to_string(),
        resize_constraints: WindowResizeConstraints {
            min_width: MAX_X as f32 * 2.0,
            min_height: MAX_Y as f32 * 2.0,
            max_width: MAX_X as f32 * 2.0,
            max_height: MAX_Y as f32 * 2.0,
        },
        resizable: false,
        enabled_buttons: EnabledButtons
        {
            maximize: false,
            ..EnabledButtons::default()
        },
        ..Window::default()
    });

    create_board(&mut game, game_state);
    create_main_player(&mut game);

    Ok(game)
}

fn create_board<S: Resource + Send + Sync + 'static>(game: &mut Game<S>, game_state: &GameState) {
    let text = game.add_text(SCORE_BOARD, game_state.score());
    text.translation = Vec2::new(MAX_X as f32 - 80.0, -MAX_Y as f32 + 50.0);

    let life = game.add_text(LIFE_BOARD, game_state.lives());
    life.translation = Vec2::new(-MAX_X as f32 + 60.0, -MAX_Y as f32 + 50.0);
}

fn create_main_player<S: Resource>(game: &mut Game<S>)
{
    let player = game.add_sprite(PLAYER_NAME, SpritePreset::RacingCarGreen);
    player.translation = Vec2::new(INIT_POS.0, INIT_POS.1);
    player.rotation = NORTH;
    player.scale = 1.0;
    player.collision = true;
}
