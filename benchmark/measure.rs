#![feature(libc)]
extern crate libc;

use std::io::{stdin, Read};
use std::fs::File;
use libc::{size_t, free, c_void};
use std::env::args;
use std::path::Path;
use std::str::FromStr;
use std::ptr::{null_mut, read};

#[link(name = "skr")]
extern {
  fn skr_compress(model: *const u8, modlen: size_t,
                  inp: *const u8, inlen: size_t,
                  outp: *const *mut u8, outlen: size_t,
                  opts: *const u32) -> size_t;
  fn skr_decompress(model: *const u8, modlen: size_t,
                    inp: *const u8, inlen: size_t,
                    outp: *const *mut u8, outlen: size_t,
                    opts: *const u32) -> size_t;
}

type TestFun = unsafe extern "C"
  fn(*const u8, size_t, *const u8, size_t, *const *mut u8, size_t, *const u32)
  -> size_t;

fn from_buf_raw(buf: *const u8, len: usize) -> Vec<u8> {
  let mut vec = Vec::with_capacity(len);
  unsafe {
    for i in 0..len as isize {
      vec.push(read(buf.offset(i)));
    }
  }
  vec
}

fn process_vec(inp: &[u8], inp2: &[u8], fun: TestFun) -> Vec<u8> {
  unsafe {
    let default_opts = 0;
    let buf = null_mut();
    let nout = fun(inp.as_ptr(), inp.len() as size_t,
                   inp2.as_ptr(), inp2.len() as size_t,
                   &buf, 0,
                   &default_opts);
    assert!(buf != null_mut() && nout > 0);
    let out = from_buf_raw(buf, nout as usize);
    free(buf as *mut c_void);
    out
  }
}

fn read_to_buf(file: &mut Read) -> Vec<u8> {
  let mut buf = Vec::new();
  Read::read_to_end(file, &mut buf).unwrap();
  buf
}

fn measure(model_file: &str, attempts: u32, fun: TestFun) {
  let input = read_to_buf(&mut stdin());
  let model = read_to_buf(&mut File::open(&Path::new(model_file)).unwrap());
  for _ in 0..attempts {
    process_vec(&model, &input, fun);
  }
}

fn main() {
  assert!(args().len() == 4);
  let attempts = FromStr::from_str(&args().nth(2).unwrap());
  let fun = match args().nth(1).unwrap().as_ref() {
      "-d" => Some(skr_decompress),
      "-c" => Some(skr_compress),
      _    => None
  };
  let model_file = args().nth(3);
  measure(&model_file.unwrap(), attempts.unwrap(), fun.unwrap())
}
