rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build:
	cargo build

run:
	cargo run				# won't work, cargo run -- play --name "Marco"   # where "Marco" is 2nd arg after play, supplided ater --name

release:
	cargo build --release

all: format lint test build
