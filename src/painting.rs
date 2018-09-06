use layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, BoxType, Rect};
use css::{Value, Color};
use std::path::PathBuf;
use dom::NodeType;
use text::get_text_size;

type FontPath = PathBuf;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
    Text(String, FontPath, Rect),
}

pub type DisplayList = Vec<DisplayCommand>;

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {

    render_background(list, layout_box);
    render_borders(list, layout_box);

    // If layout_box contains Text node, and if Node is of type Text, and  then render_text()
    // TODO: Find a better location for this / refactor for reusability (a similar pattern is used in layout.rs)
    if let BoxType::InlineNode(styled_node) = layout_box.box_type {
        if let NodeType::Text(ref text) = styled_node.node.node_type {

            // TODO: Implement parsing of font/font-family attributes in CSS
            // TODO: set default value (e.g. "old-english") based on default font passed in as argument
            let font_style = styled_node.lookup("font", "font-family", &Value::Keyword("old-english".to_owned()));

            if let Value::Keyword(mut font_name) = font_style {
                let mut font_path = PathBuf::new();
                font_path.push(font_name);
                font_path.set_extension("ttf"); // TODO: Look up font in filesystem and use extension of found file
                
                let mut rect = layout_box.dimensions.border_box();
                rect.height = rect.height + get_text_size(text, font_path.as_path()).1 as f32;
                
                list.push(DisplayCommand::Text(text.to_string(), font_path, rect));
            }
        }
    }

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background-color").map(|color| {
        list.push(DisplayCommand::SolidColor(color, layout_box.dimensions.border_box()))});
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return
    };
    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height,
    }));

    // Right border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height,
    }));

    // Top border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top,
    }));

    // Bottom border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }));
}

/// Return the specified color for CSS property `name`, or None if no color was specified.
fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BlockNode(style) | InlineNode(style) => match style.value(name) {
            Some(Value::ColorValue(color)) => Some(color),
            _ => None
        },
        AnonymousBlock => None
    }
}
