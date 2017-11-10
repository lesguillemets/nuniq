use std::io;
use std::io::{Write, BufWriter, BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let reader = io::stdin();
    let writer = io::stdout();
    let reader = BufReader::new(reader.lock());
    let mut writer = BufWriter::new(writer.lock());
    let mut deja_vu = HashSet::new();
    for l in reader.lines() {
        let ln = l.unwrap();
        if deja_vu.insert(ln.clone()) {
            writer.write(ln.as_bytes());
            writer.write(b"\n");
        }

    }
}
