use std::{
    env, fs,
    io::{self, Write},
};

use noodles::{
    bcf,
    vcf::{
        self,
        record::info::field::{Key, Value},
    },
};

fn get_allele_count(
    record: io::Result<bcf::Record>,
    header: &vcf::Header,
    string_map: &bcf::header::StringMap,
) -> io::Result<Option<i32>> {
    Ok(record?
        .info()
        .get(header, string_map, &Key::TotalAlleleCount)
        .transpose()?
        .and_then(|field| match field.value() {
            Some(Value::Integer(allele_count)) => Some(*allele_count),
            _ => None,
        }))
}

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("missing BCF path");

    let mut reader = fs::File::open(path)
        .map(io::BufReader::new)
        .map(bcf::Reader::new)?;
    reader.read_file_format()?;

    let raw_header = reader.read_header()?;
    let header = raw_header.parse().expect("error parsing header");
    let string_map = raw_header.parse().expect("error parsing header");

    let allele_counts = reader
        .records()
        .map(|record| get_allele_count(record, &header, &string_map))
        .collect::<io::Result<Option<Vec<i32>>>>()?
        .expect("missing or unexpected AN field");

    let total = allele_counts.iter().sum::<i32>();
    let n = allele_counts.len();
    writeln!(io::stderr(), "{}", total as f64 / n as f64)
}
