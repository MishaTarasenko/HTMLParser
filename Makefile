# Makefile

.PHONY: build run test fmt clippy release clean

PROJECT_NAME = html_parser

build:
	cargo build --release

run:
	cargo run --release -- parse example.html

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

release:
	cargo build --release

clean:
	cargo clean

commit: fmt clippy test
	git add .
	git commit -m "Automated commit after formatting, linting, and testing."


