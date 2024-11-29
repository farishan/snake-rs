use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::ops::Add;
use std::time::Duration;

const GRID_X_SIZE: u32 = 40;
const GRID_Y_SIZE: u32 = 30;
const DOT_SIZE_IN_PXS: u32 = 20;

enum GameState {
    Playing,
    Paused,
    Over,
}

#[derive(PartialEq)]
enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone)]
struct Point(i32, i32);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct GameContext {
    player_position: Vec<Point>,
    player_direction: PlayerDirection,
    food: Point,
    state: GameState,
}

impl GameContext {
    fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point(3, 3),
        }
    }

    fn next_tick(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }

        let head_position = self
            .player_position
            .first()
            .expect("Failed to get player position first value");
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };

        if next_head_position.0 < 0
            || next_head_position.0 >= GRID_X_SIZE as i32
            || next_head_position.1 < 0
            || next_head_position.1 >= GRID_Y_SIZE as i32
        {
            self.state = GameState::Over
        } else if next_head_position.0 == self.food.0 && next_head_position.1 == self.food.1 {
            self.food = Point(
                rand::thread_rng().gen_range(0..GRID_X_SIZE as i32),
                rand::thread_rng().gen_range(0..GRID_Y_SIZE as i32),
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

    fn move_up(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }
        self.player_direction = PlayerDirection::Up;
    }

    fn move_down(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }
        self.player_direction = PlayerDirection::Down;
    }

    fn move_right(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }
        self.player_direction = PlayerDirection::Right;
    }

    fn move_left(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }
        self.player_direction = PlayerDirection::Left;
    }

    fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Paused => GameState::Playing,
            GameState::Playing => GameState::Paused,
            GameState::Over => GameState::Over,
        }
    }

    fn over(&mut self) {
        self.state = GameState::Over;
    }
}

struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }

    fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_player(context)?;
        self.draw_food(context)?;

        self.canvas.present();

        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) {
        let color = match context.state {
            GameState::Over => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30, 30, 30),
            GameState::Playing => Color::RGB(60, 60, 60),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);

        for point in &context.player_position {
            self.draw_dot(&point)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;

        Ok(())
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize the video subsystem");

    let window = video_subsystem
        .window(
            "Snake Game",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("Failed to create window");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to create event pump");

    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window).expect("Failed to create renderer");

    println!("WASD to move. P to play/pause. R to restart. Esc to close.");

    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => {
                        if context.player_direction != PlayerDirection::Down {
                            context.move_up();
                        }
                    }
                    Keycode::S => {
                        if context.player_direction != PlayerDirection::Up {
                            context.move_down();
                        }
                    }
                    Keycode::A => {
                        if context.player_direction != PlayerDirection::Right {
                            context.move_left();
                        }
                    }
                    Keycode::D => {
                        if context.player_direction != PlayerDirection::Left {
                            context.move_right();
                        }
                    }
                    Keycode::P => context.toggle_pause(),
                    Keycode::O => context.over(),
                    Keycode::R => context = GameContext::new(),
                    _ => {}
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        frame_counter += 1;
        if frame_counter % 2 == 0 {
            context.next_tick();
            frame_counter = 0;
        }

        renderer.draw(&context).expect("Failed to draw");
    }
}
