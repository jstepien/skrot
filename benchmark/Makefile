.PHONY: all clean

all: ../skr people.csv simplewiki.csv
	(\
		echo 'summary(read.csv("people.csv"))';\
		echo 'summary(read.csv("simplewiki.csv"))';\
		) | R -q --vanilla

../skr:
	+make -C .. skr

people.csv: people.lzma people-sample
	cat people-sample | ./benchmark.rb people.lzma > $@

people-sample:
	./generate_people.rb | head -100 > $@

people.lzma:
	./generate_people.rb | head -50 | lzma -e > $@

DUMP = simplewiki-20140116-all-titles

simplewiki.csv: simplewiki.lzma simplewiki-sample
	cat simplewiki-sample | ./benchmark.rb simplewiki.lzma > $@

simplewiki-sample: $(DUMP)
	head -100 $^ > $@

simplewiki.lzma: $(DUMP)
	head -1000 $^ | lzma -e > $@

$(DUMP):
	curl -s http://dumps.wikimedia.org/simplewiki/20140116/$@.gz \
		| zcat | egrep '.{20}' | sort --random-sort > $@

clean:
	rm -f {people,simplewiki}{.lzma,.csv,-sample} $(DUMP)
