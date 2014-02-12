extern mod extra;

use std::os::args;
use std::io::{stdin, stdout, File};
use std::path::Path;
use skr::{compress};

mod skr;

fn main() {
  let args = args();
  let corpus = || {
    assert!(args.len() == 2);
    let corpus_file: &str = args[1];
    File::open(&Path::new(corpus_file)).read_to_end()
  };
  let output = {
    let input = stdin().read_to_end();
    if args[0].ends_with("mkskr") {
      skr::corpus(input)
    } else if args[0].ends_with("unskr") {
      skr::decompress(corpus(), input)
    } else {
      skr::compress(corpus(), input)
    }
  };
  stdout().write(output)
}
