use sdl2::pixels::Color;
use sdl2::rect::Rect;
// use sdl2::render::TextureCreator;
use sdl2::render::TextureQuery;
use sdl2::render::WindowCanvas;
use sdl2::ttf;
use sdl2::video::Window;
use std::path::Path;
use std::time::Instant;

use crate::constants;
use crate::game_context;
use crate::point;

pub struct Renderer {
    canvas: WindowCanvas,
    frame_count: u32,
    last_frame_time: Instant,
    fps: f32,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let frame_count: u32 = 0;
        let last_frame_time = Instant::now();
        let fps:f32 = 0.0;

        Ok(Renderer { canvas, frame_count, last_frame_time, fps })
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

    fn calculate_fps(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time);

        self.frame_count += 1;
        if elapsed.as_secs_f32() >= 1.0 {
            self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_frame_time = now;
        }
    }

    pub fn draw(&mut self, context: &game_context::GameContext) -> Result<(), String> {
        self.calculate_fps();

        self.draw_background(context);
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.draw_text(context)?;

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

    fn draw_text(&mut self, _context: &game_context::GameContext) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();

        // Load a font
        /* @TODO fix font initialization placement, should be on the constructor? */
        let ttf_context = ttf::init().map_err(|e| e.to_string())?;
        let font_path = Path::new("./inter-regular-18px.ttf");
        let font_size = 16;
        let mut font = ttf_context.load_font(font_path, font_size)?;
        font.set_style(ttf::FontStyle::NORMAL);

        let fps_text = format!("FPS: {:.2}fps", self.fps);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(&fps_text)
            .blended(Color::RGBA(255, 255, 255, 125))
            .map_err(|e| e.to_string())?;
        let texture: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let target = Rect::new(6, 4, width, height);

        self.canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
        self.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }
}
