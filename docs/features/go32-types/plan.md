# Read Go runtime types at the target pointer width

Extend the existing `gotypes` importer to positively identified Go 1.18–1.25
little-endian 32-bit images. Read runtime words, base records, field arrays and
header shells using the target width. Keep the 64-bit descriptor-region reader
restricted to its validated layout. Use the existing `gotypes off` ablation.

Exercise actual stripped Go 1.18.1 and 1.22.5 executables on i386, ARM, amd64
and arm64. Check application integers, strings, embedded pointers, nested value
fields and the public Linux `syscall.Iovec` layout. Compare whole binaries and
type declarations, including Go 1.22 and 1.27 amd64 controls, then run every
repository gate and measure the final implementation against unmodified main.
