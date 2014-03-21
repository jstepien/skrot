# Benchmarking Skrót

Let's see how Skrót performs in some synthetic microbenchmarks.
To run code in this directory you'll need Ruby and [Bundler][b].
[R][r] will be used to obtain some summary statistics.

    bundle && make

All results below were obtained on a glacial age Core 2 Duo running at 2.1GHz.
On an i7 CPU encoding and decoding is nearly twice faster.

## The people benchmark

`generate_people.rb` uses [Faker][f] to generate JSON-encoded maps with some
artificial data such as names, phone numbers, e-mail addresses and Unix
timestamps. A pretty-printed example looks as follows.

```json
{
  "name":      "Kevin Haley",
  "email":     "jenifer@example.org",
  "phone":     "(834)357-9121 x724",
  "ip":        "178.225.161.156",
  "created_at": 1389645721
}
```

Most of those maps are around 135B long. Standard compression
tools, e.g. `gzip`, `bzip2` and `xz`, make them even longer.
LZMA-based Skrót encoding achieves space savings of over 55%.
LZ4-based encoding achieves 40% savings and much shorter processing time.
Following tables summarise length of
samples before and after the encoding as well as their ratio and
time of encoding and decoding in milliseconds.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :117.0   Min.   :43.00   Min.   :0.338   Min.   :18.97   Min.   :0.709
1st Qu.:128.0   1st Qu.:50.00   1st Qu.:0.388   1st Qu.:19.10   1st Qu.:0.719
Median :133.0   Median :53.00   Median :0.405   Median :19.32   Median :0.738
Mean   :133.2   Mean   :53.76   Mean   :0.403   Mean   :19.30   Mean   :0.737
3rd Qu.:138.2   3rd Qu.:57.00   3rd Qu.:0.417   3rd Qu.:19.47   3rd Qu.:0.751
Max.   :150.0   Max.   :67.00   Max.   :0.469   Max.   :20.18   Max.   :0.828
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :117.0   Min.   :58.00   Min.   :0.483   Min.   :0.320   Min.   :0.056
1st Qu.:127.0   1st Qu.:73.75   1st Qu.:0.569   1st Qu.:0.331   1st Qu.:0.058
Median :133.0   Median :78.50   Median :0.590   Median :0.352   Median :0.059
Mean   :132.8   Mean   :78.35   Mean   :0.589   Mean   :0.353   Mean   :0.067
3rd Qu.:138.0   3rd Qu.:83.00   3rd Qu.:0.606   3rd Qu.:0.366   3rd Qu.:0.070
Max.   :154.0   Max.   :99.00   Max.   :0.675   Max.   :0.428   Max.   :0.230
```

## The simplewiki-20 benchmark

This benchmark uses a [dump][w] of Simple English Wikipedia's articles names.
Names are filtered; only ones having at least 20B are considered. Space savings
are around 20% for LZMA and 10% for LZ4.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :21.00   Min.   :14.00   Min.   :0.439   Min.   :30.41   Min.   :3.024
1st Qu.:24.00   1st Qu.:19.00   1st Qu.:0.666   1st Qu.:30.51   1st Qu.:3.031
Median :27.00   Median :22.00   Median :0.813   Median :30.64   Median :3.047
Mean   :29.22   Mean   :22.12   Mean   :0.788   Mean   :30.66   Mean   :3.054
3rd Qu.:31.00   3rd Qu.:24.00   3rd Qu.:0.907   3rd Qu.:30.78   3rd Qu.:3.068
Max.   :57.00   Max.   :43.00   Max.   :1.148   Max.   :31.35   Max.   :3.137
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :21.00   Min.   :14.00   Min.   :0.423   Min.   :1.098  Min.   :0.1356
1st Qu.:22.75   1st Qu.:20.00   1st Qu.:0.752   1st Qu.:1.119  1st Qu.:0.1407
Median :25.00   Median :24.00   Median :0.908   Median :1.140  Median :0.1468
Mean   :27.15   Mean   :24.06   Mean   :0.909   Mean   :1.146  Mean   :0.1526
3rd Qu.:29.00   3rd Qu.:28.00   3rd Qu.:1.074   3rd Qu.:1.158  3rd Qu.:0.1583
Max.   :54.00   Max.   :43.00   Max.   :1.381   Max.   :1.374  Max.   :0.2519
```

## The simplewiki-30 benchmark

For a comparison, here are results for the same dataset but limited to names not
shorter than 30B. Here savings are over 40% for LZMA and over 30% for LZ4.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :31.00   Min.   :13.00   Min.   :0.243   Min.   :36.02   Min.   :3.438
1st Qu.:33.00   1st Qu.:18.00   1st Qu.:0.478   1st Qu.:36.17   1st Qu.:3.445
Median :37.00   Median :22.00   Median :0.555   Median :36.28   Median :3.459
Mean   :39.52   Mean   :22.08   Mean   :0.574   Mean   :36.30   Mean   :3.465
3rd Qu.:42.00   3rd Qu.:25.25   3rd Qu.:0.689   3rd Qu.:36.41   3rd Qu.:3.480
Max.   :78.00   Max.   :41.00   Max.   :0.843   Max.   :36.73   Max.   :3.537
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :31.00   Min.   :16.00   Min.   :0.296   Min.   :1.543  Min.   :0.1703
1st Qu.:32.75   1st Qu.:19.75   1st Qu.:0.516   1st Qu.:1.564  1st Qu.:0.1751
Median :36.00   Median :25.00   Median :0.648   Median :1.586  Median :0.1841
Mean   :38.85   Mean   :26.16   Mean   :0.691   Mean   :1.588  Mean   :0.1853
3rd Qu.:40.00   3rd Qu.:32.00   3rd Qu.:0.871   3rd Qu.:1.609  3rd Qu.:0.1917
Max.   :73.00   Max.   :51.00   Max.   :1.281   Max.   :1.671  Max.   :0.2997
```

[b]: http://bundler.io/
[r]: http://www.r-project.org/
[f]: http://faker.rubyforge.org/
[w]: http://dumps.wikimedia.org/simplewiki/20140116/
