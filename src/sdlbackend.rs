use sdl2;
use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

use painting::{DisplayCommand,DisplayList};
use text::render_text_to_canvas;

pub fn init() -> Sdl {
    return sdl2::init().unwrap();
}

pub fn window(context: &Sdl) -> Window {
    let video_subsystem = context.video().unwrap();
    return video_subsystem
        .window("A browser?", 1000, 800)
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

        // TODO: Determine if this drawing should be in loop or not (it's not in a loop in tff-demo)
        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        for element in commands {
            match element {
                DisplayCommand::SolidColor(color, rect) => {
                    canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));

                    canvas.fill_rect(Rect::new(
                        rect.x as i32,
                        rect.y as i32,
                        rect.width as u32,
                        rect.height as u32
                    )).unwrap();
                },
                DisplayCommand::Text(text, font_path, rect) => {
                    render_text_to_canvas(text, font_path, &mut canvas, sdl2::rect::Rect::new(rect.x as i32, rect.y as i32, rect.width as u32, rect.height as u32));
                }
            }  
        }

        canvas.present();
    }
}
