use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::ttf;
use std::path::Path;
use std::time::Duration;

use crate::config;
use crate::game_context;
use crate::renderer;
use crate::renderer_dev;

const FONT_PATH: &'static str = "./inter-regular-18px.ttf";
const FONT_SIZE: u16 = 16;

fn create_window(sdl_context: &sdl2::Sdl, title: &str, is_centered: bool) -> sdl2::video::Window {
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize the video subsystem");

    let mut builder = video_subsystem.window(
        title,
        config::GRID_X_SIZE * config::DOT_SIZE_IN_PXS,
        config::GRID_Y_SIZE * config::DOT_SIZE_IN_PXS,
    );

    if is_centered {
        let window = builder
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to create window");

        return window;
    } else {
        let window = builder
            .position(0, 0)
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to create window");

        return window;
    }
}

pub struct Game;

impl Game {
    pub fn new() {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2");

        let window_dev = create_window(&sdl_context, "Snake Game - Dev", false);
        let wdid = window_dev.id();

        let window = create_window(&sdl_context, "Snake Game", true);
        let wid = window.id();

        let mut event_pump = sdl_context
            .event_pump()
            .expect("Failed to create event pump");

        let mut context = game_context::GameContext::new();

        let ttf_context = ttf::init()
            .map_err(|e| e.to_string())
            .expect("Failed to create TTF context");
        let font_path = Path::new(FONT_PATH);
        let mut font = ttf_context
            .load_font(font_path, FONT_SIZE)
            .map_err(|e| e.to_string())
            .expect("Failed to create font");
        font.set_style(ttf::FontStyle::NORMAL);

        let renderer = renderer::Renderer::new(window, &font).expect("Failed to create renderer");
        let mut renderer = Some(renderer);

        let renderer_dev =
            renderer_dev::RendererDev::new(window_dev, &font).expect("Failed to create renderer");
        let mut renderer_dev = Some(renderer_dev);

        let mut tick_count = 0;

        'mainloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'mainloop,

                    Event::Window {
                        win_event: sdl2::event::WindowEvent::Close,
                        window_id,
                        ..
                    } => {
                        println!("Window with ID {} received a close event!", window_id);

                        if window_id == wid {
                            renderer = None;
                        } else if window_id == wdid {
                            renderer_dev = None;
                        }
                    }

                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::W => {
                            if context.player_direction != game_context::PlayerDirection::Down {
                                context.move_up();
                            }
                        }
                        Keycode::S => {
                            if context.player_direction != game_context::PlayerDirection::Up {
                                context.move_down();
                            }
                        }
                        Keycode::A => {
                            if context.player_direction != game_context::PlayerDirection::Right {
                                context.move_left();
                            }
                        }
                        Keycode::D => {
                            if context.player_direction != game_context::PlayerDirection::Left {
                                context.move_right();
                            }
                        }
                        Keycode::P => context.toggle_pause(),
                        Keycode::O => context.over(),
                        Keycode::R => context = game_context::GameContext::new(),
                        _ => {}
                    },
                    _ => {}
                }
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

            tick_count += 1;
            if tick_count % 2 == 0 {
                context.next_tick();
                tick_count = 0;
            }

            if let Some(renderer) = &mut renderer {
                renderer.draw(&context).expect("Failed to draw");
            }

            if let Some(renderer_dev) = &mut renderer_dev {
                renderer_dev.draw(&context).expect("Failed to draw");
            }
        }
    }
}
