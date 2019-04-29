.NOCACHE: fmt
fmt:
	set -eux
	cargo fmt

.NOCACHE: lint
lint:
	set -eux
	touch src/main.rs && cargo clippy --all-features --all-targets -- -D clippy::all -D warnings

.NOCACHE: test
test:
	set -eux
	RUST_BACKTRACE=1 cargo test

