.PHONY: test
test:
	cargo test
	cargo fmt -- -f
	rm -rf src/*.rs.bk
	cd algo/bzip2-rs && cargo test
	cd algo/gzip2-rs && cargo test
	cd algo/brotli2-rs && cargo test
