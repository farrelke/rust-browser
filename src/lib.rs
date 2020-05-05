use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

pub mod dom;
pub mod html;
pub mod css;
pub mod style;
pub mod layout;
pub mod painting;
pub mod canvas2d;

#[derive(Debug, Serialize, Deserialize)]
struct ReturnVal {
    pub dom: dom::Node,
    pub style: css::Stylesheet,
    pub display_list: painting::DisplayList
}

#[wasm_bindgen]
pub fn render(html_str: String, css_str: String) -> Result<JsValue, JsValue> {
   // let t = dom::text(String::from("Hello"));
   // let e = dom::elem(String::from("div"), vec!(t));

    let root_node = html::parse(html_str);
    let stylesheet = css::parse(css_str);

    let style_root = style::style_tree(&root_node, &stylesheet);

    // Since we don't have an actual window, hard-code the "viewport" size.
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width  = 800.0;
    viewport.content.height = 600.0;
    let layout_root = layout::layout_tree(&style_root, viewport);
    let display_list = painting::build_display_list(&layout_root);

    canvas2d::paint(viewport.content, &display_list);


    let returnVal = ReturnVal {
        dom: root_node,
        style: stylesheet,
        display_list: display_list
    };

    Ok(JsValue::from_serde(&returnVal).unwrap())
}