#!/bin/bash
set -e
EFI=$1
DIR=$(dirname "$EFI")

mkdir -p "$DIR/esp/efi/boot"
cp "$EFI" "$DIR/esp/efi/boot/bootx64.efi"

OVMF_CODE="/usr/share/OVMF/x64/OVMF_CODE.fd"
OVMF_VARS="/usr/share/OVMF/x64/OVMF_VARS.fd"
QEMU="$(which qemu-system-x86_64)"



qemu_args=(
  # Open a gdbserver on TCP port 1234
  -s
  # Don't start the CPU until the gdb client says continue
  -S

  # Enable KVM
  -enable-kvm

  # Amount of memory
  -m 512M

  # Load OVMF
  -drive
  "if=pflash,format=raw,readonly=on,file=$OVMF_CODE"
  -drive
  "if=pflash,format=raw,readonly=on,file=$OVMF_VARS"

  # Load the EFI in a FAT filesystem
  -drive
  "format=raw,file=fat:rw:$DIR/esp"
)

${QEMU} ${qemu_args[@]}
