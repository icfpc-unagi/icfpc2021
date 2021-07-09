use icfpc2021::*;

fn main() {
	let input = read_input();
	dbg!(&input);
	write_output(&Output { vertices: input.figure.vertices.clone() })
}
