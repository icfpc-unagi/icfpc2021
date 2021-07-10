use std::io::Write;

use icfpc2021::*;

fn main() {
    for entry in glob::glob("./problems/*.json").unwrap() {
        let path = entry.unwrap();

        let path = path.to_str().unwrap();
        let filename = path
            .split("/")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .to_owned();
        let problem_id: i64 = filename.split('.').collect::<Vec<_>>()[0].parse().unwrap();

        let file = std::fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(file);
        let input: Input = serde_json::from_reader(reader).unwrap();
        // println!("{:?}", input.figure.edges);

        let out_path = format!("./tmp/{}.dot", problem_id);
        let out_file = std::fs::File::create(&out_path).unwrap();
        let mut writer = std::io::BufWriter::new(out_file);
        writer.write("graph{\n".as_bytes()).unwrap();
        writeln!(writer, "graph [dpi=96]").unwrap();
        let vs = input.figure.vertices;
        for i in 0..vs.len() {
            writer
                .write(format!("{} [height=0, width=0, shape=point]\n", i).as_bytes())
                .unwrap();
        }
        for (a, b) in input.figure.edges {
            let d2 = (vs[a] - vs[b]).abs2();
            let d = (d2 as f64).sqrt();
            writer
                .write(format!("{} -- {} [len={}]\n", a, b, d).as_bytes())
                .unwrap();
        }
        writer.write("}\n".as_bytes()).unwrap();
    }
}
