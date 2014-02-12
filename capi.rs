#[crate_type = "lib"];

extern mod extra;

use std::libc::types::os::arch::c95::size_t;
use std::libc::types::common::c95::c_void;
use std::libc::funcs::c95::stdlib::realloc;
use std::vec::raw::from_buf_raw;
use skr::{compress};

mod skr;

extern {
  fn memcpy(dst: *u8, src: *u8, n: size_t) -> *u8;
}

fn skr_fun(fun: fn(&[u8], &[u8]) -> ~[u8],
           corp: *u8, corlen: size_t,
           inp: *u8, inlen: size_t,
           outp: *mut *u8, outlen: size_t) -> size_t {
  unsafe {
    let corpus = from_buf_raw(corp, corlen as uint);
    let input = from_buf_raw(inp, inlen as uint);
    let output = fun(corpus, input);
    let output_len = output.len() as size_t;
    if outlen < output_len {
      *outp = realloc(*outp as *mut c_void, output_len) as *u8;
    }
    memcpy(*outp, output.as_ptr(), output_len);
    output_len
  }
}

#[no_mangle]
pub extern fn skr_compress(corp: *u8, corlen: size_t,
                           inp: *u8, inlen: size_t,
                           outp: *mut *u8, outlen: size_t) -> size_t {
  skr_fun(skr::compress, corp, corlen, inp, inlen, outp, outlen)
}

#[no_mangle]
pub extern fn skr_decompress(corp: *u8, corlen: size_t,
                             inp: *u8, inlen: size_t,
                             outp: *mut *u8, outlen: size_t) -> size_t {
  skr_fun(skr::decompress, corp, corlen, inp, inlen, outp, outlen)
}

#[no_mangle]
pub extern fn skr_corpus(inp: *u8, inlen: size_t,
                         outp: *mut *u8, outlen: size_t) -> size_t {
  unsafe {
    let input = from_buf_raw(inp, inlen as uint);
    let output = skr::corpus(input);
    let output_len = output.len() as size_t;
    if outlen < output_len {
      *outp = realloc(*outp as *mut c_void, output_len) as *u8;
    }
    memcpy(*outp, output.as_ptr(), output_len);
    output_len
  }
}
