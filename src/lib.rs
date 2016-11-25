#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate geo;
extern crate rustc_serialize;

#[cfg(feature = "serde_derive")]
include!("geojson.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/geojson.rs"));

pub mod error;

use geo::{Point, Polygon, LineString};
use geo::algorithm::contains::Contains;
use std::result;
use std::fs::File;
use std::io::prelude::*;

pub type Result<T> = result::Result<T, error::Error>;

#[derive(Debug, RustcDecodable)]
pub struct Record {
    pub index: i32,
    pub testid: i64,
    pub longitude: f64,
    pub latitude: f64,
}

impl Record {
    /// Returns a point with latitude and longitude (in that order).
    pub fn position(&self) -> geo::Point<f64> {
        Point::new(self.latitude, self.longitude)
    }
}

#[test]
fn test_record() {
    let record = Record { index: 0, testid: 1000, longitude: 59.210860, latitude: 8.009823 };
    let point  = Point::new(8.009823, 59.210860);
    assert_eq!(record.position(), point);
}

#[derive(Debug)]
pub struct County {
    name: String,
    poly: geo::Polygon<f64>,
}

impl County {
    /// Create a new County object from a Feature.
    pub fn new(feat: &Feature) -> County {
        let mut points = vec![];

        // Extract the first/only array of coordinates (external borders)
        let coords = feat.geometry.coordinates[0].clone();
        for coord in coords {
            let p = Point::new(coord[1].clone(), coord[0].clone());
            points.push(p);
        }

        County {
            name: feat.properties.navn.clone(),
            poly: Polygon::new(LineString(points), vec![])
        }
    }
    /// Checks whether a point is in a county.
    /// Returns the name of the county.
    pub fn lookup(&self, p: &geo::Point<f64>) -> Option<String> {
        match self.poly.contains(p) {
            true  => Some(self.name.clone()),
            false => None,
        }
    }
}

#[test]
fn test_county() {
    let json = read_geojson("./examples/data/sample.geojson").unwrap();
    let res  = County::new(&json.features[0]);
    assert_eq!(res.name, "Osterøy");
    assert_eq!(res.lookup(&Point::new(60.524035, 5.552604)).unwrap(), "Osterøy");
}

#[derive(Debug)]
pub struct Counties {
    list: Vec<County>,
}

impl Counties {
    /// Create a new Counties object from a GeoJson.
    pub fn new(json: &GeoJson) -> Counties {
        let mut counties: Vec<County> = vec![];
        for county in json.features.iter() {
            counties.push(County::new(county));
        }
        Counties {
            list: counties
        }
    }
    /// Lookup the county (if any) for a given point.
    /// Returns the name of the county.
    pub fn lookup(&self, p: &geo::Point<f64>) -> Option<String> {
        for kommune in self.list.iter() {
            match kommune.lookup(p) {
                Some(v) => {
                    return Some(v);
                },
                None    => {},
            }
        }
        // No county -> None
        None
    }
    /// Lookup multiple locations and return their county name.
    pub fn lookup_all(&self, p: &Vec<geo::Point<f64>>) -> Vec<Option<String>> {
        let mut res: Vec<Option<String>> = vec![];
        for point in p.iter() {
            res.push(self.lookup(&point));
        }
        res
    }
}

#[test]
fn test_counties() {
    let json = read_geojson("./examples/data/sample.geojson").unwrap();
    let res  = Counties::new(&json);
    assert_eq!(res.lookup(&Point::new(60.524035, 5.552604)).unwrap(), "Osterøy");
}

/// Read the 'kommuner.geojson' file. The structure is predefined and should
/// not be changed.
///
/// # Arguments
/// `file`: A borrowed string with the path to the JSON to be read.
///
/// # Returns
/// A `Result` a GeoJson struct.
///
pub fn read_geojson(file: &str) -> Result<GeoJson> {
    let mut f = try!(File::open(file));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let res: GeoJson = try!(serde_json::from_str(&s));
    Ok(res)
}

#[test]
fn test_read_geojson() {
    let res = read_geojson("./examples/data/sample.geojson");
    match res {
        Ok(v)    => {
            assert_eq!(v.kind, "FeatureCollection");
            assert_eq!(v.features[0].kind, "Feature");
        },
        Err(err) => panic!("Error: {:?}", err),
    }
}


/// Read a test records containing index, testid, longitude and latitude.
///
/// # Arguments
/// `file`: A borrowed string with the path to the CSV file to be read.
///
/// # Returns
/// A `Result` with a vector of records, where each record is a line in the CSV.
///
pub fn read_csv(file: &str) -> Result<Vec<Record>> {
    let mut csv = try!(csv::Reader::from_file(&file));
    let mut res: Vec<Record> = vec![];
    for line in csv.decode() {
        let record: Record = try!(line);
        res.push(record);
    }
    Ok(res)
}

#[test]
fn test_read_csv() {
    let res = read_csv("./examples/data/sample.csv");
    match res {
        Ok(v)    => {
            // TODO: This is the 2nd row. Why?
            assert_eq!(v[0].testid, 2258426552);
            assert_eq!(v[0].longitude, 11.0531);
            assert_eq!(v[0].latitude, 59.2761);
        },
        Err(err) => panic!("Error: {:?}", err),
    }
}

pub fn test_read_all() {
    let csv = match read_csv("./examples/data/kommuner.csv") {
        Ok(v)  => {
            println!("Read {} csv records!", v.len()); v
        },
        Err(e) => panic!("{:?}", e),
    };
    let json = match read_geojson("./examples/data/kommuner.geojson") {
        Ok(v)  => {
            println!("Finished reading json!"); v
        },
        Err(e) => panic!("{:?}", e),
    };

    let kommuner = Counties::new(&json);

    for record in csv.iter() {
        match kommuner.lookup(&record.position()) {
            Some(v) => {
                println!("{:?}", v);
            },
            None    => println!("No kommune found!"),
        }

    }
}

