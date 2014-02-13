use std::io::{stdin, stdout, File};
use std::libc::{size_t, free, c_void};
use std::os::args;
use std::path::Path;
use std::ptr::null;
use std::vec::raw::from_buf_raw;

extern {
  fn skr_corpus(inp: *u8, inlen: size_t,
                outp: **u8, outlen: size_t) -> size_t;
  fn skr_compress(corp: *u8, corlen: size_t,
                  inp: *u8, inlen: size_t,
                  outp: **u8, outlen: size_t) -> size_t;
  fn skr_decompress(corp: *u8, corlen: size_t,
                    inp: *u8, inlen: size_t,
                    outp: **u8, outlen: size_t) -> size_t;
}

type Fun = extern "C" unsafe fn(*u8, size_t, **u8, size_t) -> size_t;
type Fun2 = extern "C" unsafe fn(*u8, size_t, *u8, size_t, **u8, size_t) -> size_t;

fn process_vec(inp: &[u8], fun: Fun) -> ~[u8] {
  unsafe {
    let buf = null();
    let nout = fun(inp.as_ptr(), inp.len() as size_t, &buf, 0);
    assert!(buf != null() && nout > 0);
    let out = from_buf_raw(buf, nout as uint);
    free(buf as *c_void);
    out
  }
}

fn process_vec2(inp: &[u8], inp2: &[u8], fun: Fun2) -> ~[u8] {
  unsafe {
    let buf = null();
    let nout = fun(inp.as_ptr(), inp.len() as size_t,
                   inp2.as_ptr(), inp2.len() as size_t,
                   &buf, 0);
    assert!(buf != null() && nout > 0);
    let out = from_buf_raw(buf, nout as uint);
    free(buf as *c_void);
    out
  }
}

fn main() {
  let args = args();
  let corpus = || {
    assert!(args.len() == 2);
    let corpus_file: &str = args[1];
    File::open(&Path::new(corpus_file)).read_to_end()
  };
  let output = {
    let input = || { stdin().read_to_end() };
    if args[0].ends_with("mkskr") {
      process_vec(input(), skr_corpus)
    } else if args[0].ends_with("unskr") {
      process_vec2(corpus(), input(), skr_decompress)
    } else {
      process_vec2(corpus(), input(), skr_compress)
    }
  };
  stdout().write(output)
}
