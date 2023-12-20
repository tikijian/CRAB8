
extern crate sdl2;

use std::time::{Duration, Instant};

pub mod computer;
pub mod utils;

use crate::computer::Computer;
use crate::computer::display::WIDTH as DISPLAY_WIDTH;
use crate::computer::display::HEIGHT as DISPLAY_HEIGHT;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const SCALE_FACTOR: f32 = 10.0;

pub fn main() -> Result<(), String> {
    // init Computer
    let mut computer = Computer::new();
    computer.reset();

    // load ROM
    let args: Vec<String> = std::env::args().collect();
    let rom_name = match args.get(1) {
        Some(value) => value.as_str(),
        None => "IBM"
    };
    let rom_data = utils::load_rom(rom_name).unwrap();
    computer.load_rom(rom_data);

    // init SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window_width = DISPLAY_WIDTH as u32 * SCALE_FACTOR as u32;
    let window_height = DISPLAY_HEIGHT as u32 * SCALE_FACTOR as u32;

    let window = video_subsystem
        .window("CRAB-8", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_scale(SCALE_FACTOR, SCALE_FACTOR).unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut last_time = Instant::now();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        computer.emulate_cycle();

        if computer.should_clear_screen {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            computer.should_clear_screen = false;
        }

        if computer.should_redraw {
            canvas.set_draw_color(Color::WHITE);
            let mut x_pos: i32 = 0;
            let mut y_pos: i32 = 0;
            let display_divisor: i32 = (DISPLAY_WIDTH - 1).into();

            for pixel_data in computer.display.memory {
                if pixel_data != 0 {
                    canvas.draw_point(Point::new(x_pos, y_pos)).unwrap();
                }

                // calculate Y coordinate in linear array of pixels
                if x_pos != 0 && (x_pos % display_divisor as i32) == 0 {
                    x_pos = 0;
                    y_pos += 1;
                } else {
                    x_pos += 1;
                }
            }

            canvas.present();
            computer.should_redraw = false;
        }

        if last_time.elapsed() >= Duration::from_millis(1000 / 60) {
            if computer.delay_timer > 0 {
                computer.delay_timer -= 1;
            }
            if computer.sound_timer > 0 {
                computer.sound_timer -= 1;
            }

            last_time = Instant::now();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

    Ok(())
}