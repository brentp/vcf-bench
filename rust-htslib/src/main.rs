extern crate rust_htslib;
use crate::rust_htslib::bcf::{Read, Reader};
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &*args[1];
    let mut bcf = Reader::from_path(path).expect("couldn't open input vcf");

    let mut li: Vec<i32> = Vec::new();

    for r in bcf.records() {
        let rec = r.expect("error getting record");

        if let Some(v) = rec.info(b"AN").integer().expect("error acessing info") {
            li.push(v[0]);
        }
    }
    let mut stderr = std::io::stderr();
    let s: i32 = li.iter().sum();

    writeln!(stderr, "sum: {}, avg:{}", s, s as f64 / li.len() as f64)
        .expect("error writing to stderr");
}
