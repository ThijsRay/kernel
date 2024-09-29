ARCH := x86_64
EFI_NAME := bootx64.efi
OVMF=/usr/share/OVMF/x64/OVMF.fd

# Set this to 'release' for an optimized build
VERSION := debug
COMPILATION_OPTIONS :=

LOADER_LOCATION=/efi/boot/$(EFI_NAME)
KERNEL_LOCATION=/efi/boot/kernel.elf

ifeq "$(VERSION)" "release"
	COMPILATION_OPTIONS += --release
endif

.PHONY: build
build: build-boot build-kernel

build-boot:
	echo -n 'pub(crate) const KERNEL_LOCATION: &str = "$(KERNEL_LOCATION)";' > boot/src/location.rs
	cd boot && cargo build $(COMPILATION_OPTIONS)

build-%:
	cd $* && cargo build $(COMPILATION_OPTIONS)

.PHONY: fmt
fmt: fmt-boot fmt-kernel
fmt-%:
	cd $* && cargo fmt

.PHONY: clean
clean:
	cargo clean

efi: build
	mkdir -p target/esp/efi/boot
	cp target/$(ARCH)-unknown-uefi/$(VERSION)/boot.efi target/esp$(LOADER_LOCATION)
	cp target/$(ARCH)/$(VERSION)/kernel target/esp$(KERNEL_LOCATION)

.PHONY: run
run: efi
	qemu-system-$(ARCH) -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=$(OVMF) \
    -drive format=raw,file=fat:rw:target/esp

.PHONY: gdb
gdb:
	gdb -ex "target remote localhost:1234"
