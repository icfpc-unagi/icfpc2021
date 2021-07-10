use icfpc2021::{*, util::*};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
      eprintln!("{} <problem.json> <pose.json>", args[0]);
      std::process::exit(1);
    }
	let input = read_input_from_file(&std::path::PathBuf::from(&args[1]));
	let output = read_output_from_file(&std::path::PathBuf::from(&args[2]));
  
    let score = compute_score(&input, &output);
    if score >= 1000000000 {
        println!("-1");
    } else {
        println!("{}", score);
    }
    Ok(())
}
