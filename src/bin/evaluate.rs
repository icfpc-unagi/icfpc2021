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
  
    let mut obtained_bonus: Vec<_> = Vec::new();
    for ref bonus in &input.bonuses {
        for p in &output.vertices {
            if bonus.position == *p {
                obtained_bonus.push(bonus.to_owned());
            }
        }
    }

    let evaluation = evaluate(&input, &output);

	println!("{}", serde_json::to_string(&evaluation).unwrap());
    Ok(())
}
