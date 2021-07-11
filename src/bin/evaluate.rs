use icfpc2021::{*, util::*};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
      eprintln!("{} <problem.json> <pose.json>", args[0]);
      std::process::exit(1);
    }
	let mut input = read_input_from_file(&std::path::PathBuf::from(&args[1]));
	let mut output = read_output_from_file(&std::path::PathBuf::from(&args[2]));

    // Hack.
    input.internal = Some(InputInternal{reversed_hole: false});

    input.to_external();
    output.to_external();
  
    let evaluation = evaluate(&input, &output);

	println!("{}", serde_json::to_string(&evaluation).unwrap());
    Ok(())
}
