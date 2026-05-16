.PHONY: setup run build test fix verify fmt lint audit deny migrate clean dev

setup:
	chmod +x scripts/*.sh
	chmod +x .githooks/*
	git config core.hooksPath .githooks
	@echo "Setup complete"

run:
	cargo run -p api

dev:
	cargo run -p api

build:
	cargo build --workspace --all-targets --all-features

test:
	cargo test --workspace --all-features

fix:
	./scripts/fix.sh

verify:
	./scripts/verify.sh

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

audit:
	cargo audit

deny:
	cargo deny check

migrate:
	sqlx migrate run

clean:
	cargo clean