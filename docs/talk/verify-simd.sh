#!/usr/bin/env bash
set -euo pipefail

talk_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
talk_tmp=$(mktemp -d /tmp/kuna-talk-check.XXXXXX)

clang -O3 -msse2 -mno-avx -fno-unroll-loops \
    -Dbyte_sum=native_byte_sum -Dmain=original_entry \
    -c "$talk_root/docs/talk/byte_sum.c" -o "$talk_tmp/native.o"

jq -r '"#include <stdint.h>\ntypedef uint8_t uint1;\ntypedef uint32_t uint4;\ntypedef uint64_t uint8;\ntypedef int32_t int4;\n" + (.arms.kuna_devectorize.code | sub("byte_sum"; "recovered_byte_sum"))' \
    "$talk_root/docs/talk/simd-evidence.json" > "$talk_tmp/recovered.c"

for talk_cc in gcc clang; do
    for talk_opt in 0 2; do
        "$talk_cc" "-O$talk_opt" \
            "$talk_root/docs/talk/byte_sum_check.c" \
            "$talk_tmp/recovered.c" "$talk_tmp/native.o" \
            -o "$talk_tmp/check-$talk_cc-O$talk_opt"
        printf '%s -O%s: ' "$talk_cc" "$talk_opt"
        "$talk_tmp/check-$talk_cc-O$talk_opt"
    done
done

printf 'Generated files retained at %s\n' "$talk_tmp"
