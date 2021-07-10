use crate::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen(start)]
// pub fn main() -> Result<(), JsValue> {
// 	let window = web_sys::window().unwrap();
// 	let document = window.document().unwrap();
// 	let body = document.body().unwrap();

// 	let container = document.create_element("div")?;
// 	container.set_attribute("id", "_container")?;
// 	body.append_child(&container)?;

// 	Ok(())
// }

#[wasm_bindgen]
pub fn check_solution1(input: JsValue, out: JsValue) -> JsValue {
	let input: Input = input.into_serde().unwrap();
	let out: Output = out.into_serde().unwrap();
	let mut ok_v = vec![];
	for &p in &out.vertices {
		ok_v.push(P::contains_p(&input.hole, p) >= 0);
	}
	let mut ok_e = vec![];
	for &(i, j) in &input.figure.edges {
		ok_e.push(P::contains_s(&input.hole, (out.vertices[i], out.vertices[j])));
	}
	JsValue::from_serde(&(ok_v, ok_e)).unwrap()
}

#[wasm_bindgen]
pub fn render_problem(s: &str) -> Result<String, JsValue> {
	let prob = serde_json::from_str::<Input>(s).unwrap();

	let mut buf = Vec::new();
	paths::render_problem_svg(&prob, &mut buf).map_err(|e| JsValue::from(e.to_string()))?;

	Ok(String::from_utf8(buf).map_err(|e| JsValue::from(e.to_string()))?)
}

#[wasm_bindgen]
pub fn render_pose(problem: &str, pose: &str) -> Result<String, JsValue> {
	let prob = read_input_from_reader(problem.as_bytes()).map_err(|e| JsValue::from(e.to_string()))?;
	let pose = serde_json::from_str(pose).map_err(|e| JsValue::from(e.to_string()))?;

	let mut buf = Vec::new();
	paths::render_pose_svg(&prob, &pose, &mut buf).map_err(|e| JsValue::from(e.to_string()))?;

	Ok(String::from_utf8(buf).map_err(|e| JsValue::from(e.to_string()))?)
}

#[wasm_bindgen]
pub fn calculate_score(problem: &str, pose: &str) -> Result<f64, JsValue> {
	let prob = read_input_from_reader(problem.as_bytes()).map_err(|e| JsValue::from(e.to_string()))?;
	let pose = serde_json::from_str(pose).map_err(|e| JsValue::from(e.to_string()))?;

	Ok(compute_score(&prob, &pose) as f64)
}
