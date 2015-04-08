# Benchmarking Skrót

Let's see how Skrót performs in some synthetic microbenchmarks.
To run code in this directory you'll need Ruby and [Bundler][b].
[R][r] will be used to obtain some summary statistics.

    bundle && bundle exec make

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
LZMA-based Skrót encoding achieves space savings of over 55%.
LZ4-based encoding achieves 40% savings and much shorter processing time.
Following tables summarise length of
samples before and after the encoding as well as their ratio and
time of encoding and decoding in milliseconds.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :120.0   Min.   :43.00   Min.   :0.3543  Min.   :7.276  Min.   :0.8594
1st Qu.:127.8   1st Qu.:49.00   1st Qu.:0.3857  1st Qu.:7.349  1st Qu.:0.8723
Median :132.0   Median :53.00   Median :0.4007  Median :7.400  Median :0.8766
Mean   :133.6   Mean   :53.63   Mean   :0.4010  Mean   :7.432  Mean   :0.8781
3rd Qu.:139.0   3rd Qu.:57.25   3rd Qu.:0.4156  3rd Qu.:7.475  3rd Qu.:0.8817
Max.   :153.0   Max.   :65.00   Max.   :0.4483  Max.   :8.142  Max.   :0.9409
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time      Dec. time
Min.   :120.0   Min.   : 66.00  Min.   :0.5308  Min.   :0.149  Min.   :0.0179
1st Qu.:127.8   1st Qu.: 75.00  1st Qu.:0.5827  1st Qu.:0.151  1st Qu.:0.0182
Median :132.0   Median : 80.00  Median :0.6043  Median :0.152  Median :0.0184
Mean   :133.6   Mean   : 80.62  Mean   :0.6030  Mean   :0.153  Mean   :0.0187
3rd Qu.:139.0   3rd Qu.: 86.00  3rd Qu.:0.6250  3rd Qu.:0.153  3rd Qu.:0.0189
Max.   :153.0   Max.   :106.00  Max.   :0.6928  Max.   :0.187  Max.   :0.0253
```

## The simplewiki-20 benchmark

This benchmark uses a [dump][w] of Simple English Wikipedia's articles names.
Names are filtered; only ones having at least 20B are considered. Space savings
are around 15% for both LZMA and LZ4.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :21.00   Min.   :14.00   Min.   :0.4070  Min.   :19.18  Min.   :3.716
1st Qu.:23.00   1st Qu.:20.00   1st Qu.:0.7308  1st Qu.:19.40  1st Qu.:3.735
Median :25.50   Median :23.50   Median :0.8571  Median :19.47  Median :3.764
Mean   :28.96   Mean   :23.73   Mean   :0.8667  Mean   :19.53  Mean   :3.766
3rd Qu.:31.00   3rd Qu.:26.00   3rd Qu.:1.0435  3rd Qu.:19.64  3rd Qu.:3.777
Max.   :86.00   Max.   :39.00   Max.   :1.2727  Max.   :20.85  Max.   :4.203
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :21.00   Min.   :13.0   Min.   :0.2600  Min.   :0.9400  Min.   :0.070
1st Qu.:23.00   1st Qu.:19.0   1st Qu.:0.6721  1st Qu.:0.9475  1st Qu.:0.071
Median :25.50   Median :23.0   Median :0.8519  Median :0.9518  Median :0.071
Mean   :28.96   Mean   :23.2   Mean   :0.8501  Mean   :0.9538  Mean   :0.072
3rd Qu.:31.00   3rd Qu.:26.0   3rd Qu.:1.0536  3rd Qu.:0.9582  3rd Qu.:0.072
Max.   :86.00   Max.   :38.0   Max.   :1.2273  Max.   :0.9822  Max.   :0.075
```

## The simplewiki-30 benchmark

For a comparison, here are results for the same dataset but limited to names not
shorter than 30B. Here savings are over 40% for LZMA and nearly 30% for LZ4.

### LZMA

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :31.00   Min.   :13.00   Min.   :0.2742  Min.   :25.33  Min.   :4.208
1st Qu.:33.75   1st Qu.:18.00   1st Qu.:0.4362  1st Qu.:25.57  1st Qu.:4.224
Median :37.00   Median :22.50   Median :0.5679  Median :25.79  Median :4.260
Mean   :40.86   Mean   :22.52   Mean   :0.5741  Mean   :25.82  Mean   :4.261
3rd Qu.:43.25   3rd Qu.:26.00   3rd Qu.:0.7059  3rd Qu.:25.96  3rd Qu.:4.290
Max.   :82.00   Max.   :42.00   Max.   :0.9412  Max.   :26.83  Max.   :4.375
```

### LZ4

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :31.00   Min.   :17.00   Min.   :0.3559  Min.   :1.360  Min.   :0.083
1st Qu.:33.75   1st Qu.:24.00   1st Qu.:0.5714  1st Qu.:1.375  1st Qu.:0.084
Median :37.00   Median :28.00   Median :0.7292  Median :1.384  Median :0.085
Mean   :40.86   Mean   :28.74   Mean   :0.7333  Mean   :1.389  Mean   :0.086
3rd Qu.:43.25   3rd Qu.:33.00   3rd Qu.:0.8832  3rd Qu.:1.393  3rd Qu.:0.086
Max.   :82.00   Max.   :58.00   Max.   :1.1471  Max.   :1.471  Max.   :0.120
```

[b]: http://bundler.io/
[r]: http://www.r-project.org/
[f]: http://faker.rubyforge.org/
[w]: http://dumps.wikimedia.org/simplewiki/20150406/
