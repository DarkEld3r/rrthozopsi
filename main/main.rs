extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("rrthozopsi", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();

    let mut timer = context.timer().unwrap();
    let mut fps = 0;
    let mut start_time = timer.ticks();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        renderer.clear();
        renderer.present();

        fps += 1;

        let current_time = timer.ticks();
        let diff_time = current_time - start_time;
        if diff_time > 1_000 {
            println!("FPS: {}", fps);
            start_time = current_time;
            fps = 0;
        }
    }
}
