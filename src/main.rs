extern crate kommuner;
extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Rust geolookup")
                          .version("0.1.0")
                          .author("Kristian D. Olsen <kristian@doingit.no>")
                          .about("Adds county name to CSV with locations.")
                          .arg(Arg::with_name("geojson")
                               .short("g")
                               .long("geojson")
                               .value_name("PATH")
                               .help("Path to geojson for Counties.")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("input")
                               .short("i")
                               .long("input")
                               .value_name("PATH")
                               .help("Path to input CSV with locations.")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("output")
                               .short("o")
                               .long("output")
                               .value_name("PATH")
                               .help("Path to the output file.")
                               .takes_value(true)
                               .required(false))
                          .get_matches();

    // Gather all input options
    let geojson = matches.value_of("geojson").unwrap();
    let input   = matches.value_of("input").unwrap();
    let output  = matches.value_of("output").unwrap_or("out.csv");

    // Read input and geojson
    let counties = match kommuner::read_geojson(&geojson) {
        Ok(v)  => kommuner::Counties::new(&v),
        Err(e) => panic!("Failed to read geojson: {:?}", e),

    };
    let records  = match kommuner::read_csv(&input) {
        Ok(v)  => v,
        Err(e) => panic!("Failed to read input: {:?}", e),

    };

    // Loop over records - lookup and write to file
    for record in records.iter() {
        let res = counties.lookup(&record.position());
        println!("{:?}", res.unwrap());
    }

}
