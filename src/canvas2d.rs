use crate::painting::{DisplayList, DisplayCommand};
use crate::layout::Rect;
use wasm_bindgen::JsCast;


pub fn paint(viewport: Rect, display_list: &DisplayList) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(viewport.width as u32);
    canvas.set_height(viewport.height as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


    for item in display_list {
        paint_item(&context, &item);
    }
}


pub fn paint_item(ctx: &web_sys::CanvasRenderingContext2d, item: &DisplayCommand) {
    match *item {
        DisplayCommand::SolidColor(rect, color) => {
            let fill_color = format!("rgba({},{},{},{})", color.r, color.g, color.b, color.a);
            ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(&fill_color));
            ctx.fill_rect(rect.x as f64, rect.y as f64, rect.width as f64, rect.height as f64);
        }
        DisplayCommand::Text(rect, ref text) => {
            ctx.set_font("10px serif");
            ctx.set_text_baseline("hanging");
            ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("blue"));
            ctx.fill_text(text, rect.x as f64, rect.y as f64);
        }
    }
}