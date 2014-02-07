use std::os::args;
use std::run::{Process, ProcessOptions};
use std::io::{stdin, stdout, File};
use std::path::Path;
use std::libc::funcs::posix88::unistd::execv;
use std::ptr::null;
use std::vec::append_one;

fn exec(cmd: ~str, input: &[u8]) -> Option<~[u8]> {
  let args = [~"-c", cmd];
  let prog = ~"/bin/sh";
  let opts = ProcessOptions {
    env: None,
    dir: None,
    in_fd: None,
    out_fd: None,
    err_fd: Some(2)
  };
  let mut process = Process::new(prog, args, opts).unwrap();
  process.input().write(input);
  process.close_input();
  let out = process.output().read_to_end();
  assert!(process.finish().success());
  Some(out)
}

fn compress(corpus: ~[u8]) {
  let compressed = {
    let full_corpus = exec(~"lzcat", corpus).unwrap();
    let to_compress = full_corpus + stdin().read_to_end();
    exec(~"lzma -e", to_compress).unwrap()
  };
  let mut idx = 0;
  while compressed[idx] == corpus[idx] {
    idx += 1
  }
  let diff: uint = corpus.len() - idx;
  assert!(diff <= 0xff);
  stdout().write_u8(diff as u8);
  stdout().write(compressed.slice_from(idx));
}

fn decompress(corpus: ~[u8]) {
  let patch = stdin().read_to_end();
  let cutoff = corpus.len() - patch[0] as uint;
  let patched = corpus.slice(0, cutoff) + patch.slice_from(1);
  let decompressed = exec(~"lzcat", patched).unwrap();
  let full_corpus_len = exec(~"lzcat", corpus).unwrap().len();
  stdout().write(decompressed.slice_from(full_corpus_len));
}

fn exec_lzma() {
  unsafe {
    let args = append_one(
      ["env", "lzma", "-e"].map(|s| { s.to_c_str().unwrap() }),
      null());
    assert!(execv("/usr/bin/env".to_c_str().unwrap(), args.as_ptr()) == 0);
  }
}

fn main() {
  let args = args();
  let corpus = || {
    assert!(args.len() == 2);
    let corpus_file: &str = args[1];
    File::open(&Path::new(corpus_file)).read_to_end()
  };
  if args[0].ends_with("mkskr") {
    exec_lzma()
  } else if args[0].ends_with("unskr") {
    decompress(corpus())
  } else {
    compress(corpus())
  }
}
