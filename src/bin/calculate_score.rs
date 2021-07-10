use icfpc2021::{*, util::*};
use std::fs::File;

fn compute_score(input: &Input, out: &Output) -> i64 {
	let mut score = 0;
    if input.figure.vertices.len() != out.vertices.len() {
        return -1;
    }
	for &p in &input.hole {
		let mut min = i64::max_value();
		for q in &out.vertices {
			min.setmin((p - q.to_owned()).abs2());
		}
		score += min;
	}
	score
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
      eprintln!("{} <problem.json> <pose.json>", args[0]);
      std::process::exit(1);
    }
    let prob = serde_json::from_reader(File::open(&args[1])?)?;
    let pose = serde_json::from_reader(File::open(&args[2])?)?;

  
    let score = compute_score(&prob, &pose);
    println!("{}", score);
    Ok(())
}
