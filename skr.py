#!/usr/bin/env python

from os import uname
from sys import argv, stdin, stdout
from ctypes import cdll, byref, create_string_buffer, c_void_p, c_int, string_at

def process_vec(fun, input, input2=None):
    outptr = c_void_p()
    opts = c_int(0)
    if input2 is None:
        count = fun(byref(input), len(input), byref(outptr), 0, byref(opts))
    else:
        count = fun(byref(input), len(input), byref(input2), len(input2),
                    byref(outptr), 0, byref(opts))
    assert count >= 0
    return string_at(outptr, count)

def read_to_buffer(file):
    bytes = file.read()
    return create_string_buffer(bytes, len(bytes))

def model_from_args():
    with open(argv[1]) as file:
        return read_to_buffer(file)

def load_lib():
    ext = {'Darwin': 'dylib'}.get(uname()[0], 'so')
    return cdll.LoadLibrary('libskr.' + ext)

def main():
    input = read_to_buffer(stdin)
    lib = load_lib()
    prog = argv[0]
    if prog.endswith('mkskr'):
        output = process_vec(lib.skr_model, input)
    elif prog.endswith('unskr'):
        output = process_vec(lib.skr_decompress, model_from_args(), input)
    else:
        output = process_vec(lib.skr_compress, model_from_args(), input)
    stdout.write(output)

if __name__ == '__main__':
    main()
