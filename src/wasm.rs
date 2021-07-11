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
pub fn score_or_message(prob: JsValue, pose: JsValue) -> String {
	let prob: Input = prob.into_serde().unwrap();
	let pose: Output = pose.into_serde().unwrap();
	match compute_score_or_err(&prob, &pose) {
		Ok(score) => format!("score: {}", score),
		Err(msg) => format!("message: {}", msg),
	}
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

// zenkan
#[wasm_bindgen]
pub fn all_pair_abs2_ub(prob: JsValue) -> js_sys::Uint32Array {
	let prob: Input = prob.into_serde().unwrap();
	let dist = all_pair_dist_ub(&prob);
	let abs2_ub_flat: Vec<_> = dist.into_iter().flatten()
		.map(|d| (d * d + 0.1) as u32)
		.collect();
	abs2_ub_flat[..].into()
}

#[wasm_bindgen]
pub fn all_pair_abs2(pose: JsValue) -> js_sys::Uint32Array {
	let pose: Output = pose.into_serde().unwrap();
	let vs = pose.vertices;
	let mut ret = vec![];
	for &v0 in &vs {
		for &v1 in &vs {
			let d = (v0 - v1).abs2();
			ret.push(d as u32);
		}
	}
	ret[..].into()
}

#[wasm_bindgen]
pub struct AllPairDist {
	n: usize,
	dist_flat: js_sys::Float64Array,
}

// impl Index<(usize, usize)> for AllPairDist {
// 	type Output = f64;
// 	fn index(&self, (i, j): (usize, usize)) -> &f64 {
// 		let k = i * self.n + j;
// 		&self.dist_flat.get_index(k as u32)
// 	}
// }

// #[wasm_bindgen]
impl AllPairDist {
	#[deprecated]
	pub fn from_problem(prob: JsValue) -> Self {
		let prob: Input = prob.into_serde().unwrap();
		let dist = all_pair_dist_ub(&prob);
		let n = dist.len();
		let dist_flat: Vec<_> = dist.into_iter().flatten().collect();
		Self {
			n,
			dist_flat: dist_flat[..].into(),
		}
	}

	#[deprecated]
	pub fn test_pose(&self, pose: JsValue) -> js_sys::Int16Array {
		let pose: Output = pose.into_serde().unwrap();
		let vs = pose.vertices;
		let n = self.n;
		let mut ret = vec![];
		for i in 0..n {
			for j in i+1..n {
				let d = ((vs[i] - vs[j]).abs2() as f64).sqrt();
				let k = i * n + j;
				if self.dist_flat.get_index(k as u32) < d - 1e-4 {
					ret.push(i as i16);
					ret.push(j as i16);
				}
			}
		}
		ret[..].into()
	}
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
	let prob = read_input_from_reader(problem.as_bytes()).unwrap();
	let pose = read_output_from_reader(pose.as_bytes()).unwrap();

	compute_score(&prob, &pose) as f64
}

#[wasm_bindgen]
pub fn morph(problem: &str, pose: &str, n: usize) -> String {
	let prob = read_input_from_reader(problem.as_bytes()).unwrap();
	let pose = read_output_from_reader(pose.as_bytes()).unwrap();

	let (pose, k) = ugougo::ugougo(&prob, &pose, n);
	console::log_1(&format!("success rate {}/{}", k, n).into());

	let mut buf = Vec::new();
	write_output_to_writer(&pose, &mut buf);
	String::from_utf8(buf).unwrap()
}
