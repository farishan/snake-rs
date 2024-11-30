use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::render::WindowCanvas;
use sdl2::ttf;
use sdl2::video::Window;
use std::time::Instant;
use sysinfo::System;

use crate::game_context;

pub struct RendererDev<'font> {
    canvas: WindowCanvas,
    frame_count: u32,
    last_frame_time: Instant,
    fps: f32,
    system: System,
    memory_usage: f32,
    current_pid: sysinfo::Pid,
    font: &'font ttf::Font<'font, 'font>,
}

impl<'font> RendererDev<'font> {
    pub fn new(
        window: Window,
        font: &'font ttf::Font<'font, 'font>,
    ) -> Result<RendererDev<'font>, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let frame_count: u32 = 0;
        let last_frame_time = Instant::now();
        let fps: f32 = 0.0;
        let memory_usage = 0.0;
        let system = System::new_all();
        let current_pid = sysinfo::Pid::from_u32(std::process::id());
        println!("PID: {}", std::process::id());

        Ok(RendererDev {
            canvas,
            frame_count,
            last_frame_time,
            fps,
            system,
            memory_usage,
            current_pid,
            font,
        })
    }

    fn calculate_memory_usage(&mut self) {
        self.system
            .refresh_processes(sysinfo::ProcessesToUpdate::Some(&[self.current_pid]), true);
        if let Some(process) = self.system.process(self.current_pid) {
            self.memory_usage = process.memory() as f32 / 1024.0 / 1024.0; // Convert to MiB
        }
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
        self.calculate_memory_usage();
        self.calculate_fps();

        self.draw_background(context);
        let height = self.draw_fps(context)?;
        self.draw_memory_usage(context, height)?;

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

    fn draw_fps(&mut self, _context: &game_context::GameContext) -> Result<u32, String> {
        let texture_creator = self.canvas.texture_creator();

        let fps_text = format!("FPS: {:.2}fps", self.fps);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = &self
            .font
            .render(&fps_text)
            .blended(Color::RGBA(255, 255, 255, 125))
            .map_err(|e| e.to_string())?;
        let texture: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let target = Rect::new(6, 4 as i32 + 4, width, height);

        self.canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
        self.canvas.copy(&texture, None, Some(target))?;

        Ok(height)
    }

    fn draw_memory_usage(
        &mut self,
        _context: &game_context::GameContext,
        offset_height: u32,
    ) -> Result<u32, String> {
        let texture_creator = self.canvas.texture_creator();

        let fps_text = format!("Memory usage: {:.2}MB", self.memory_usage);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = &self
            .font
            .render(&fps_text)
            .blended(Color::RGBA(255, 255, 255, 125))
            .map_err(|e| e.to_string())?;
        let texture: sdl2::render::Texture<'_> = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let target = Rect::new(6, 4 + offset_height as i32 + 4, width, height);

        self.canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
        self.canvas.copy(&texture, None, Some(target))?;

        Ok(height)
    }
}
