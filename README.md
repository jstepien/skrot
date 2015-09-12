# Skrót

A customisable compression utility dedicated to short inputs.
[![Build Status](https://travis-ci.org/jstepien/skrot.svg?branch=master)](https://travis-ci.org/jstepien/skrot)

[Skrót][definition] allows you to build a model of your data
and use it to compress short byte sequences of predictable contents.
It can efficiently compress byte sequences shorter than 200B.
It's based on established dictionary-based data compression algorithms: LZMA and
LZ4.

Interested in some numbers?
Take a look at [results of our benchmarks][numbers].

Skrót comes in two variants:

  - a native library written in portable C, and
  - a pure Java library with no native dependencies.

[numbers]: https://github.com/jstepien/skrot/tree/master/benchmark
[definition]: http://en.wiktionary.org/wiki/skr%C3%B3t

## Usage

Skrót has two implementations: a native one and a JVM one.
They share no code and the way they're built and used differs.

### Native library and a command line tool

A tentative C API is specified in `skr.h`.
It's implemented in form of a shared library `libskr`.
Its only dependencies are [liblz4][lz4] and [liblzma][xz];
they're present in repositories of most contemporary GNU/Linux distributions and
they're available in [Homebrew][brew] as `lz4` and `xz`.

`skr` and its aliases `unskr` and `mkskr` are command line wrappers around
`libskr`.
As of time of writing it's written in Python.
They should be invoked in a following way:

  - `mkskr < raw_model > model` builds a Skrót model file from standard input,
  - `skr model < input > compressed` compresses contents of standard input using
    the model,
  - `unskr model < compressed > uncompressed` decompresses contents of standard
    input using the model.

Both `skr` and `libskr` can be built by invoking `make`.

[xz]: http://tukaani.org/xz/
[lz4]: https://code.google.com/p/lz4/
[brew]: https://github.com/Homebrew/homebrew

### Java library

The API of the Java library has some [documentation][javadoc].

The Java library is built with [Leiningen][lein];
Install it first if you want to build a JAR.
Afterwards execute `lein jar` in the `java` directory.
The JAR will be placed in the `java/target` directory.

[lein]: http://leiningen.org/
[javadoc]: https://jstepien.github.io/skrot/javadoc/master/

## Theoretical background

[LZMA][lzma] is a dictionary based compression algorithm used in, among others,
[7zip][7z] and [xz-utils][xz].
[LZ4][lz4-algo] is a compression algorithm belonging to the same family as LZMA
and it's used among others in the Linux kernel, Hadoop and BSD implementation of
ZFS.
One of interesting features of these algorithms is the fact that changes
introduced at the end of an uncompressed input stream result only in changes at
the end of the compressed output stream; the rest remains intact.
Moreover, LZ4 and the basic form of LZMA—i.e. LZMA1—don't use any checksums.

Let's focus on LZMA1 in the rest of this section
(the same applies to LZ4, though).
Consider the following back-of-the-envelope reasoning.
Let's take two non-empty byte sequences, `a` and `b`.
Compressing `a` with LZMA1 produces `comp_a`.
Compressing a concatenation of `a` and `b` produces `comp_ab`.
If `a` and `b` are _similar_ the difference of lengths of `comp_ab` and
`comp_a` is small compared to `b`.
As an example of _similar_ byte sequences consider a C source file and a
corresponding header, or two JSON documents with an identical schema.

    ╭───┄─╮         ╭─────────╮
    │ a   │       → │ comp_a  │
    ╰───┄─╯         ╰─────────╯
    ╭───┄─┬───┄─╮   ╭───────────╮
    │ a   │ b   │ → │ comp_ab   │
    ╰───┄─┴───┄─╯   ╰───────────╯

Sequences `comp_a` and `comp_ab` are identical up to index `n`,
where the first differing byte has index `n + 1 ≤ length(comp_a)`.

    ╭─────────────────╮
    │ comp_a          │
    ╰─────────────────╯
    ╭───────────────────────────╮
    │ comp_ab                   │
    ╰───────────────────────────╯
    ╭─────────────┬─────────────╮
    │ comp_a[:n]  │ comp_ab[n:] │
    ╰─────────────┴─────────────╯
    0             n

Since LZMA1 doesn't use any form of checksumming, in order to recover `b` all
we need is `comp_a`, `n` and `comp_ab[n:]`.
Typically, the difference between `length(comp_a)` and `n` is significantly
lesser than 255.
As a consequence, `comp_ab[n:]` should be relatively short as long as `a` and
`b` are _similar_.

This allows us to build a specialised compression tool for any byte sequence
`x` _similar_ to `a`.
A compressed representation of `a`, `comp_a`, shall be built into the tool.

Given `x`, in order to compress it the tool

  - uncompresses `comp_a` to produce `a`,
  - compress concatenation of `a` and `x` to produce `comp_ax`,
  - determines smallest `n` such as `comp_ax[n] ≠ comp_a[n]`, and
  - saves `n` and `comp_ax[n:]` as a compressed representation of `x`.

In order to obtain `x` given `n` and byte sequence `patch`,

  - uncompresses `comp_a` to produce `a`,
  - uncompresses a concatenation of `comp_a[:n]` and `patch` to produce
    `comp_ax`, and
  - recovers `x` by dropping `len(a)` first elements of `comp_ax`.

That's the whole algorithm. It tends to work.

[lzma]: https://en.wikipedia.org/wiki/LZMA
[lz4-algo]: https://en.wikipedia.org/wiki/LZ4_%28compression_algorithm%29
[7z]: http://www.7-zip.org/

## Related work

[Smaz][smaz] “is a simple compression library suitable for compressing very
short strings”. It excels at compressing English.

[smaz]: https://github.com/antirez/smaz

## License

    Copyright (c) 2014 Jan Stępień

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation
    files (the "Software"), to deal in the Software without
    restriction, including without limitation the rights to use,
    copy, modify, merge, publish, distribute, sublicense, and/or
    sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included
    in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
    OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
    THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
