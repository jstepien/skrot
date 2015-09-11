# Benchmarking Skrót

Let's see how Skrót performs in some synthetic microbenchmarks.
To run code in this directory you'll need Ruby and [Bundler][b].
[R][r] will be used to obtain some summary statistics.

    bundle install --path vendor/bundle && bundle exec make

All results below were obtained on an i5-4200U CPU.
Time of encoding and decoding is given in milliseconds.

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
LZMA-based Skrót encoding achieves space savings of nearly 60%.
LZ4-based encoding achieves 40% savings and much shorter processing time.
Following tables summarise length of
samples before and after the encoding as well as their ratio and
time of encoding and decoding in milliseconds.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :121.0   Min.   :45.00   Min.   :0.3538  Min.   :7.065  Min.   :0.8449
1st Qu.:130.0   1st Qu.:52.00   1st Qu.:0.3937  1st Qu.:7.206  1st Qu.:0.8546
Median :135.0   Median :55.00   Median :0.4113  Median :7.417  Median :0.8618
Mean   :135.4   Mean   :55.62   Mean   :0.4103  Mean   :7.464  Mean   :0.8636
3rd Qu.:140.2   3rd Qu.:59.00   3rd Qu.:0.4267  3rd Qu.:7.694  3rd Qu.:0.8662
Max.   :151.0   Max.   :68.00   Max.   :0.4643  Max.   :7.992  Max.   :1.0756
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time      Dec. time
Min.   :121.0   Min.   :62.00   Min.   :0.5000  Min.   :0.156  Min.   :0.0216
1st Qu.:130.0   1st Qu.:75.00   1st Qu.:0.5632  1st Qu.:0.157  1st Qu.:0.0219
Median :135.0   Median :78.00   Median :0.5848  Median :0.158  Median :0.0221
Mean   :135.4   Mean   :79.03   Mean   :0.5832  Mean   :0.158  Mean   :0.0222
3rd Qu.:140.2   3rd Qu.:83.25   3rd Qu.:0.6031  3rd Qu.:0.159  3rd Qu.:0.0224
Max.   :151.0   Max.   :98.00   Max.   :0.6490  Max.   :0.168  Max.   :0.0243
```

## The simplewiki-20 benchmark

This benchmark uses a [dump][w] of Simple English Wikipedia's articles names.
Names are filtered; only ones having at least 20B are considered.
LZMA shrinks input values by nearly 15%. LZ4 doesn't lead to any savings; in
fact encoded values are marginally longer.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :21.00   Min.   :12.00   Min.   :0.3617  Min.   :18.84  Min.   :3.634
1st Qu.:24.00   1st Qu.:18.00   1st Qu.:0.6429  1st Qu.:19.20  1st Qu.:3.648
Median :28.00   Median :22.00   Median :0.7710  Median :19.52  Median :3.653
Mean   :29.24   Mean   :21.78   Mean   :0.7722  Mean   :19.52  Mean   :3.665
3rd Qu.:32.00   3rd Qu.:26.00   3rd Qu.:0.8986  3rd Qu.:19.73  3rd Qu.:3.669
Max.   :66.00   Max.   :38.00   Max.   :1.1818  Max.   :20.97  Max.   :3.877
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :21.00   Min.   :18.00  Min.   :0.3548  Min.   :0.9513  Min.   :0.073
1st Qu.:22.00   1st Qu.:25.00  1st Qu.:0.9000  1st Qu.:0.9588  1st Qu.:0.074
Median :26.00   Median :29.00  Median :1.1053  Median :0.9617  Median :0.074
Mean   :28.87   Mean   :28.85  Mean   :1.0651  Mean   :0.9642  Mean   :0.075
3rd Qu.:32.00   3rd Qu.:33.00  3rd Qu.:1.2884  3rd Qu.:0.9684  3rd Qu.:0.075
Max.   :69.00   Max.   :52.00  Max.   :1.5714  Max.   :0.9895  Max.   :0.085
```

## The simplewiki-30 benchmark

For a comparison, here are results for the same dataset but limited to names not
shorter than 30B. Here savings are nearly 40% for LZMA and around 25% for LZ4.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :31.00   Min.   :13.00   Min.   :0.2698  Min.   :25.65  Min.   :4.244
1st Qu.:34.00   1st Qu.:21.00   1st Qu.:0.5097  1st Qu.:25.99  1st Qu.:4.257
Median :38.00   Median :25.00   Median :0.6443  Median :26.18  Median :4.265
Mean   :40.56   Mean   :25.03   Mean   :0.6312  Mean   :26.30  Mean   :4.275
3rd Qu.:45.00   3rd Qu.:29.00   3rd Qu.:0.7441  3rd Qu.:26.52  3rd Qu.:4.280
Max.   :66.00   Max.   :41.00   Max.   :1.0000  Max.   :27.56  Max.   :4.410
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :31.00   Min.   :19.00   Min.   :0.3455  Min.   :1.365  Min.   :0.089
1st Qu.:34.00   1st Qu.:25.00   1st Qu.:0.6308  1st Qu.:1.382  1st Qu.:0.090
Median :38.00   Median :30.00   Median :0.7500  Median :1.392  Median :0.090
Mean   :40.56   Mean   :30.88   Mean   :0.7787  Mean   :1.400  Mean   :0.091
3rd Qu.:45.00   3rd Qu.:36.00   3rd Qu.:0.9058  3rd Qu.:1.407  3rd Qu.:0.091
Max.   :66.00   Max.   :48.00   Max.   :1.3548  Max.   :1.655  Max.   :0.099
```

[b]: http://bundler.io/
[r]: http://www.r-project.org/
[f]: http://faker.rubyforge.org/
[w]: http://dumps.wikimedia.org/simplewiki/20150406/
