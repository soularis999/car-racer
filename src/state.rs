use std::cmp::max;
use std::collections::HashSet;
use std::time::Duration;

use rusty_engine::prelude::*;

use crate::OBSTACLE_NAME;
use crate::state::GameStatus::{Crash, Loose, Run, Win};

#[derive(Debug, Resource, Clone)]
pub struct GameState
{
    timer: Timer,
    next_obst: usize,
    obstacles: HashSet<String>,
    lives: usize,
    score: usize,
    game_status: GameStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameStatus
{
    Begin,
    Run,
    Crash,
    Win,
    Loose,
}

impl GameState {
    pub fn new() -> GameState {
        GameState::default()
    }

    pub fn tick_game(self: &mut GameState, dur: Duration) -> GameStatus
    {
        let timer_finished = self.timer.tick(dur).just_finished();
        match self.game_status
        {
            GameStatus::Begin | Crash => {
                if timer_finished
                {
                    self.game_status = Run;
                    self.timer.set_duration(Duration::from_secs(2));
                }
                self.game_status
            },
            Run => {
                if 50 == self.score
                {
                    self.game_status = Win;
                } else if 0 == self.lives
                {
                    self.game_status = Loose;
                }
                self.game_status
            },
            _ => self.game_status,
        }
    }

    pub fn transition_to_crash(self: &mut GameState)
    {
        self.game_status = Crash;
        self.lives -= 1;
        self.timer.reset();
        self.timer.set_duration(Duration::from_secs(1));
    }

    pub fn spawn_next_obstacle(self: &mut GameState) -> Option<String>
    {
        if self.timer.just_finished()
        {
            let name: String = format!("{}-{}", OBSTACLE_NAME, self.next_obst);
            self.next_obst += 1;
            self.obstacles.insert(name.clone());
            return Some(name);
        }
        None
    }

    pub fn retain_obstacles<F>(self: &mut GameState, visitor: F)
    where
        F: FnMut(&String) -> bool,
    {
        let ct = self.obstacles.len();
        self.obstacles.retain(visitor);
        self.score += max(0, ct - self.obstacles.len())
    }

    pub fn remove_obstacle(self: &mut GameState, obstacle: &String) -> bool
    {
        self.obstacles.remove(obstacle)
    }

    pub fn is_obstacle(self: &GameState, obstacle: &String) -> bool {
        self.obstacles.contains(obstacle)
    }

    pub fn begin_label(&self) -> String
    {
        format!("{}", 3 - self.timer.elapsed().as_secs())
    }

    pub fn end_label(&self) -> String
    {
        format!("You {:?}", self.game_status)
    }
    pub fn lives(self: &GameState) -> String
    {
        format!("Lives: {}", self.lives)
    }

    pub fn score(self: &GameState) -> String
    {
        format!("Score: {}", self.score)
    }
}

impl Default for GameState
{
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            next_obst: 0,
            obstacles: HashSet::new(),
            lives: 3,
            score: 0,
            game_status: GameStatus::Begin,
        }
    }
}
