use icfpc2021::*;

const USAGE: &'static str = "
Running your program against multiple inputs.

Usage:
  tester [-m <msg>] [-p <num-threads>] [-i <input_dir>] <command>
  tester (-h | --help)

Options:
  -h, --help  Show this screen.
  -m <msg>    Use the given <msg> as the name of this execution.
  -p <num>    Specify the number of parallel executions (default: 1).
  -i <dir>    Specify the input directory (default: in).
";

#[derive(Debug, serde::Deserialize)]
struct Args {
	arg_command: String,
	flag_m: Option<String>,
	flag_i: Option<String>,
	flag_p: i32
}

fn exec(cmd: &str, input: &std::path::PathBuf) -> (i64, f64, String) {
	let name = input.file_name().unwrap();
	let output = std::path::Path::new("tmp/out").join(name);
	let mut cmd_args = cmd.split_whitespace().chain([input.file_stem().unwrap().to_str().unwrap()]);
	let ms = {
		let input_file = std::fs::File::open(input).unwrap();
		let output_file = std::fs::File::create(&output).unwrap();
		let err_file = std::fs::File::create(std::path::Path::new("tmp/err").join(name)).unwrap();
		let stime = std::time::SystemTime::now();
		std::process::Command::new(cmd_args.next().unwrap())
		// std::process::Command::new("../tools/target/release/tester") // for interactive problem
			.args(cmd_args)
			.stdin(std::process::Stdio::from(input_file))
			.stdout(std::process::Stdio::from(output_file))
			.stderr(std::process::Stdio::from(err_file))
			.output().unwrap_or_else(|_| {
				eprintln!("failed to execute command {}", cmd);
				std::process::exit(1);
			});
		let t = std::time::SystemTime::now().duration_since(stime).unwrap();
		t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
	};
	let input = read_input_from_file(&input);
	let output = read_output_from_file(&output);
	let score = compute_score(&input, &output);
	let mut svg = vec![];
	if output.vertices.len() > 0 {
		icfpc2021::paths::render_pose_svg(&input, &output, &mut svg).unwrap();
	}
	let mut html = format!("<h2>{}</h2>", name.to_string_lossy().trim_end_matches(".txt"));
	html += &format!("<p>Score = {}</p>", score);
	if svg.len() > 0 {
		html += &String::from_utf8(svg).unwrap();
	}
	(score, ms, html)
}

fn read_list_of_inputs(input_dir: &str) -> Vec<std::path::PathBuf> {
	let mut inputs = std::path::Path::new(input_dir).read_dir().unwrap_or_else(|_| {
		eprintln!("no such directory: {}", input_dir); std::process::exit(1)
	}).map(|f| f.unwrap().path()).filter(|f| f.extension().unwrap_or_default() == "json").collect::<Vec<_>>();
	inputs.sort_by_key(|a| a.file_stem().unwrap().to_str().unwrap().parse::<i64>().unwrap());
	inputs
}

fn main() {
	let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.help(true).deserialize()).unwrap_or_else(|e| e.exit());
	let msg = args.flag_m.clone().unwrap_or(chrono::Local::now().format("%m-%d %H:%M").to_string());
	let num_threads = args.flag_p.max(1);
	let all_inputs = read_list_of_inputs(&args.flag_i.clone().unwrap_or("problems".to_owned()));
	for dir in &["tmp", "tmp/out", "tmp/err"] {
		if !std::path::Path::new(dir).exists() {
			std::fs::create_dir(dir).unwrap();
		}
	}
	let next_id = std::sync::Arc::new(std::sync::Mutex::new(0));
	let results = std::sync::Arc::new(std::sync::Mutex::new(vec![(0, 0.0, String::new()); all_inputs.len()]));
	let bar = std::sync::Arc::new(indicatif::ProgressBar::new(all_inputs.len() as u64));
	let threads = (0..num_threads).map(|_| {
		let all_inputs = all_inputs.clone();
		let next_id = next_id.clone();
		let results = results.clone();
		let bar = bar.clone();
		let cmd = args.arg_command.clone();
		std::thread::spawn(move || {
			loop {
				let id = {
					let mut next_id = next_id.lock().unwrap();
					let id = *next_id;
					*next_id += 1;
					if id >= all_inputs.len() {
						return;
					}
					id
				};
				let ret = exec(&cmd, &all_inputs[id]);
				let mut results = results.lock().unwrap();
				results[id] = ret;
				bar.inc(1);
			}
		})
	}).collect::<Vec<_>>();
	for t in threads {
		t.join().unwrap();
	}
	bar.finish();
	let mut total_score = 0;
	let mut max_time = 0.0;
	let mut max_input = all_inputs[0].clone();
	let mut html = r#"<html><head><meta content="text/html;charset=utf-8" http-equiv="Content-Type"/></head><body>"#.to_owned();
	for i in 0..all_inputs.len() {
		let (score, time, ref h) = results.lock().unwrap()[i];
		total_score += score;
		if max_time < time {
			max_time = time;
			max_input = all_inputs[i].clone();
		}
		html += &h;
	}
	html += "</body></html>";
	print!("\"{}\"\t{}", msg, total_score);
	for &(score, _, _) in results.lock().unwrap().iter() {
		print!("\t{}", score);
	}
	println!();
	eprintln!("total_score = {}", total_score);
	eprintln!("max_time = {:.3} ({})", max_time, max_input.to_string_lossy());
	std::fs::write("tmp/vis_all.html", html).unwrap();
}
