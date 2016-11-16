#![feature(test)]

extern crate test;
extern crate kommuner;

use test::Bencher;

#[bench]
fn bench_reader(b: &mut Bencher) {
    b.iter(|| kommuner::read_geojson("kommuner.geojson").unwrap());
}

