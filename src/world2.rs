use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
// use sdl2::video::Window;
use sdl2::EventPump;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const SQUARE_SIZE: u32 = 200;
const BUTTON_SIZE: u32 = 50;
enum GeneralWindow{
    First,
    Other
}
pub fn run(canvas: &mut WindowCanvas, event_pump: &mut EventPump) -> i32 {

    let mut square_rect = Rect::new(
        (WINDOW_WIDTH / 2 - SQUARE_SIZE as u32 / 2) as i32,
        (WINDOW_HEIGHT / 2 - SQUARE_SIZE as u32 / 2) as i32,
        SQUARE_SIZE,
        SQUARE_SIZE,
    );
    let mut square_rect2 = Rect::new(
        (WINDOW_WIDTH / 5 - SQUARE_SIZE as u32 / 2) as i32,
        (WINDOW_HEIGHT / 5 - SQUARE_SIZE as u32 / 2) as i32,
        SQUARE_SIZE,
        SQUARE_SIZE,
    );

    let mut button_rect = Rect::new(
        (square_rect.x() + square_rect.width() as i32 / 2 - BUTTON_SIZE as i32 / 2) as i32,
        (square_rect.y() + square_rect.height() as i32 / 2 - BUTTON_SIZE as i32 / 2) as i32,
        BUTTON_SIZE,
        BUTTON_SIZE,
    );

    let mut button_rect2 = Rect::new(
        (square_rect2.x() + square_rect2.width() as i32 / 2 - BUTTON_SIZE as i32 / 2) as i32,
        (square_rect2.y() + square_rect2.height() as i32 / 2 - BUTTON_SIZE as i32 / 2) as i32,
        BUTTON_SIZE,
        BUTTON_SIZE,
    );

    let mut is_button_pressed = false;
    let mut is_button2_pressed = false;
    let mut first = false;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break,
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
                    let mouse_point = Point::new(x, y);
                    if button_rect.contains_point(mouse_point) {
                        is_button_pressed = true;
                    }
                    if button_rect2.contains_point(mouse_point) {
                        is_button2_pressed = true;
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    is_button_pressed = false;
                    is_button2_pressed = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    if is_button_pressed {
                        let mouse_point = Point::new(x, y);
                        let dx = mouse_point.x - square_rect.width() as i32 / 2;
                        let dy = mouse_point.y - square_rect.height() as i32 / 2;
                        square_rect.set_x(dx);
                        square_rect.set_y(dy);

                        let button_x = square_rect.x() + square_rect.width() as i32 / 2
                            - BUTTON_SIZE as i32 / 2;
                        let button_y = square_rect.y() + square_rect.height() as i32 / 2
                            - BUTTON_SIZE as i32 / 2;
                        button_rect.set_x(button_x);
                        button_rect.set_y(button_y);

                    }
                    if is_button2_pressed {
                        let mouse_point = Point::new(x, y);
                        let dx = mouse_point.x - square_rect2.width() as i32 / 2;
                        let dy = mouse_point.y - square_rect2.height() as i32 / 2;
                        square_rect2.set_x(dx);
                        square_rect2.set_y(dy);

                        let button_x = square_rect2.x() + square_rect2.width() as i32 / 2
                            - BUTTON_SIZE as i32 / 2;
                        let button_y = square_rect2.y() + square_rect2.height() as i32 / 2
                            - BUTTON_SIZE as i32 / 2;
                        button_rect2.set_x(button_x);
                        button_rect2.set_y(button_y);
                    }
                }
                
                _ => {}
            }
        }

        // Очистка экрана
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        // Отрисовка квадрата
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_rect(square_rect).unwrap();
        canvas.draw_rect(square_rect2).unwrap();
        // Отрисовка кнопки
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(button_rect).unwrap();
        canvas.fill_rect(button_rect2).unwrap();
        canvas.present();
    }
}

