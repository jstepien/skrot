.PHONY: all clean
RANDOMISE = ruby -pe '$$_ = "\#{rand} \#{$$_}"' | sort | sed 's/[\.0-9]* //'

all: measure
	+make people.csv simplewiki-20.csv simplewiki-30.csv
	(\
		echo 'summary(read.csv("people.csv"))';\
		echo 'summary(read.csv("simplewiki-20.csv"))';\
		echo 'summary(read.csv("simplewiki-30.csv"))';\
		) | R -q --vanilla

measure: measure.rs ../skr
	rustc -O $< -o $@ -L ..

../skr:
	+make -C .. skr

people.csv: ../skr people.lzma people-sample
	cat people-sample | ./benchmark.rb people.lzma > $@

people-sample:
	./generate_people.rb | head -100 > $@

people.lzma:
	./generate_people.rb | head -50 | ../mkskr > $@

DUMP = simplewiki-20140116-all-titles

simplewiki-20.csv: ../skr simplewiki-20.lzma simplewiki-20-sample
	cat simplewiki-20-sample | ./benchmark.rb simplewiki-20.lzma > $@

simplewiki-20-sample: $(DUMP)-20
	cat $^ | $(RANDOMISE) | head -100 > $@

simplewiki-20.lzma: $(DUMP)-20
	cat $^ | $(RANDOMISE) | head -1000 | ../mkskr > $@

simplewiki-30.csv: ../skr simplewiki-30.lzma simplewiki-30-sample
	cat simplewiki-30-sample | ./benchmark.rb simplewiki-30.lzma > $@

simplewiki-30-sample: $(DUMP)-30
	cat $^ | $(RANDOMISE) | head -100 > $@

simplewiki-30.lzma: $(DUMP)-30
	cat $^ | $(RANDOMISE) | head -1000 | ../mkskr > $@

$(DUMP)-20:
	curl -s http://dumps.wikimedia.org/simplewiki/20140116/$(DUMP).gz \
		| zcat | egrep '.{20}' > $@

$(DUMP)-30: $(DUMP)-20
	cat $< | egrep '.{30}' > $@

clean:
	rm -f people* simplewiki* measure
