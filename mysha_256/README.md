# mysha_256
`mysha_256` is a cli tool using sha-256 to generate digest.

# Usage:
```
mysha_256 -i "abc"
ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
```
# Benchmark
```
Gnuplot not found, using plotters backend
mysha-256: small        time:   [51.829 us 53.180 us 54.617 us]
                        change: [+12.602% +17.008% +22.792%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

Benchmarking mysha-256: large: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 10.8s or reduce sample count to 40.
mysha-256: large        time:   [1.6552 ms 1.6761 ms 1.7029 ms]                                     
                        change: [+4.7782% +9.1207% +14.121%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

```