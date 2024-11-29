use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod constants;
mod game_context;
mod point;
mod renderer;

fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize the video subsystem");

    let window = video_subsystem
        .window(
            "Snake Game",
            constants::GRID_X_SIZE * constants::DOT_SIZE_IN_PXS,
            constants::GRID_Y_SIZE * constants::DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("Failed to create window");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to create event pump");

    let mut context = game_context::GameContext::new();
    let mut renderer = renderer::Renderer::new(window).expect("Failed to create renderer");

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

        frame_counter += 1;
        if frame_counter % 2 == 0 {
            context.next_tick();
            frame_counter = 0;
        }

        renderer.draw(&context).expect("Failed to draw");
    }
}
