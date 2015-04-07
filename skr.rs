#![feature(libc)]
extern crate libc;

use std::io::{stdin, stdout, Read, Write};
use std::fs::{File};
use libc::{size_t, free, c_void};
use std::env::args;
use std::path::Path;
use std::ptr::{null_mut, read};

#[link(name = "skr")]
extern {
  fn skr_model(inp: *const u8, inlen: size_t,
               outp: &*mut u8, outlen: size_t,
               opts: *const u32) -> size_t;
  fn skr_compress(model: *const u8, modlen: size_t,
                  inp: *const u8, inlen: size_t,
                  outp: &*mut u8, outlen: size_t,
                  opts: *const u32) -> size_t;
  fn skr_decompress(model: *const u8, modlen: size_t,
                    inp: *const u8, inlen: size_t,
                    outp: &*mut u8, outlen: size_t,
                    opts: *const u32) -> size_t;
}

type Fun = unsafe extern "C" fn(*const u8, size_t, &*mut u8, size_t, *const u32) -> size_t;
type Fun2 = unsafe extern "C" fn(*const u8, size_t, *const u8, size_t, &*mut u8, size_t, *const u32) -> size_t;

fn from_raw_buf(buf: *const u8, len: usize) -> Vec<u8> {
  let mut vec = Vec::with_capacity(len);
  unsafe {
    for i in 0..len as isize {
      vec.push(read(buf.offset(i)));
    }
  }
  vec
}

fn process_vec(inp: Vec<u8>, fun: Fun) -> Vec<u8> {
  unsafe {
    let default_opts = 0;
    let buf = null_mut();
    let nout = fun(inp.as_ptr(), inp.len() as size_t, &buf, 0, &default_opts);
    assert!(!buf.is_null() && nout > 0);
    let out = from_raw_buf(buf as *const u8, nout as usize);
    free(buf as *mut c_void);
    out
  }
}

fn process_vec2(inp: Vec<u8>, inp2: Vec<u8>, fun: Fun2) -> Vec<u8> {
  unsafe {
    let default_opts = 0;
    let buf = null_mut();
    let nout = fun(inp.as_ptr(), inp.len() as size_t,
                   inp2.as_ptr(), inp2.len() as size_t,
                   &buf, 0,
                   &default_opts);
    assert!(!buf.is_null() && nout > 0);
    let out = from_raw_buf(buf as *const u8, nout as usize);
    free(buf as *mut c_void);
    out
  }
}

fn main() {
  let model = || {
    let model_file = args().nth(1).unwrap();
    let mut buf = Vec::new();
    let path = Path::new(&model_file);
    File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    buf
  };
  let output = {
    let input = || {
      let mut buf = Vec::new();
      Read::read_to_end(&mut stdin(), &mut buf).unwrap();
      buf
    };
    let prog = args().nth(0).unwrap();
    if prog.ends_with("mkskr") {
      process_vec(input(), skr_model)
    } else if prog.ends_with("unskr") {
      process_vec2(model(), input(), skr_decompress)
    } else {
      process_vec2(model(), input(), skr_compress)
    }
  };
  stdout().write(&output).unwrap();
}
