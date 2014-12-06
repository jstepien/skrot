extern crate libc;

use std::io::{stdin, stdout, File};
use libc::{size_t, free, c_void};
use std::os::args;
use std::path::Path;
use std::ptr::null_mut;
use std::vec::raw::from_buf;

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

fn process_vec(inp: Vec<u8>, fun: Fun) -> Vec<u8> {
  unsafe {
    let default_opts = 0;
    let buf = null_mut();
    let nout = fun(inp.as_ptr(), inp.len() as size_t, &buf, 0, &default_opts);
    assert!(buf.is_not_null() && nout > 0);
    let out = from_buf(buf as *const u8, nout as uint);
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
    assert!(buf.is_not_null() && nout > 0);
    let out = from_buf(buf as *const u8, nout as uint);
    free(buf as *mut c_void);
    out
  }
}

fn main() {
  let args = args();
  let model = || {
    assert!(args.len() == 2);
    let model_file = args[1].as_slice();
    File::open(&Path::new(model_file)).read_to_end().unwrap()
  };
  let output = {
    let input = || { stdin().read_to_end().unwrap() };
    let prog = args[0].as_slice();
    if prog.ends_with("mkskr") {
      process_vec(input(), skr_model)
    } else if prog.ends_with("unskr") {
      process_vec2(model(), input(), skr_decompress)
    } else {
      process_vec2(model(), input(), skr_compress)
    }
  };
  let _ = stdout().write(output.as_slice()).unwrap();
}
