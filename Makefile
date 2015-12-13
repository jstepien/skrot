.PHONY: all clean benchmark tarball

VERSION = $(shell git describe --tags)
PKG = skrot-$(VERSION)
CFLAGS ?= -std=c99 -O2 -Wall -Werror -fPIC
ifeq ($(shell uname), Darwin)
	LIBRARY_PATH = DYLD_LIBRARY_PATH
	SHARED_EXT = dylib
else
	LIBRARY_PATH = LD_LIBRARY_PATH
	SHARED_EXT = so
endif
SHARED_LIB = libskr.$(SHARED_EXT)

all: $(SHARED_LIB)
	ln -sf skr.py mkskr
	ln -sf skr.py unskr
	ln -sf skr.py skr

$(SHARED_LIB): skr.c
	$(CC) $(CFLAGS) $^ -shared -o $@ -llzma -llz4 -I.

clean:
	rm -f unskr mkskr $(SHARED_LIB) $(PKG).tar.bz2
	+make clean -C benchmark

check: all
	./t.sh

benchmark: all
	+make -C benchmark $(LIBRARY_PATH)=$(PWD)

tarball: $(PKG).tar.bz2

$(PKG).tar.bz2:
	test -n "$(VERSION)"
	rm -rf $(PKG)
	mkdir $(PKG)
	bash -c 'cp -r README.md Makefile t.sh skr.{c,h,py} benchmark $(PKG)/'
	sed -i $(PKG)/Makefile -e 's,^VERSION =.*,VERSION = $(VERSION),'
	+make -C $(PKG) clean
	tar -cjf $@ $(PKG)
	rm -rf $(PKG)
