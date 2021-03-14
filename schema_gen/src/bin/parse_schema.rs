use schema_gen::Schema;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("../schemas/schemaorg-current-https.jsonld").unwrap();
    let reader = BufReader::new(file);

    let s = Schema::from_reader(reader).unwrap();
}
