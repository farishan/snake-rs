use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use crate::constants;
use crate::game_context;
use crate::point;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &point::Point) -> Result<(), String> {
        let point::Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * constants::DOT_SIZE_IN_PXS as i32,
            y * constants::DOT_SIZE_IN_PXS as i32,
            constants::DOT_SIZE_IN_PXS,
            constants::DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }

    pub fn draw(&mut self, context: &game_context::GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_player(context)?;
        self.draw_food(context)?;

        self.canvas.present();

        Ok(())
    }

    fn draw_background(&mut self, context: &game_context::GameContext) {
        let color = match context.state {
            game_context::GameState::Over => Color::RGB(0, 0, 0),
            game_context::GameState::Paused => Color::RGB(30, 30, 30),
            game_context::GameState::Playing => Color::RGB(60, 60, 60),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_player(&mut self, context: &game_context::GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);

        for point in &context.player_position {
            self.draw_dot(&point)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, context: &game_context::GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;

        Ok(())
    }
}
