#!/usr/bin/env python

from os import uname
from sys import argv, stdin
from ctypes import cdll, byref, create_string_buffer, c_void_p, c_int, string_at
from time import clock

def process_vec(fun, input1, input2):
    outptr = c_void_p()
    opts = c_int(0)
    count = fun(byref(input1), len(input1), byref(input2), len(input2),
                byref(outptr), 0, byref(opts))
    assert count >= 0
    return string_at(outptr, count)

def read_to_buffer(file):
    bytes = file.read()
    return create_string_buffer(bytes, len(bytes))

def load_lib():
    ext = {'Darwin': 'dylib'}.get(uname()[0], 'so')
    return cdll.LoadLibrary('libskr.' + ext)

def measure(model_file, attempts, fun):
    input = read_to_buffer(stdin)
    with open(model_file) as file:
        model = read_to_buffer(file)
    then = clock()
    for _ in range(attempts):
        process_vec(fun, model, input)
    now = clock()
    print((now - then) / attempts)

def main():
    lib = load_lib()
    fun = {'-d': lib.skr_decompress, '-c': lib.skr_compress}[argv[1]]
    attempts = int(argv[2])
    model = argv[3]
    measure(model, attempts, fun)

if __name__ == '__main__':
    main()
