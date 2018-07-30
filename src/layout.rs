/*

Box Model

- box elements have dimensions: width / height and x / y coordinates
- also can have padding, border, and margin values; defined in CSS delcarations, but live in separate structure?
- block elements are placed vertically
- inline elements placed horizontally
- inline elements must only contain block elements or inline elements; in the caseof mixed content, anonymous block renderes will be created to wrap inline elements


layout tree is similar to style tree in that each element contains children of the same type as itself



Parts:

LayoutBox {
    dimensions,
    box_type,
    children,
}

Dimensions {
    content_box,
    padding,
    margin,
    border,
}

Box {
    x,
    y,
    height,
    width,
}

EdgeSize {
    top,
    bottom,
    left,
    right,
}

EdgeType?

Display {
    Inline,
    Block,
    None
}
*/