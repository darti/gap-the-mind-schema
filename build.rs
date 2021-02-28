use schema_gen::{Generator, Schema};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

fn main() {
    let file = File::open("./schemas/schemaorg-current-https.jsonld").unwrap();
    let reader = BufReader::new(file);

    let s = Schema::from_reader(reader).unwrap();

    let mut g = Generator::new(&s);
    g.generate();

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out = out.join("mod").with_extension("rs");
    let out = File::create(out).unwrap();

    let mut writer = BufWriter::new(out);
    writer.write_all(g.to_string().as_bytes()).unwrap();
}
