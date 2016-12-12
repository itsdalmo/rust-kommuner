[![OSX/Linux](https://travis-ci.org/itsdalmo/rust-kommuner.svg?branch=master)](https://travis-ci.org/itsdalmo/rust-kommuner)
[![Windows](https://ci.appveyor.com/api/projects/status/adismvrqb931okqd/branch/master?svg=true)](https://ci.appveyor.com/project/itsdalmo/rust-kommuner/branch/master)

# Rust GEOJSON test

Read geojson and stuff.

## Usage

```bash
kommuner --input ./kommuner.csv --geojson ./kommuner.geojson --output test.csv
```

## Benches

Where `lookup_all` tests use 100 records at either the start or end of the geojson.

```bash
test bench_csv_read         ... bench:      39,266 ns/iter (+/- 867)
test bench_geojson_read     ... bench:     496,560 ns/iter (+/- 20,151)
test bench_lookup_all_first ... bench:     355,065 ns/iter (+/- 57,129)
test bench_lookup_all_last  ... bench: 175,942,486 ns/iter (+/- 8,378,414)
test bench_lookup_first     ... bench:       7,137 ns/iter (+/- 1,317)
test bench_lookup_last      ... bench:   4,530,849 ns/iter (+/- 377,375)
```

