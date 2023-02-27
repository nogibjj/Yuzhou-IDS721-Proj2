rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format-check:
	cd actix-translator && cargo fmt --quiet

lint:
	cd actix-translator && cargo clippy --quiet

test:
	cd actix-translator && cargo test --quiet

run:
	cd actix-translator && cargo run

build-release:
	cd actix-translator && cargo build --release

all: format lint test run
