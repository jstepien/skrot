.PHONY: clean

skr: skr.rs
	rustc -O $< -o $@

clean:
	rm -f skr
