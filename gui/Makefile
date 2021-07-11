.PHONY: build
all: build
build:
	wasm-pack build --release
	npm install
	npm run env -- webpack --mode production
	sed -e "s/WebAssembly.instantiateStreaming/undefined/g" dist/bundle.js > dist/bundle-fix.js
	mv dist/bundle-fix.js dist/bundle.js
