.PHONY: clean

CFLAGS ?= -std=c99 -O2 -Wall -Werror

skr: skr.rs lzma.o
	rustc -O $< -o $@ --link-args "lzma.o -llzma"

clean:
	rm -f skr lzma.o
	+make clean -C benchmark
