use sdl2;
use sdl2::Sdl;
use sdl2::rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;


pub fn init() -> Sdl {
    return sdl2::init().unwrap();
}

pub fn window(context: &Sdl) -> Window {
    let video_subsystem = context.video().unwrap();
    return video_subsystem
        .window("A browser?", 640, 480)
        .resizable()
        .build()
        .unwrap();
}

pub fn render(context: &Sdl, window: Window) {
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        canvas.set_draw_color(Color::RGBA(0xff, 0, 0, 0xff));
        canvas.fill_rect(rect::Rect::new(10, 10, 50, 50)).unwrap();
        canvas.present();
    }
}
