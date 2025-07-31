# === rsqr Makefile ===

CARGO=cargo
BIN=rsqr

.PHONY: build run test qa completions clean release

build:
	$(CARGO) build

run:
	$(CARGO) run -- "Hello from Makefile"

test:
	$(CARGO) test

release:
	$(CARGO) build --release

qa:
	./qa.sh

completions:
	@echo "Generating shell completions..."
	$(CARGO) build

clean:
	$(CARGO) clean
	rm -f *.png
	rm -rf completions
