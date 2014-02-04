#!/usr/bin/env python2
from subprocess import PIPE, Popen
import sys

def run(cmd, stdin=None):
    popen_stdin = None if stdin is None else PIPE
    proc = Popen(cmd, stdin=popen_stdin, stderr=PIPE, stdout=PIPE)
    (out, err) = proc.communicate(input=stdin)
    if proc.wait() != 0:
        raise Exception(err)
    return out

def compress(corpus_file):
    corpus_comp = file(corpus_file).read()
    corpus = run(["unlzma"], corpus_comp)
    compressed = run(["lzma", "-e"], corpus + sys.stdin.read())
    idx = 0
    while compressed[idx] == corpus_comp[idx]:
        idx += 1
    diff = len(corpus_comp) - idx
    assert 0 <= diff <= 0xff
    sys.stdout.write(bytearray([diff]))
    sys.stdout.write(compressed[idx:])

def decompress(corpus_file):
    corpus_len = len(run(["lzcat", corpus_file]))
    corpus = file(corpus_file).read()
    patch = sys.stdin.read()
    cutoff = len(corpus) - ord(patch[0])
    decompressed = run(["unlzma"], corpus[:cutoff] + patch[1:])
    sys.stdout.write(decompressed[corpus_len:])

if __name__ == "__main__":
    prog, corpus_file = sys.argv
    if prog.endswith("unskr.py"):
        decompress(corpus_file)
    else:
        compress(corpus_file)
