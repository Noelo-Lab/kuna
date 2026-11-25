#!/bin/sh
# Rebuild the checked-in fixtures: auth_<isa>_<opt>.elf from auth.c and negatives.c.
set -eu
cd "$(dirname "$0")"
LLVM=${LLVM:-/opt/homebrew/opt/llvm/bin}
FLAGS="-ffreestanding -fno-builtin -fno-stack-protector -fno-pic -fno-jump-tables -fno-unwind-tables -fno-asynchronous-unwind-tables -nostdlib -static -fuse-ld=lld -Wl,--build-id=none -Wl,-e,auth_step"
for isa in x86_64 aarch64; do
  for opt in O0 O2; do
    "$LLVM/clang" --target=$isa-linux-gnu -$opt $FLAGS -o auth_${isa}_${opt}.elf auth.c negatives.c
  done
done
