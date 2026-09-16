CARGO = cargo
NPM = npm

.PHONY: all build test test-e2e clean

all: build

# Rust workspace root: Leptos (WASM) frontend + Axum backend + shared types.
build:
	bash build.sh

test:
	$(CARGO) test --workspace

test-e2e: build
	cd tests/e2e && $(NPM) ci && npx playwright test

clean:
	$(CARGO) clean
