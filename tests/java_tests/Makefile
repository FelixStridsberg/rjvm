CC=javac

JAVA_FILES=$(shell find . -name '*.java')
CLASS_FILES=$(JAVA_FILES:.java=.class)

%.class: %.java
	$(CC) -cp ../java_lib:../../jre:.. $<

.PHONY: all
all: compile_java_lib compile

.PHONY: compile_java_lib
compile_java_lib:
	cd ../java_lib && $(MAKE) compile

.PHONY: compile
compile: $(CLASS_FILES)

clean:
	rm ./*.class
