# Rust GEOJSON test

Read geojson and stuff.

## Usage

```bash
kommuner --input ./kommuner.csv --geojson ./kommuner.geojson --output test.csv
```

## Benches

Where `lookup_all` tests use 100 records at either the start or end of the geojson.

```bash
test bench_csv_read             ... bench:      38,971 ns/iter (+/- 1,760)
test bench_geojson_read         ... bench:     486,793 ns/iter (+/- 87,166)
test bench_lookup_all_first     ... bench:     650,267 ns/iter (+/- 20,073)
test bench_lookup_all_last      ... bench: 450,096,608 ns/iter (+/- 7,648,733)
test bench_lookup_first         ... bench:       6,578 ns/iter (+/- 443)
test bench_lookup_last          ... bench:   4,508,952 ns/iter (+/- 553,645)
test bench_par_lookup_all_first ... bench:     352,237 ns/iter (+/- 43,922)
test bench_par_lookup_all_last  ... bench: 183,507,724 ns/iter (+/- 8,548,400)
```
