# Skrót

A customisable compression utility dedicated to short inputs.

Take a look at some [numbers][numbers].

[numbers]: https://github.com/jstepien/skrot/tree/master/benchmark

## Usage

A tentative C API is specified in `skr.h`.
It's implemented in form of a shared library `libskr`.
Its only dependency is [liblzma][xz]; it's installed by default on most
contemporary GNU/Linux machines and is available in [Homebrew][brew].

`skr` and its aliases `unskr` and `mkskr` are command line wrappers around
`libskr`.
As of time of writing it's written in Rust.
They should be invoked in a following way:

  - `mkskr < raw_model > model` builds a Skrót model file from standard input,
  - `skr model < input > compressed` compresses contents of standard input using
    the model,
  - `unskr model < compressed > uncompressed` decompresses contents of standard
    input using the model.

Both `skr` and `libskr` can be built by invoking `make`.

[xz]: http://tukaani.org/xz/
[brew]: https://github.com/Homebrew/homebrew/blob/master/Library/Formula/xz.rb

## Theoretical background

[LZMA][lzma] is a dictionary based compression algorithm used in, among others,
[7zip][7z] and [xz-utils][xz].
One of its interesting features is the fact that changes introduced at the end
of an uncompressed input stream result only in changes at the end of the
compressed output stream; the rest remains intact.
Moreover, in its basic form—LZMA1—it doesn't use any checksums.

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
[7z]: http://www.7-zip.org/

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
