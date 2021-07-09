use crate::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    svg.set_attribute("id", "_svg")?;
    body.append_child(&svg)?;

    Ok(())
}

#[wasm_bindgen]
pub fn render_problem(s: &str) -> Result<(), JsValue> {
    let prob = serde_json::from_str::<Input>(s).unwrap();
    let hole_polygon = paths::polygon(&prob.hole);
    let figure_path = paths::path(&prob.figure.edges, &prob.figure.vertices);

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let svg = document.get_element_by_id("_svg").unwrap();

    let polygon = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")?;
    polygon.set_attribute("fill", "grey")?;
    polygon.set_attribute("points", &hole_polygon)?;
    svg.append_child(&polygon)?;

    let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path.set_attribute("stroke", "red")?;
    path.set_attribute("d", &figure_path)?;
    svg.append_child(&path)?;

    Ok(())
}
