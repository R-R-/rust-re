lib: src/re.rs src/re.rc
	rustc --out-dir . src/re.rc

check: lib
	rustc -L . --out-dir . tests/success.rs
	./success
	rm success

.PHONY: lib check
