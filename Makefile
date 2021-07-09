.PHONY: usage
usage:
	-echo 'Usage: make (test)'

.PHONY: test
test:
	cargo vendor
	cargo build
	cargo test
