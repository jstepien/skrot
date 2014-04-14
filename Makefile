.PHONY: clean

javadoc/master: classpath skrot
	javadoc -cp $(shell cat classpath) -d $(PWD)/$@ cc.stepien.skrot

skrot:
	git clone $(PWD) skrot -b master || rm -rf $@

classpath: skrot
	cd skrot/java && lein cp $(PWD)/$@

clean:
	-rm -rf skrot classpath javadoc
