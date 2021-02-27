use schema_gen::{Generator, Schema};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

fn main() {
    let file = File::open("./schemas/schemaorg-current-https.jsonld").unwrap();
    let reader = BufReader::new(file);

    let s = Schema::from_reader(reader).unwrap();

    let mut g = Generator::new(&s);
    g.generate();

    let out = File::create("../schema_org/src/schema.rs").unwrap();
    let mut writer = BufWriter::new(out);
    writer.write_all(g.to_string().as_bytes()).unwrap();
}
