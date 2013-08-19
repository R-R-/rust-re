lib: src/re.rs src/re.rc
	rustc src/re.rc

check:
	rustc tests/success.rs

.PHONY: lib check
