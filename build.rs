use schema_gen::{Generation, Schema};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

fn main() {
    let file = File::open("./schemas/schemaorg-current-https.jsonld").unwrap();
    let reader = BufReader::new(file);

    let s = Schema::from_reader(reader).unwrap();
    let gen = Generation::new(&s);

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    fs::write(
        out.join("structs").with_extension("rs"),
        gen.generate_structs(),
    );

    fs::write(out.join("enums").with_extension("rs"), gen.generate_enums());

    fs::write(
        out.join("unions").with_extension("rs"),
        gen.generate_unions(),
    );
}
