use rand::Rng;

use crate::config;
use crate::point;

pub enum GameState {
    Playing,
    Paused,
    Over,
}

#[derive(PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

pub struct GameContext {
    pub player_position: Vec<point::Point>,
    pub player_direction: PlayerDirection,
    pub food: point::Point,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![point::Point(3, 4), point::Point(2, 4), point::Point(1, 4)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: point::Point(3, 6),
        }
    }

    pub fn next_tick(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }

        let head_position = self
            .player_position
            .first()
            .expect("Failed to get player position first value");
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + point::Point(0, -1),
            PlayerDirection::Down => *head_position + point::Point(0, 1),
            PlayerDirection::Right => *head_position + point::Point(1, 0),
            PlayerDirection::Left => *head_position + point::Point(-1, 0),
        };

        if next_head_position.0 < 0
            || next_head_position.0 >= config::GRID_X_SIZE as i32
            || next_head_position.1 < 0
            || next_head_position.1 >= config::GRID_Y_SIZE as i32
        {
            self.state = GameState::Over
        } else if next_head_position.0 == self.food.0 && next_head_position.1 == self.food.1 {
            self.food = point::Point(
                rand::thread_rng().gen_range(0..config::GRID_X_SIZE as i32),
                rand::thread_rng().gen_range(0..config::GRID_Y_SIZE as i32),
            );
        } else {
            self.player_position.pop();
        }

        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();

        for (i, point) in self.player_position.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if point.0 == next_head_position.0 && point.1 == next_head_position.1 {
                self.state = GameState::Over
            }
        }
    }

    pub fn move_up(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }
        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }
        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_right(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }
        self.player_direction = PlayerDirection::Right;
    }

    pub fn move_left(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }
        self.player_direction = PlayerDirection::Left;
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Paused => GameState::Playing,
            GameState::Playing => GameState::Paused,
            GameState::Over => GameState::Over,
        }
    }

    pub fn over(&mut self) {
        self.state = GameState::Over;
    }
}
