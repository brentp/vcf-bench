use noodles_bcf::{
    self as bcf, 
};
use noodles_vcf::{
    record::{info::{field::{Key, Value}}},
    header::{Number, info::Type},
};
//use bcf::record::{value::Value};
use std::fs::{File};
use std::io::Write;


fn main() {
    // NOTE: must use a modified BCF with AD,Number=. changed to AD,Number=A due to strict checking
    // of VCF
    let args: Vec<String> = std::env::args().collect();
    let path = &*args[1];


    let mut bcf = File::open(path).map(bcf::Reader::new).expect("couldn't open bcf");
    let mut stderr = std::io::stderr();
    bcf.read_file_format().expect("error reading file format");
    let mut raw_hdr = bcf.read_header().expect("invalid header");
    let header = raw_hdr.parse().expect("error parsing header");
    let string_map = raw_hdr.parse().expect("error parsing header");

    //let key = Key::Other(std::string::String::from("AN"), Number::Count(1), Type::Integer, std::string::String::from(""));
    let key = Key::TotalAlleleCount;
    let mut li: Vec<i64> = Vec::new();

    for r in bcf.records() {
        let bcf_rec = r.expect("error getting record");
        // NOTE: get an error here
        let mut rec = bcf_rec.try_into_vcf_record(&header, &string_map).expect("error converting to vcf record");
        let dp = rec.info().get(&key)
            .map(|field| field.value())
            .map(|value| match value {
                Value::Integer(n) => *n,
                _ => -1,
            }).unwrap();
        li.push(dp as i64);
    }

     let s: i64 = li.iter().sum();
     writeln!(stderr, "{}", s as f64 / li.len() as f64)
        .expect("error writing to stderr");


}
