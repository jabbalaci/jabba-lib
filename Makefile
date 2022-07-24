cat:
	cat Makefile

# single-threaded testing because of the clipboard
test:
	cargo test -- --test-threads=1

doc:
	cargo doc --open
