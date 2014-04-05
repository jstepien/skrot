use std::io::{stdin, File};
use std::libc::{size_t, free, c_void};
use std::os::args;
use std::path::Path;
use std::ptr::null;
use std::slice::raw::from_buf_raw;

#[link(name = "skr")]
extern {
  fn skr_compress(model: *u8, modlen: size_t,
                  inp: *u8, inlen: size_t,
                  outp: **u8, outlen: size_t,
                  opts: *u32) -> size_t;
  fn skr_decompress(model: *u8, modlen: size_t,
                    inp: *u8, inlen: size_t,
                    outp: **u8, outlen: size_t,
                    opts: *u32) -> size_t;
}

type F = extern "C" unsafe fn(*u8, size_t, *u8, size_t, **u8, size_t, *u32) -> size_t;

fn process_vec(inp: &[u8], inp2: &[u8], fun: F) -> ~[u8] {
  unsafe {
    let default_opts = 0;
    let buf = null();
    let nout = fun(inp.as_ptr(), inp.len() as size_t,
                   inp2.as_ptr(), inp2.len() as size_t,
                   &buf, 0,
                   &default_opts);
    assert!(buf != null() && nout > 0);
    let out = from_buf_raw(buf, nout as uint);
    free(buf as *mut c_void);
    out
  }
}

fn measure(model_file: ~str, attempts: uint, fun: F) {
  let input = stdin().read_to_end().unwrap();
  let model = File::open(&Path::new(model_file)).read_to_end().unwrap();
  for _ in range(0, attempts) {
    process_vec(model, input, fun);
  }
}

fn main() {
  let args = args();
  assert!(args.len() == 4);
  let attempts = from_str(args[2]).unwrap();
  let f = if args[1] == ~"-d" {
    skr_decompress
  } else if args[1] == ~"-c" {
    skr_compress
  } else {
    fail!()
  };
  measure(args[3], attempts, f)
}
