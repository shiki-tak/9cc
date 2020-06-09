build:
	cargo build --release

test: clean build
	./test.sh

clean:
	cargo clean
	rm -f *~ tmp*

.PHONY: test clean