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
tools, e.g. `gzip`, `bzip2` and `xz`, make them even longer. Skrót encoding
achieves space savings of over 55%. The following table summarises length of
samples before and after the encoding as well as their ratio and
time of encoding and decoding in milliseconds.

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :117.0   Min.   :43.00   Min.   :0.338   Min.   :18.97   Min.   :0.709
1st Qu.:128.0   1st Qu.:50.00   1st Qu.:0.388   1st Qu.:19.10   1st Qu.:0.719
Median :133.0   Median :53.00   Median :0.405   Median :19.32   Median :0.738
Mean   :133.2   Mean   :53.76   Mean   :0.403   Mean   :19.30   Mean   :0.737
3rd Qu.:138.2   3rd Qu.:57.00   3rd Qu.:0.417   3rd Qu.:19.47   3rd Qu.:0.751
Max.   :150.0   Max.   :67.00   Max.   :0.469   Max.   :20.18   Max.   :0.828
```

## The simplewiki benchmark

This benchmark uses a [dump][w] of Simple English Wikipedia's articles names.
Names are filtered; only ones having at least 20B are considered. Space savings
are around 20%.

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :21.00   Min.   :14.00   Min.   :0.439   Min.   :30.41   Min.   :3.024
1st Qu.:24.00   1st Qu.:19.00   1st Qu.:0.666   1st Qu.:30.51   1st Qu.:3.031
Median :27.00   Median :22.00   Median :0.813   Median :30.64   Median :3.047
Mean   :29.22   Mean   :22.12   Mean   :0.788   Mean   :30.66   Mean   :3.054
3rd Qu.:31.00   3rd Qu.:24.00   3rd Qu.:0.907   3rd Qu.:30.78   3rd Qu.:3.068
Max.   :57.00   Max.   :43.00   Max.   :1.148   Max.   :31.35   Max.   :3.137
```

For a comparison, here are results for the same dataset but limited to names not
shorter than 30B. Here savings are over 40%.

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :31.00   Min.   :13.00   Min.   :0.243   Min.   :36.02   Min.   :3.438
1st Qu.:33.00   1st Qu.:18.00   1st Qu.:0.478   1st Qu.:36.17   1st Qu.:3.445
Median :37.00   Median :22.00   Median :0.555   Median :36.28   Median :3.459
Mean   :39.52   Mean   :22.08   Mean   :0.574   Mean   :36.30   Mean   :3.465
3rd Qu.:42.00   3rd Qu.:25.25   3rd Qu.:0.689   3rd Qu.:36.41   3rd Qu.:3.480
Max.   :78.00   Max.   :41.00   Max.   :0.843   Max.   :36.73   Max.   :3.537
```

[b]: http://bundler.io/
[r]: http://www.r-project.org/
[f]: http://faker.rubyforge.org/
[w]: http://dumps.wikimedia.org/simplewiki/20140116/
