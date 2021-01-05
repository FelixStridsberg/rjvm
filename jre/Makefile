# NOTE: javac must be java 8, later version conflicts when trying to compile
# jre classes.

CC=javac

JAVA_FILES=$(shell find . -name *.java)
CLASS_FILES=$(JAVA_FILES:.java=.class)

%.class: %.java
	$(CC) $<

.PHONY: compile
compile: $(CLASS_FILES)
