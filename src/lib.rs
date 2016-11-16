#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(feature = "serde_derive")]
include!("geojson.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/geojson.rs"));

pub mod error;

use std::result;
use std::fs::File;
use std::io::prelude::*;

pub type Result<T> = result::Result<T, error::Error>;

pub fn read_geojson(file: &str) -> Result<GeoJson> {
    let mut f = try!(File::open(file));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let res: GeoJson = try!(serde_json::from_str(&s));
    Ok(res)
}

pub fn test_reader(file: &str) {
    println!("Reading!");
    read_geojson(file).unwrap();
    println!("Done!");
}
