NAME=gen

.PHONY: run
run: lint
	cargo run

.PHONY: build
build: lint
	cargo build

.PHONY: test
test:
	cargo test -- --nocapture

.PHONY: release
release: lint
	cargo build --release

.PHONY: watch
watch:
	cargo watch -x "clippy; cargo run"

.PHONY: clean
clean:
	cargo clean

.PHONY: install
install:
	@set -e
	@if [ $$(id -u) -eq 0 ]; then echo "Do not run as root"; exit 1; fi

	cargo install --path .
	if [ ! -d ~/.config/gen/ ]; then mkdir ~/.config/gen; fi
	cp -r templates ~/.config/gen/

.PHONY: publish
publish:
	cargo publish

.PHONY: fmt
fmt:
	rustfmt **/*.rs

.PHONY: lint
lint:
	cargo clippy
