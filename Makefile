.PHONY: test
test:
	cargo test
	cargo fmt -- -f
	rm -rf src/*.rs.bk
	cd algo/flate2-rs && cargo test && cargo clippy
	cd algo/brotli2-rs && cargo test && cargo clippy

.PHONY: doc
doc: docs
.PHONY: docs
docs:
	cargo doc -j4 -q
	cp docs/index.html target/doc/index.html
	rm -rf docs
	mv target/doc docs

