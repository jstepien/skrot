.PHONY: all clean benchmark

CFLAGS ?= -std=c99 -O2 -Wall -Werror -fPIC
ifeq ($(shell uname), Darwin)
	LIBRARY_PATH = DYLD_LIBRARY_PATH
	SHARED_EXT = dylib
else
	LIBRARY_PATH = LD_LIBRARY_PATH
	SHARED_EXT = so
endif
SHARED_LIB = libskr.$(SHARED_EXT)

all: $(SHARED_LIB) skr

$(SHARED_LIB): skr.c
	$(CC) $(CFLAGS) $^ -shared -o $@ -llzma -llz4 -I.

skr: skr.rs $(SHARED_LIB)
	rustc -O $< -o $@ --link-args "-L. -lskr"

clean:
	rm -f skr $(SHARED_LIB)
	+make clean -C benchmark

benchmark:
	+make -C benchmark $(LIBRARY_PATH)=$(PWD)
