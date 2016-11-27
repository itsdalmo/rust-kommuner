#![feature(test)]

extern crate test;
extern crate kommuner;
extern crate geo;

use test::Bencher;

#[bench]
fn bench_geojson_read(b: &mut Bencher) {
    b.iter(|| kommuner::read_geojson("./examples/data/sample.geojson").unwrap());
}

#[bench]
fn bench_csv_read(b: &mut Bencher) {
    b.iter(|| kommuner::read_csv("./examples/data/sample.csv").unwrap());
}

#[bench]
fn bench_lookup_first(b: &mut Bencher) {
    // Looks up a location in the first county: Osterøy
    let json = kommuner::read_geojson("./examples/data/kommuner.geojson").unwrap();
    let counties = kommuner::Counties::new(&json);
    b.iter(|| counties.lookup(&geo::Point::new(60.524035, 5.552604)));
}

#[bench]
fn bench_lookup_last(b: &mut Bencher) {
    // Looks up a location in the last county: Horten
    let json = kommuner::read_geojson("./examples/data/kommuner.geojson").unwrap();
    let counties = kommuner::Counties::new(&json);
    b.iter(|| counties.lookup(&geo::Point::new(59.419622, 10.466373)));
}

#[bench]
fn bench_lookup_all_first(b: &mut Bencher) {
    let json = kommuner::read_geojson("./examples/data/kommuner.geojson").unwrap();
    let counties = kommuner::Counties::new(&json);

    // Grow a vector with 'last entries' only
    let mut res = vec![];
    let p = geo::Point::new(60.524035, 5.552604);
    for _ in 1..100 {
        res.push(p);
    }

    b.iter(|| counties.lookup_all(&res));

}

#[bench]
fn bench_lookup_all_last(b: &mut Bencher) {
    let json = kommuner::read_geojson("./examples/data/kommuner.geojson").unwrap();
    let counties = kommuner::Counties::new(&json);

    // Grow a vector with 'last entries' only
    let mut res = vec![];
    let p = geo::Point::new(59.419622, 10.466373);
    for _ in 1..100 {
        res.push(p);
    }

    b.iter(|| counties.lookup_all(&res));

}

