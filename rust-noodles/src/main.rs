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
    let path = &"../1kg.chr1.subset.bcf";

    let mut bcf = File::open(path).map(bcf::Reader::new).expect("couldn't open bcf");
    let mut stderr = std::io::stderr();
    bcf.read_file_format().expect("error reading file format");
    let mut raw_hdr = bcf.read_header().expect("invalid header");
    let header = raw_hdr.parse().expect("error parsing header");
    let string_map = raw_hdr.parse().expect("error parsing header");
    let key = Key::Other(std::string::String::from("AN"), Number::Count(1), Type::Integer, std::string::String::from(""));

    for r in bcf.records() {
        let bcf_rec = r.expect("error getting record");
        let mut rec = bcf_rec.try_into_vcf_record(&header, &string_map).expect("error converting to vcf record");
        let dp = rec.info().get(&key)
            .map(|field| field.value())
            .map(|value| match value {
                Value::Integer(n) => *n,
                _ => -1,
            });
    }
    writeln!(stderr, "made it");
}
