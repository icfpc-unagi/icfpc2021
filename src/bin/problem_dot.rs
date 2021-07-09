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
        for (x, y) in input.figure.edges {
            writer
                .write(format!("{} -- {}\n", x, y).as_bytes())
                .unwrap();
        }
        writer.write("}\n".as_bytes()).unwrap();
    }
}
