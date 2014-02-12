extern mod extra;

use std::ptr::null;
use std::vec::raw::from_buf_raw;
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
    let buf = null();
    let nout = fun(inp.as_ptr(), inp.len() as size_t, &buf, 0);
    assert!(buf != null() && nout > 0);
    let out = from_buf_raw(buf, nout as uint);
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

pub fn compress(corpus: &[u8], input: &[u8]) -> ~[u8] {
  let compressed = {
    let full_corpus = unlzma_vec(corpus);
    let to_compress = full_corpus + input;
    lzma_vec(to_compress)
  };
  let mut idx = 0;
  while compressed[idx] == corpus[idx] {
    idx += 1
  }
  let diff: uint = corpus.len() - idx;
  assert!(diff <= 0xff);
  [diff as u8] + compressed.slice_from(idx)
}

pub fn decompress(corpus: &[u8], input: &[u8]) -> ~[u8] {
  let arc = Arc::new(corpus.to_owned());
  let (port, chan) = Chan::new();
  chan.send(arc.clone());
  let mut fut = do Future::spawn {
    let arc = port.recv();
    unlzma_vec(*arc.get()).len()
  };
  let cutoff = arc.get().len() - input[0] as uint;
  let patched = arc.get().slice(0, cutoff) + input.slice_from(1);
  let decompressed = unlzma_vec(patched);
  decompressed.slice_from(fut.get()).to_owned()
}

pub fn corpus(input: &[u8]) -> ~[u8] {
  lzma_vec(input)
}
