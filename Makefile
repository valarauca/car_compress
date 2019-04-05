.PHONY: test
test:
	cargo test
	cargo fmt -- -f
	rm -rf src/*.rs.bk
	cd algo/flate2-rs && cargo test && cargo clippy
	cd algo/brotli2-rs && cargo test && cargo clippy
