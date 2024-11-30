use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::render::WindowCanvas;
use sdl2::ttf;
use sdl2::video::Window;

use crate::config;
use crate::game_context;
use crate::point;

pub struct Renderer<'font> {
    canvas: WindowCanvas,
    font: &'font ttf::Font<'font, 'font>,
}

impl<'font> Renderer<'font> {
    pub fn new(
        window: Window,
        font: &'font ttf::Font<'font, 'font>,
    ) -> Result<Renderer<'font>, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Renderer { canvas, font })
    }

    fn draw_dot(&mut self, point: &point::Point) -> Result<(), String> {
        let point::Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * config::DOT_SIZE_IN_PXS as i32,
            y * config::DOT_SIZE_IN_PXS as i32,
            config::DOT_SIZE_IN_PXS,
            config::DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }

    pub fn draw(&mut self, context: &game_context::GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.draw_instructions(context)?;

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

    fn draw_instructions(&mut self, _context: &game_context::GameContext) -> Result<u32, String> {
        let texture_creator = self.canvas.texture_creator();

        let text = "WASD to move. 'P' = play/pause. 'R' = restart. 'Esc' = exit.";

        // render a surface, and convert it to a texture bound to the canvas
        let surface = &self
            .font
            .render(&text)
            .blended(Color::RGBA(255, 255, 255, 125))
            .map_err(|e| e.to_string())?;
        let texture: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let target = Rect::new(6, 4, width, height);

        self.canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
        self.canvas.copy(&texture, None, Some(target))?;

        Ok(height)
    }
}
