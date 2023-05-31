use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::image::LoadTexture;
use std::path::Path;
use sdl2::EventPump;
use sdl2::render::WindowCanvas;

use std::fs;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 600;
const LEFT_PANEL_WIDTH: u32 = 300;
const FONT_SIZE: u16 = 16;
struct FileSystemItem {
    name: String,
    is_directory: bool,
    is_open: bool,
    items: Vec<FileSystemItem>,
}
pub fn run(canvas: &mut WindowCanvas, event_pump: &mut EventPump) -> i32{
    let texture_creator = canvas.texture_creator();

    let left_panel_texture = texture_creator
        .load_texture(Path::new("/home/zahar/Project.git/sdl/pivo/22.jpeg"))
        .unwrap();

    let right_panel_texture = texture_creator
        .load_texture(Path::new("/home/zahar/Project.git/sdl/pivo/21.jpeg"))
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("/home/zahar/Project.git/prelude_world/EightBits.ttf");
    let font = ttf_context.load_font(font_path, FONT_SIZE).unwrap();
    

    // Создаем вектор элементов файловой системы
    let mut root_item = load_directory(Path::new("/home/zahar/Project.git/prelude_world"));


    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    println!("Switching to World 3");
                    return 3;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    if x < LEFT_PANEL_WIDTH as i32 {
                        // Клик в левой панели
                        let mut y_pos = 10;
                        let mut clicked_item = None;
                        for item in &mut root_item.items {
                            let (_, item_height) = font.size_of(&item.name).unwrap();
                            if y > y_pos && y < y_pos + item_height as i32 {
                                clicked_item = Some(item);
                                break;
                            }
                            y_pos += item_height as i32 + 10;
                        }

                        if let Some(item) = clicked_item {
                            toggle_directory(item);
                        }
                    }
                }
                _ => {}
            }
        }

        // Очищаем холст
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Рисуем левую панель
        canvas
            .copy(&left_panel_texture, None, Rect::new(0, 0, LEFT_PANEL_WIDTH as u32, WINDOW_HEIGHT))
            .unwrap();

        // Рисуем разделительную линию
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_line((LEFT_PANEL_WIDTH as i32, 0), (LEFT_PANEL_WIDTH as i32, WINDOW_HEIGHT as i32)).unwrap();

        // Выводим список файлов и папок в левой панели
        let mut y = 10;
    for item in &root_item.items {
        let text_color = if item.is_directory {
            Color::RGB(0, 0, 255) // Синий цвет для папок
        } else {
            Color::RGB(0, 0, 0) // Черный цвет для файлов
        };
        let text_surface = font
            .render(&item.name)
            .blended(text_color)
            .map_err(|e| e.to_string())
            .unwrap();
        let text_texture = texture_creator
            .create_texture_from_surface(&text_surface)
            .map_err(|e| e.to_string())
            .unwrap();
        let (_, h) = text_surface.size();
        let text_rect = Rect::new(10, y, text_surface.width(), h);
        canvas.copy(&text_texture, None, text_rect).unwrap();
        y += h as i32 + 10;

        // Рисуем стрелку для папки
        if item.is_directory {
            let arrow_texture = if item.is_open {
                texture_creator
                    .load_texture(Path::new("/home/zahar/Project.git/sdl/pivo/arow.png"))
                    .unwrap()
            } else {
                texture_creator
                    .load_texture(Path::new("/home/zahar/Project.git/sdl/pivo/arow2.png"))
                    .unwrap()
            };
            let arrow_rect = Rect::new(LEFT_PANEL_WIDTH as i32 - 30, y - h as i32 / 2, 20, 20);
            canvas.copy(&arrow_texture, None, arrow_rect).unwrap();

            // Если папка открыта, рисуем вложенные элементы
            if item.is_open {
                let mut inner_y = y;
                for inner_item in &item.items {
                    let inner_text_color = if inner_item.is_directory {
                        Color::RGB(0, 0, 255) // Синий цвет для папок
                    } else {
                        Color::RGB(0, 0, 0) // Черный цвет для файлов
                    };
                    let inner_text_surface = font
                        .render(&inner_item.name)
                        .blended(inner_text_color)
                        .map_err(|e| e.to_string())
                        .unwrap();
                    let inner_text_texture = texture_creator
                        .create_texture_from_surface(&inner_text_surface)
                        .map_err(|e| e.to_string())
                        .unwrap();
                    let (_, inner_h) = inner_text_surface.size();
                    let inner_text_rect = Rect::new(30, inner_y, inner_text_surface.width(), inner_h);
                    canvas.copy(&inner_text_texture, None, inner_text_rect).unwrap();
                    inner_y += inner_h as i32 + 10;
                }
                y = inner_y;
            }
        }
    }


        // Рисуем правую панель
        canvas
            .copy(&right_panel_texture, None, Rect::new(LEFT_PANEL_WIDTH as i32, 0, (WINDOW_WIDTH - LEFT_PANEL_WIDTH) as u32, WINDOW_HEIGHT))
            .unwrap();

        // Обновляем экран
        canvas.present();
    }
}

fn load_directory(directory: &Path) -> FileSystemItem {
    let name = directory.file_name().unwrap().to_string_lossy().to_string();
    let mut item = FileSystemItem {
        name: name.clone(),
        is_directory: true,
        is_open: false,
        items: Vec::new(),
    };

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().into_string().unwrap();
                let is_directory = entry.file_type().unwrap().is_dir();
                item.items.push(FileSystemItem {
                    name,
                    is_directory,
                    is_open: false,
                    items: Vec::new(),
                });
            }
        }
    }

    item
}

fn toggle_directory(item: &mut FileSystemItem) {
    item.is_open = !item.is_open;
    
}