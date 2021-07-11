.PHONY: build
all: build
build:
	wasm-pack build --release
	npm install
	npm run env -- webpack --mode production
