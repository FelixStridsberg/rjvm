

test_java:
	cd tests/java_tests/ && $(MAKE)
	cargo test run_java_tests

test_java_debug:
	cd tests/java_tests/ && $(MAKE)
	cargo test run_java_tests --features debug
