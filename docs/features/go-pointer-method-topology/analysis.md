# Go pointer receiver evidence

The [proposal](proposal.md) contains the reproduction. The issue reproduces
on Linux AMD64 with Go 1.22.10 and 1.27.2. The two types of record needed
for a method declaration already exist in the tested binaries.

| Function | Source and runtime evidence |
| --- | --- |
| Pointer method | The pointee's runtime method table identifies the exact function entry in both call forms. DWARF puts the named pointer receiver in RAX. The body reads Base at offset 0 and updates Total at offset 8. |
| Receiver-like ordinary function | The name, pointer parameter and field offsets match the method shape. Its entry has no matching method-table record. |
| Generated pointer wrapper | The wrapper has a pointer parameter and table references. Its function ID records a wrapper, and the value type identifies the genuine method entry. |
| Ordinary call to the pointer method | The call passes the pointer in RAX and the scalar in RBX. Explicit call-site prototypes apply at that address. |

These controls motivate joining type, function and physical storage proof
before projecting the Go receiver position. The first implementation
targets the direct declaration and the matching direct call together.
