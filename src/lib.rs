use rand::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
) {
    let [top, left, right] = points;

    let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.fill();
    // context.stroke();
}

fn _draw_spot(context: &web_sys::CanvasRenderingContext2d, point: (f64, f64)) {
    context.move_to(point.0, point.1);
    context.fill_rect(point.0 - 4.0, point.1 - 4.0, 8.0, 8.0);
}

fn draw_sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    depth: u8,
    points: [(f64, f64); 3],
) {
    let [top, left, right] = points;

    let quarter = (right.0 - left.0) / 4.0;
    let half = quarter * 2.0;

    let mid_left = (left.0 + quarter, top.1 + half);
    let mid_right = (right.0 - quarter, top.1 + half);
    let mid_bottom = (left.0 + half, left.1);

    let mut rng = thread_rng();
    let color = (
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );
    println!("color: {:?}", color);
    draw_triangle(&context, [mid_left, left, mid_bottom], color);
    let color = (
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );
    draw_triangle(&context, [mid_right, mid_bottom, right], color);
    let color = (
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );
    draw_triangle(&context, [top, mid_left, mid_right], color);

    if depth > 0 {
        draw_sierpinski(&context, depth - 1, [top, mid_left, mid_right]);
        draw_sierpinski(&context, depth - 1, [mid_left, left, mid_bottom]);
        draw_sierpinski(&context, depth - 1, [mid_right, mid_bottom, right]);
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let top = (300.0, 0.0);
    let left = (0.0, 600.0);
    let right = (600.0, 600.0);
    let depth: u8 = 4;

    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("walk-the-dog"));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    draw_sierpinski(&context, depth, [top, left, right]);

    Ok(())
}
