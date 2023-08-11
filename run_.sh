#!/bin/bash
# name to run_.sh to prevent make run would run run.sh
set -exu

cd os

<<- EOF
[build]
target = "riscv64gc-unknown-none-elf"
[target.riscv64gc-unknown-none-elf]
rustflags = [
    "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes"
]
EOF
cargo clean
cp src/linker-qemu.ld src/linker.ld
cargo b

# objcopy --only-section copy code/data section only?
target=target/riscv64gc-unknown-none-elf/debug
# objcopy --remove-section .eh_frame_hdr --remove-section .eh_frame --remove-section .comment --remove-section .note --remove-section .note.gnu.build-id --remove-section .gnu.hash --remove-section .gnu.version --remove-section .gnu.version_d --remove-section .gnu.version_r --remove-section .interp --remove-section .dynsym --remove-section .dynstr --remove-section .rela.dyn --remove-section .rela.plt --remove-section .plt --remove-section .got --remove-section .got.plt --remove-section .data --remove-section .bss --remove-section .rodata --remove-section .data1 --remove-section .text --remove-section .shstrtab --remove-section .symtab --remove-section .strtab --remove-section .dynsymtab --remove-section .dynstrtab --remove-section .gnu_debuglink --remove-section .gnu_debugdata <input_file> <output_file>
riscv64-unknown-elf-objcopy $target/os --strip-all  -O binary $target/os.bin

qemu-system-riscv64 -machine virt -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=$target/os.bin,addr=0x80200000
