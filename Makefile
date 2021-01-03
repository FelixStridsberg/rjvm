

test_java:
	cd tests/java_tests/ && $(MAKE)
	cargo test run_java_tests
