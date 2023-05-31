use crate::jsonen::Content;

use super::jsonen;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseWheelDirection;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::surface;
use sdl2::sys::__u_short;
use sdl2::ttf::Font;
use sdl2::EventPump;
use std::time::Duration;

pub fn run(canvas: &mut WindowCanvas, event_pump: &mut EventPump) -> i32 {
    let all = jsonen::all();
    let MOVE_WIGHT: i32 = 50;
    let mut scroll_offset: i32 = 0;
    let ttf_context = sdl2::ttf::init().unwrap();
    let font: Font = ttf_context
        .load_font(
            "/home/zahar/Project.git/prelude_world/BrahmsGotischCyr.otf",
            24,
        )
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let background = texture_creator.load_texture("/home/zahar/Project.git/sdl/pivo/756896-Magic-Noah-Bradley-Pixel-Art-roots-Trees-1080P.jpg").unwrap();
    let mut textures = Vec::new();
    let mut rects = Vec::new();
    let mut y_offset = 0;
    let mut alle: String = "".to_string();
    for i in &all {
        alle.push_str(&i);
        alle.push_str("\n")
    }
    let mut theme = "DevOps";
    let line: Vec<&str> = alle.split('\n').collect();
    let mut hight_all_text = 0;
    let mut width_text = 0;
    let mut rect;
    let mut own_position;
    let mut state_theme: jsonen::Content = jsonen::new(theme.to_string(), "1".to_string());

    for linen in line {
        if linen.trim().is_empty() {
            continue;
        }
        let surface = font
            .render(linen)
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        rect = Rect::new(50, y_offset, surface.width(), 24);
        if surface.width() > width_text {
            width_text = surface.width()
        }
        y_offset += 24 as i32 + 10;
        textures.push(texture);
        rects.push(rect);
    }
    let mut texte = &state_theme.name;
    let mut description = state_theme.description;

    // let texture_rect = Rect::new(50, 50, surface.width(), surface.height());
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
                Event::MouseButtonDown { x, y, .. } => {
                    for i in &rects {
                        if i.contains_point(Point::new(x, y)) {
                            own_position = rects.iter().position(|&x| x == *i).unwrap()
                                + scroll_offset as usize / 34;
                            // if scroll_offset == 0 {own_position = own_position + scroll_offset as usize / 30} else {own_position = own_position + scroll_offset as usize / 30 - 2}

                            theme = &all[own_position];
                            state_theme = jsonen::new(theme.to_string(), "1".to_string());
                            texte = &state_theme.name;
                            description = state_theme.description;
                        }
                    }
                }
                Event::MouseWheel { y, direction, .. } => {
                    let scroll_direction = match direction {
                        MouseWheelDirection::Normal => -y,
                        MouseWheelDirection::Flipped => y,
                        _ => 0,
                    };
                    scroll_offset += scroll_direction * 34;
                    // scroll_offset += y * MOVE_WIGHT;
                    if scroll_offset < 0 {
                        scroll_offset = 0;
                    }
                    if scroll_offset > y_offset - 600 {
                        scroll_offset = y_offset - 600;
                    }
                    println!("{}", scroll_offset);
                }
                _ => {}
            }
        }
        let surname = font
            .render(texte)
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let texture_name = texture_creator
            .create_texture_from_surface(&surname)
            .unwrap();
        let rect_name = Rect::new(300, 50, surname.width(), surname.height());
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas
            .copy(
                &background,
                None,
                Rect::new(0, 0, width_text.to_owned() + 60, y_offset.to_owned() as u32),
            )
            .unwrap();
        let surdescription = font
            .render(&description)
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let texture_description = texture_creator
            .create_texture_from_surface(&surdescription)
            .unwrap();
        let rect_description = Rect::new(300, 100, surdescription.width(), surdescription.height());

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas
            .copy(
                &background,
                None,
                Rect::new(0, 0, width_text.to_owned() + 60, y_offset.to_owned() as u32),
            )
            .unwrap();
        for (texture, rect) in textures.iter().zip(rects.iter()) {
            let mut scrolled_rect = *rect;
            let text_right = rect_description.x() + rect_description.width() as i32;
            if text_right >= 800{ } 
            canvas.copy(&texture_name, None, rect_name).unwrap();
            canvas.copy(&texture_description, None, rect_description).unwrap();
            scrolled_rect.set_y(rect.y() - scroll_offset);
            canvas.copy(texture, None, Some(scrolled_rect)).unwrap();
            

            let line_start = Point::new(rect.x(), rect.y() + rect.height() as i32 - scroll_offset);
            let line_end = Point::new(
                rect.x() + rect.width() as i32,
                rect.y() + rect.height() as i32 - scroll_offset,
            );

            canvas.draw_line(line_start, line_end).unwrap();
        }

        canvas.present();
    }
}
