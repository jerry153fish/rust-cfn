extern crate anyhow;
extern crate flate2;
extern crate heck;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use anyhow::Result;
use std::io::Read;

mod codegen;
mod model;

fn main() -> Result<()> {
    let data = download_gz_file("https://d1uauaxba7bl26.cloudfront.net/latest/gzip/CloudFormationResourceSpecification.json")?;
    let specification = serde_json::from_str::<model::Specification>(&data)
        .expect("failed to parse specification data");
    codegen::generate(specification, "../src/aws").expect("failed to generate output files");
    Ok(())
}

pub fn download_gz_file(url: &str) -> Result<String> {
    let resp = reqwest::blocking::get(url)?;
    let bytes = resp.bytes()?;
    let mut data = String::new();
    let mut gz = flate2::read::GzDecoder::new(&bytes[..]);
    gz.read_to_string(&mut data)?;
    Ok(data)
}
