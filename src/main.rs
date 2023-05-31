mod world1;
mod world2;
mod world3;
mod world4;
mod world5;
mod jsonen;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let timer = sdl_context.timer().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SDL2 Worlds", 1000, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut current_world = 5;


    loop {
        match current_world {
            1 => {
                current_world = world1::run(&mut canvas, &mut event_pump);
            }
            2 => {
                current_world = world2::run(&mut canvas, &mut event_pump);
            }
            3 => {
                current_world = world3::run(&mut canvas, &mut event_pump, &timer);
            }
            4 => {
                current_world = world4::run(&mut canvas, &mut event_pump, &timer);
            }
            5 => {
                current_world = world5::run(&mut canvas, &mut event_pump);
            }
            99 => break,

            _ => panic!("Invalid current_world value"),
        }
    }
}