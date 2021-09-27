#!/bin/bash
set -e

qemu-system-x86_64 \
    -enable-kvm \
    -m 128 \
    -nographic \
    -bios /usr/share/OVMF/x64/OVMF_CODE.fd \
    -device driver=e1000,netdev=n0 \
    -netdev user,id=n0,tftp=target/x86_64-unknown-uefi/release,bootfile=uefios.efi
