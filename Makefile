.NOCACHE: fmt
fmt:
	cargo fmt

.NOCACHE: lint
lint:
	cargo fix --allow-dirty --allow-staged
	# `rustup target add` may exit with error if target already added
	# rustup target add x86_64-apple-darwin || true
	# cargo check --all-features --target x86_64-apple-darwin
	# rustup target add x86_64-pc-windows-msvc || true
	# cargo check --all-features --target x86_64-pc-windows-msvc
	# rustup target add x86_64-unknown-linux-gnu || true
	# cargo check --all-features --target x86_64-unknown-linux-gnu
	cargo check --all-features --benches --tests
	cargo clippy --all-features --benches --tests -- --deny clippy::all

.NOCACHE: test
test:
	RUST_BACKTRACE=1 cargo test

