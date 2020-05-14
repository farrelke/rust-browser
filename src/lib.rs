use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod canvas2d;
pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod painting;
pub mod style;

#[derive(Debug, Serialize, Deserialize)]
struct ReturnVal {
    pub dom: dom::Node,
    pub style: css::Stylesheet,
    pub layout: layout::DisplayLayoutNode,
    pub display_list: painting::DisplayList,
}

#[wasm_bindgen]
pub fn render(html_str: String, css_str: String) -> Result<JsValue, JsValue> {
    // let t = dom::text(String::from("Hello"));
    // let e = dom::elem(String::from("div"), vec!(t));

    let root_node = html::parse(html_str);
    let stylesheet = css::parse(css_str);

    let style_root = style::style_tree(&root_node, &stylesheet, None);

    // Since we don't have an actual window, hard-code the "viewport" size.
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;
    let layout_root = layout::layout_tree(&style_root, viewport);
    let display_list = painting::build_display_list(&layout_root);

    canvas2d::paint(viewport.content, &display_list);

    let return_val = ReturnVal {
        layout: layout_root.to_display_tree(),
        dom: root_node,
        style: stylesheet,
        display_list: display_list,
    };

    Ok(JsValue::from_serde(&return_val).unwrap())
}
