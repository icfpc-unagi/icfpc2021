use crate::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

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
pub fn read_problem(s: &str) -> Result<JsValue, JsValue> {
	let mut prob: Input = serde_json::from_str(s).map_err(|e| JsValue::from(e.to_string()))?;
	prob.to_internal();
	JsValue::from_serde(&prob).map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen]
pub fn read_pose(s: &str) -> Result<JsValue, JsValue> {
	let mut pose: Output = serde_json::from_str(s).map_err(|e| JsValue::from(e.to_string()))?;
	pose.to_internal();
	JsValue::from_serde(&pose).map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen]
pub fn write_pose(j: JsValue) -> Result<String, JsValue> {
	let mut pose: Output = j.into_serde().map_err(|e| JsValue::from(e.to_string()))?;
	pose.to_external();
	serde_json::to_string(&pose).map_err(|e| JsValue::from(e.to_string()))
}

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
		ok_e.push(P::contains_s(
			&input.hole,
			(out.vertices[i], out.vertices[j]),
		));
	}
	JsValue::from_serde(&(ok_v, ok_e)).unwrap()
}

#[wasm_bindgen]
pub fn render_problem(s: &str) -> String {
	let prob = read_input_from_reader(s.as_bytes()).unwrap();

	let mut buf = Vec::new();
	paths::render_problem_svg(&prob, &mut buf).unwrap();

	String::from_utf8(buf).unwrap()
}

#[wasm_bindgen]
pub fn render_pose(problem: &str, pose: &str) -> String {
	let prob = read_input_from_reader(problem.as_bytes()).unwrap();
	let pose = read_output_from_reader(pose.as_bytes()).unwrap();

	let mut buf = Vec::new();
	paths::render_pose_svg(&prob, &pose, &mut buf).unwrap();

	String::from_utf8(buf).unwrap()
}

#[wasm_bindgen]
pub fn calculate_score(problem: &str, pose: &str) -> f64 {
	let prob = serde_json::from_str(problem).unwrap();
	let pose = serde_json::from_str(pose).unwrap();

	compute_score(&prob, &pose) as f64
}

#[wasm_bindgen]
pub fn morph(problem: &str, pose: &str, n: usize) -> String {
	let prob = serde_json::from_str(problem).unwrap();
	let pose = serde_json::from_str(pose).unwrap();

	let (pose, k) = ugougo::ugougo(&prob, &pose, n);
	console::log_1(&format!("success rate {}/{}", k, n).into());

	serde_json::to_string(&pose).unwrap()
}
