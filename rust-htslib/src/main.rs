extern crate rust_htslib;
use crate::rust_htslib::bcf::{Reader, Read};
use std::io::Write;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &*args[1];
    let mut bcf = Reader::from_path(path).expect("couldn't open input vcf");

    let mut li: Vec<i64> = Vec::new();

    for r in bcf.records() {
        let rec = r.expect("error getting record");

         match rec.info(b"AN").integer().expect("error acessing info") {
                 Some(v) => li.push(v[0] as i64),
                 None => continue,
         }

    }
    let mut stderr = std::io::stderr();
    let s:i64 = li.iter().sum();

     writeln!(stderr, "sum: {}, avg:{}", s, s / li.len() as i64).expect("error writing to stderr");



    
}
