.NOCACHE: fmt
fmt:
	cargo fmt

.NOCACHE: lint
lint:
	# cargo check --all-features --target x86_64-apple-darwin
	# cargo check --all-features --target x86_64-pc-windows-msvc
	cargo check --all-features --target x86_64-unknown-linux-gnu
	cargo clippy -- --deny clippy::all

.NOCACHE: test
test:
	cargo test

