extern mod extra;

use std::os::args;
use std::io::{stdin, stdout, File};
use std::path::Path;
use std::libc::funcs::posix88::unistd::execv;
use std::ptr::null;
use std::vec::append_one;
use std::libc::types::os::arch::c95::{c_int, size_t};
use std::libc::types::common::c95::c_void;
use std::libc::funcs::c95::stdlib::free;
use extra::future::Future;
use extra::arc::Arc;

extern {
  fn unlzma(inp: *u8, ninp: size_t, out: **u8, nout: size_t) -> c_int;
  fn lzma(inp: *u8, ninp: size_t, out: **u8, nout: size_t) -> c_int;
}

type Fun = extern "C" unsafe fn(*u8, u64, **u8, u64) -> i32;

fn process_vec(inp: &[u8], fun: Fun) -> ~[u8] {
  unsafe {
    let buf = std::ptr::null();
    let nout = fun(inp.as_ptr(), inp.len() as size_t, &buf, 0);
    assert!(buf != std::ptr::null() && nout > 0);
    let out = std::vec::raw::from_buf_raw(buf, nout as uint);
    free(buf as *c_void);
    out
  }
}

fn unlzma_vec(inp: &[u8]) -> ~[u8] {
  process_vec(inp, unlzma)
}

fn lzma_vec(inp: &[u8]) -> ~[u8] {
  process_vec(inp, lzma)
}

fn compress(corpus: ~[u8]) {
  let compressed = {
    let full_corpus = unlzma_vec(corpus);
    let to_compress = full_corpus + stdin().read_to_end();
    lzma_vec(to_compress)
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
  let arc = Arc::new(corpus);
  let (port, chan) = Chan::new();
  chan.send(arc.clone());
  let mut fut = do Future::spawn {
    let arc = port.recv();
    unlzma_vec(*arc.get()).len()
  };
  let patch = stdin().read_to_end();
  let cutoff = arc.get().len() - patch[0] as uint;
  let patched = arc.get().slice(0, cutoff) + patch.slice_from(1);
  let decompressed = unlzma_vec(patched);
  stdout().write(decompressed.slice_from(fut.get()));
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
