# Benchmarking Skrót

Let's see how Skrót performs in some synthetic microbenchmarks.
To run code in this directory you'll need Ruby and [Bundler][b].
[R][r] will be used to obtain some summary statistics.

    bundle && make

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
samples before and after the encoding as well as their ratio and slightly
overestimated time of encoding and decoding in seconds.

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :122.0   Min.   :50.00   Min.   :0.362   Min.   :0.045   Min.   :0.022
1st Qu.:130.0   1st Qu.:53.00   1st Qu.:0.407   1st Qu.:0.048   1st Qu.:0.023
Median :135.0   Median :56.50   Median :0.420   Median :0.049   Median :0.024
Mean   :134.5   Mean   :56.54   Mean   :0.420   Mean   :0.049   Mean   :0.025
3rd Qu.:138.0   3rd Qu.:59.00   3rd Qu.:0.433   3rd Qu.:0.050   3rd Qu.:0.026
Max.   :150.0   Max.   :67.00   Max.   :0.457   Max.   :0.059   Max.   :0.038
```

## The simplewiki benchmark

This benchmark uses a [dump][w] of Simple English Wikipedia's articles names.
Names are filtered; only ones having at least 20B are considered. Space savings
are around 20%.

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :21.00   Min.   :14.00   Min.   :0.327   Min.   :0.062   Min.   :0.025
1st Qu.:22.75   1st Qu.:19.00   1st Qu.:0.663   1st Qu.:0.065   1st Qu.:0.027
Median :26.00   Median :22.00   Median :0.812   Median :0.067   Median :0.029
Mean   :29.91   Mean   :21.95   Mean   :0.787   Mean   :0.068   Mean   :0.029
3rd Qu.:32.00   3rd Qu.:25.00   3rd Qu.:0.952   3rd Qu.:0.070   3rd Qu.:0.030
Max.   :81.00   Max.   :38.00   Max.   :1.190   Max.   :0.077   Max.   :0.037

```

For a comparison, here are results for the same dataset but limited to names not
shorter than 30B. Here savings are over 40%.

```
   Original        Encoded          Ratio         Enc. time       Dec. time
Min.   :31.00   Min.   :14.00   Min.   :0.291   Min.   :0.068   Min.   :0.025
1st Qu.:32.00   1st Qu.:17.00   1st Qu.:0.466   1st Qu.:0.072   1st Qu.:0.027
Median :35.00   Median :22.00   Median :0.548   Median :0.073   Median :0.029
Mean   :38.67   Mean   :22.06   Mean   :0.586   Mean   :0.074   Mean   :0.029
3rd Qu.:41.25   3rd Qu.:25.25   3rd Qu.:0.682   3rd Qu.:0.076   3rd Qu.:0.031
Max.   :73.00   Max.   :37.00   Max.   :1.032   Max.   :0.082   Max.   :0.036
```

All results above were obtained on a glacial age Core 2 Duo running at 2.1GHz.

[b]: http://bundler.io/
[r]: http://www.r-project.org/
[f]: http://faker.rubyforge.org/
[w]: http://dumps.wikimedia.org/simplewiki/20140116/
