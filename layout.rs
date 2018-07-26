//CSS box model. All sizes are in px.

struct Dimensions {
  // Position of the content area relative to the document origin:
  content: Rect,

  // Surrounding edges:
  padding: EdgeSizes,
  border: EdgeSizes,
  margin: EdgeSizes,
}

struct Rect {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
}

struct EdgeSizes {
  left: f32,
  right: f32,
  top: f32,
  bottom: f32,
}

struct LayoutBox<'a> {
  dimensions: Dimensions,
  box_type: BoxType<'a>,
  children: Vec<LayoutBox<'a>>,
}

enum BoxType<'a> {
  BlockNode(&'a StyledNode<'a>),
  InlineNode(&'a StyledNode<'a>),
  AnonymousBlock,
}

enum Display {
  Inline,
  Block,
  None,
}

impl StyledNode {
  // Return the specified value of a property if it exists, otherwise `None`.
  fn value(&self, name: &str) -> Option<Value> {
    self.specified_values.get(name).map(|v| v.clone())
  }

  // The value of the `display` property (defaults to inline).
  fn display(&self) -> Display {
    match self.value("display") {
      Some(Keyword(s)) => match &*s {
        "block" => Display::Block,
        "none" => Display::None,
        _ => Display::Inline
      },
      _ => Display::Inline
    }
  }
}

// Build the tree of LayoutBoxes, but don't perform any layout calculations yet.
fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
  // Create the root box.
  let mut root = LayoutBox::new(match style_node.display() {
    Block => BlockNode(style_node),
    Inline => InlineNode(style_node),
    DisplayNone => panic!("Root node has display: none.")
  });

  // Create the descendant boxes.
  for child in &style_node.children {
    match child.display() {
      Block => root.children.push(build_layout_tree(child)),
      Inline => root.get_inline_container().children.push(build_layout_tree(child)),
      DisplayNone => {} //Skip nodes with `display: none;`
    }
  }
  return root;
}

impl LayoutBox {
  // Constructor function
  fn new(box_type: BoxType) -> LayoutBox {
    LayoutBox {
      box_type: box_type,
      dimensions: Default::defautl(), // initially set all fields to 0.0
      children: Vec::new(),
    }
  }

}

fn get_inline_container(&mut self) -> &mut LayoutBox {
  match self.box_type {
    InlineNode(_) | AnonymousBlock => self,
    BlockNode(_) => {
      // If we've just generated an anonymous block box, keep using it.
      // Otherwise, create a new one.
      match self.children.last() {
        Some(&LayoutBox { box_type: AnonymousBlock,..}) => {}
        _ => self.children.push(LayoutBox::new(AnonymousBlock))
      }
      self.children.last_mut().unwrap()
    }
  }
}
