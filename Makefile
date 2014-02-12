.PHONY: all clean

CFLAGS ?= -std=c99 -O2 -Wall -Werror -fPIC
LIBS = -lm -ldl -llzma -lpthread
RUSTLIBDIR = $(shell dirname $(shell which rustc))/../lib/rustlib/*/lib
RUSTLIBS = $(RUSTLIBDIR)/*.rlib $(RUSTLIBDIR)/libmorestack.a
ifeq ($(shell uname), Darwin)
	SHARED_EXT = dylib
	LDFLAGS = -Wl,-U,__rust_crate_map_toplevel
else
	SHARED_EXT = so
	LDFLAGS =
endif
SHARED_LIB = libskr.$(SHARED_EXT)

all: $(SHARED_LIB) skr

$(SHARED_LIB): capi.o lzma.o
	$(CC) -shared $^ $(RUSTLIBS) $(LIBS) $(LDFLAGS) -o $@

capi.o: capi.rs skr.rs
	rustc -O $< -c

skr: main.rs skr.rs lzma.o
	rustc -O $< -o $@ --link-args "lzma.o -llzma"

clean:
	rm -f skr $(SHARED_LIB) lzma.o capi*.o
	+make clean -C benchmark
