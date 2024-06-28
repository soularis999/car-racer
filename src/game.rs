use std::f32::consts::FRAC_PI_8;

use rand::{Rng, thread_rng};
use rusty_engine::game::Engine;
use rusty_engine::keyboard::KeyCode;
use rusty_engine::NORTH;
use rusty_engine::prelude::{Sprite, SpritePreset};
use rusty_engine::prelude::bevy::utils::HashMap;
use rusty_engine::text::Text;

use crate::{LIFE_BOARD, MAX_X, MAX_Y, PLAYER_NAME, RESULT, SCORE_BOARD};
use crate::state::{GameState, GameStatus};

pub fn game_logic(game: &mut Engine, game_state: &mut GameState)
{
    match game_state.tick_game(game.delta)
    {
        GameStatus::Begin => do_begin(game, game_state),
        GameStatus::Run => do_play(game, game_state),
        GameStatus::Crash => do_crash(game),
        GameStatus::Win | GameStatus::Loose => do_end(game, game_state),
    };
    update_score_board(&mut game.texts, game_state);
}

fn do_begin(game: &mut Engine, game_state: &mut GameState) {
    let res = game.texts.get_mut(RESULT);
    if res.is_none()
    {
        let _ = game.add_text(RESULT, game_state.begin_label());
    }
    else {
        res.unwrap().value = game_state.begin_label();
    }
}

fn do_play(game: &mut Engine, game_state: &mut GameState) {
    ensure_correct_direction(game);
    ensure_no_result(game);
    spawn_blocks(game, game_state);
    move_blocks(&mut game.sprites, game.delta_f32, game_state);

    move_car(game);
    check_crash(game, game_state);
}

fn ensure_no_result(game: &mut Engine) {
    game.texts.remove(RESULT);
}

fn do_crash(game: &mut Engine) {
    let player = game.sprites.get_mut(PLAYER_NAME).unwrap();
    player.rotation += FRAC_PI_8;
}

fn do_end(game: &mut Engine, game_state: &mut GameState) {
    let res = game.texts.get_mut(RESULT);
    if res.is_none()
    {
        let _ = game.add_text(RESULT, game_state.end_label());
    }
    else {
        res.unwrap().value = game_state.end_label();
    }

    if game.keyboard_state.pressed_any(&[KeyCode::N])
    {

    }
}



fn ensure_correct_direction(game: &mut Engine) {
    let player = game.sprites.get_mut(PLAYER_NAME).unwrap();
    player.rotation = NORTH;
}

fn spawn_blocks(game: &mut Engine, game_state: &mut GameState) {
    let next_obstacle = game_state.spawn_next_obstacle();
    if next_obstacle.is_some()
    {
        let obst = game.add_sprite(next_obstacle.unwrap(),
                                   SpritePreset::RacingBarrelBlue);
        obst.translation.x = thread_rng().gen_range((-MAX_X as f32)..(MAX_X as f32));
        obst.translation.y = MAX_Y as f32 + 50.0;
        obst.collision = true;
    }
}

fn move_blocks(sprites: &mut HashMap<String, Sprite>,
               time_delta: f32,
               game_state: &mut GameState) {
    game_state.retain_obstacles(|obstacle| {
        let spr = sprites.get_mut(obstacle).unwrap();
        spr.translation.y -= 50.0 * time_delta;

        let mut res = true;
        if spr.translation.y < -MAX_Y as f32 {
            sprites.remove(obstacle);
            res = false;
        }
        res
    });
}

fn move_car(game: &mut Engine)
{
    let player = game.sprites.get_mut(PLAYER_NAME).unwrap();
    if game.keyboard_state.pressed_any(&[KeyCode::Right, KeyCode::L])
    {
        let val = player.translation.x + 250.0 * game.delta_f32;
        let max_right = (MAX_X as f32) - 30.0;
        player.translation.x = if max_right - val < 0.0 { max_right } else { val };
    }
    if game.keyboard_state.pressed_any(&[KeyCode::Left, KeyCode::H])
    {
        let val = player.translation.x - 250.0 * game.delta_f32;
        let max_left = (-MAX_X as f32) + 30.0;
        player.translation.x = if val - max_left < 0.0 { max_left } else { val };
    }
    if game.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::K])
    {
        let val = player.translation.y + 250.0 * game.delta_f32;
        let max_top = (MAX_Y as f32) - 50.0;
        player.translation.y = if max_top - val < 0.0 { max_top } else { val };
    }
    if game.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::J])
    {
        let val = player.translation.y - 250.0 * game.delta_f32;
        let max_bottom = (-MAX_Y as f32) + 50.0;
        player.translation.y = if val - max_bottom < 0.0 { max_bottom } else { val };
    }
}

fn check_crash(game: &mut Engine, game_state: &mut GameState) {
    let obst: Vec<String> = game.collision_events.drain(..)
        .flat_map(|event| event.pair.into_iter())
        .filter(|obst| game_state.is_obstacle(obst))
        .collect();
    perform_crash(game, obst, game_state);
}

fn perform_crash(game: &mut Engine,
                 obstacles: Vec<String>,
                 game_state: &mut GameState) {
    obstacles.into_iter()
        .for_each(|obstacle| {
            game.sprites.remove(&obstacle);
            game_state.remove_obstacle(&obstacle);
            game_state.transition_to_crash();
        });
}

fn update_score_board(texts: &mut HashMap<String, Text>, game_state: &mut GameState) {
    texts.get_mut(SCORE_BOARD).unwrap().value = game_state.score();
    texts.get_mut(LIFE_BOARD).unwrap().value = game_state.lives();
}

