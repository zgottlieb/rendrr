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
