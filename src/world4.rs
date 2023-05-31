use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::TimerSubsystem;

pub fn run(
    mut canvas: &mut WindowCanvas,
    event_pump: &mut EventPump,
    timer: &TimerSubsystem,
) -> i32 {
    println!("Running World 4");

    let texture_creator = canvas.texture_creator();
    let moon_texture = texture_creator
        .load_texture("/home/zahar/Project.git/sdl/pivo/moon.png")
        .unwrap();
    let background = texture_creator.load_texture("/home/zahar/Project.git/sdl/pivo/756896-Magic-Noah-Bradley-Pixel-Art-roots-Trees-1080P.jpg").unwrap();
    let crystal_texture = texture_creator.load_texture("/home/zahar/Project.git/sdl/pivo/free_crystal_dark_frakassets/crystal_dark_64x64_24f_20d.png").unwrap();
    let mut moon_rect = Rect::new(100, 100, 100, 100);

    let mut moon_score = 0;
    let mut crystal_score = 0;
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return 99,
                Event::KeyDown {
                    keycode: Some(Keycode::F6),
                    ..
                } => {
                    canvas
                        .window_mut()
                        .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                        .unwrap();
                }
                Event::MouseButtonDown { x, y, .. } => {
                    if x >= 1400 && x <= 1450 && y >= 384 && y <= 432 { return 3 }
                    if moon_rect.contains_point(Point::new(x, y)) { return 2 }
                }
                Event::MouseMotion { x, .. } => {
                    if x < canvas.window().size().0 as i32 / 2 {moon_rect.x = canvas.window().size().0 as i32 - 200 ; moon_rect.y = canvas.window().size().1 as i32 - 400} else {moon_rect.x = 100; moon_rect.y = 100}
                }

                _ => {}
            }
        }

        let timer_loop = timer.ticks();
        if timer_loop % 15 == 0 {
            if crystal_score >= 23 {
                crystal_score = 0
            } else {
                crystal_score += 1
            }
        }
        if moon_score >= 57 {
            moon_score = 0
        } else {
            moon_score += 1
        }
        render(
            &mut canvas,
            &background,
            &moon_texture,
            &crystal_texture,
            //i,
            moon_score,
            crystal_score,
            moon_rect,
        )
        .unwrap();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 20));
    }
}


fn render(
    canvas: &mut WindowCanvas,
    background: &Texture,
    moon_texture: &Texture,
    crystal_texture: &Texture,
    moon_score: i32,
    crystal_score: i32,
    moon_rect: Rect,
) -> Result<(), String> {
    let frame_moon = Rect::new(0 + 48 as i32 * moon_score, 0, 48, 48);
    let frame_crystal = Rect::new(0 + 64 as i32 * crystal_score, 0, 64, 64);

    canvas.copy(background, None, None)?;
    canvas.copy(moon_texture, frame_moon, moon_rect)?;
    canvas.copy(crystal_texture, frame_crystal, Rect::new(1000, 100, 50, 50))?;

    canvas.present();

    Ok(())
}
