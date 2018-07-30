use sdl2;
use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

use css;
use layout;
use painting::{DisplayCommand,DisplayList};

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

pub fn render(context: &Sdl, window: Window, commands: &DisplayList) {
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

        for element in commands {
            let (color, rect) = match element {
                DisplayCommand::SolidColor(color, rect) => (color, rect),
                _ => (&css::Color { r: 0, g: 0, b: 0, a: 0 },
                      &layout::Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }),
            };

            canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));

            canvas.fill_rect(Rect::new(
                rect.x as i32,
                rect.y as i32,
                rect.width as u32,
                rect.height as u32
            )).unwrap();
        }

        canvas.present();
    }
}
