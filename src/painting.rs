
use crate::layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};
use crate::css::{Value, Color};
use serde::{Deserialize, Serialize};
use crate::dom::NodeType;

#[derive(Debug, Deserialize, Serialize)]
pub enum DisplayCommand {
    SolidColor(Rect, Color),
    Text(Rect, String)
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
    render_text(list, layout_box);
    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_text(list: &mut DisplayList, layout_box: &LayoutBox) {
    match layout_box.box_type {
        BlockNode(style) | InlineNode(style) => match &style.node.node_type {
            NodeType::Text(text) => {
                let text = text.parse().unwrap();
                let d = &layout_box.dimensions;
                let border_box = d.border_box();
                list.push(DisplayCommand::Text(border_box, text));
            },
            _ => return
        },
        AnonymousBlock => return
    };
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color|
        list.push(DisplayCommand::SolidColor(layout_box.dimensions.border_box(), color)));
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height,
    }, color));

    // Right border
    list.push(DisplayCommand::SolidColor(Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height,
    }, color));

    // Top border
    list.push(DisplayCommand::SolidColor(Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top,
    }, color));

    // Bottom border
    list.push(DisplayCommand::SolidColor(Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }, color));
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


trait Clamp {
    fn clamp(self, lower: Self, upper: Self) -> Self;
}
impl Clamp for f32 {
    fn clamp(self, lower: f32, upper: f32) -> f32 {
        self.max(lower).min(upper)
    }
}