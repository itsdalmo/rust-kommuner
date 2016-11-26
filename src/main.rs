extern crate kommuner;
extern crate clap;
extern crate csv;

use clap::{Arg, App};

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
                          .arg(Arg::with_name("parallel")
                               .short("p")
                               .long("parallel")
                               .value_name("BOOLEAN")
                               .help("Enable/disable parallel execution.")
                               .takes_value(true)
                               .required(false))
                          .get_matches();

    // Gather all input options
    let geojson  = matches.value_of("geojson").unwrap();
    let input    = matches.value_of("input").unwrap();
    let output   = matches.value_of("output").unwrap_or("out.csv");

    // Read input and geojson
    let counties = match kommuner::read_geojson(&geojson) {
        Ok(v)  => kommuner::Counties::new(&v),
        Err(e) => panic!("Failed to read geojson: {:?}", e),

    };
    let records  = match kommuner::read_csv(&input) {
        Ok(v)  => v,
        Err(e) => panic!("Failed to read input: {:?}", e),

    };

    // Initialize a CSV writer
    let mut file = match csv::Writer::from_file(&output) {
        Ok(v)  => v,
        Err(e) => panic!("Failed to create ouput: {:?}", e),
    };

    // Loop over records - lookup and write to file
    for record in records.iter() {
        let c = counties.lookup(&record.position()).unwrap_or("".to_string());
        let r = (record.testid, c);
        match file.encode(r) {
            Ok(_)  => {},
            Err(e) => {
                println!("Failed to write record: {:?}", e);
            },
        }
    }
}
