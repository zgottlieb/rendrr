extern crate sdl2;

use std::path::Path;
use std::path::PathBuf;

use sdl2::rect::Rect;
use sdl2::pixels::Color;

static FONT_SIZE: u16 = 50;
static FONT_STYLE: sdl2::ttf::FontStyle = sdl2::ttf::STYLE_BOLD;

// handle the annoying Rect i32
// Taken from rust-sdl2/examples/ttf-demo.rs
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn get_text_size(text: &str, font_path: &Path) -> (u32, u32) {
    // Creating a context and loading a font here may be suboptimal;
    // we already need to load the font in render_text_to_canvas, so
    // there may be a way to preload the font and use it as needed
    
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(font_path, FONT_SIZE).unwrap();
    font.size_of(text).unwrap()
}

pub fn render_text_to_canvas(text: &str, font_path: &PathBuf, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, rect: Rect) {
    let ttf_context = sdl2::ttf::init().unwrap();

    let font = ttf_context.load_font(font_path.as_path(), FONT_SIZE).unwrap();
    // font.set_style(FONT_STYLE);

    let (font_width, font_height) = font.size_of(text).unwrap();
    
    // create the target destination for the texture
    let target = rect!(rect.x, rect.y, font_width, font_height);

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font.render(text)
                    .blended(Color::RGBA(0, 255, 0, 255))
                    .unwrap();
        
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    canvas.copy(&texture, None, Some(target)).unwrap();
}
