ARCH := x86_64

.PHONY: build

build:
	cargo build --target arch/$(ARCH).json
