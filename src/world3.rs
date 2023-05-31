use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::TimerSubsystem; 
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use std::fs::File;
use sdl2::pixels::Color;
use std::io::Write;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const TEXT_AREA_WIDTH: u32 = 600;
const TEXT_AREA_HEIGHT: u32 = 400;
const TEXT_FILE_PATH: &str = "text.txt";
const LINE_HEIGHT: i32 = 30;
const START_BOLD: &str = "*";
const END_BOLD: &str = "*";

pub fn run(mut canvas: &mut WindowCanvas, event_pump: &mut EventPump, timer: &TimerSubsystem) -> i32 {
    println!("Running World 3");

    let mut text_area_rect = Rect::new(
        (WINDOW_WIDTH - TEXT_AREA_WIDTH) as i32 / 2,
        (WINDOW_HEIGHT - TEXT_AREA_HEIGHT) as i32 / 2,
        TEXT_AREA_WIDTH,
        TEXT_AREA_HEIGHT,
    );
    let mut text_buffer = String::new();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font: Font = ttf_context.load_font("/home/zahar/Project.git/testo/EightBits.ttf", 24).unwrap();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    println!("Switching to World 4");
                    return 4;
                }
                Event::TextInput { text, .. } => {
                    text_buffer.push_str(&text);
                    if let Some(pos) = text_buffer.rfind(START_BOLD) {
                        if let Some(end) = text_buffer[pos + START_BOLD.len()..].find(END_BOLD) {
                            let start_index = pos + START_BOLD.len();
                            let end_index = pos + START_BOLD.len() + end;
                            let bold_text = &text_buffer[start_index..end_index];
                            let replaced_text = bold_text.to_uppercase();
                            text_buffer.replace_range(start_index..end_index, &replaced_text);
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                        // Shift + Enter pressed, add a new line
                        text_buffer.push('\n');
                        // Increase the height of the text area
                        let num_lines = text_buffer.chars().filter(|&c| c == '\n').count() + 1;
                        let new_height = LINE_HEIGHT * (num_lines as i32 + 1);
                        let mut new_rect = text_area_rect;
                        new_rect.set_height(new_height as u32);
                        text_area_rect = new_rect;
                    } else {
                        // Save the text to a file
                        if !text_buffer.is_empty() {
                            save_text_to_file(&text_buffer);
                            text_buffer.clear();
                        }
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Draw the text area
        canvas.set_draw_color(Color::BLACK);
        canvas.draw_rect(text_area_rect).unwrap();

        // Render the text buffer
        let mut y = text_area_rect.y() + 10;
        for line in text_buffer.lines() {
            let surface = font
                .render(line)
                .blended(Color::BLACK)
                .unwrap();
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let texture_rect = surface.rect();
            let dst_rect = Rect::new(
                text_area_rect.x() + 10,
                y,
                texture_rect.width(),
                texture_rect.height(),
            );
            canvas.copy(&texture, texture_rect, dst_rect).unwrap();
            y += LINE_HEIGHT;
        }

        // Update the canvas
        canvas.present();

    }
    
}

fn save_text_to_file(text: &str) {
    let mut file = File::create(TEXT_FILE_PATH).unwrap();
    file.write_all(text.as_bytes()).unwrap();
    println!("Text saved to file: {}", TEXT_FILE_PATH);
}