# Names in C type definitions

The C header declared the DWARF type `elm.tag` as `elm_tag`, then used
`elm.tag *p` in a member declaration. A C11 compiler rejected the member.
The same mismatch reached arrays of pointers and pointers to arrays.

C type definitions now spell their named declarator bases with the same
alias as the corresponding definition. The existing C declarator walk
still controls qualifiers and pointer, array, and function precedence.
The context applies to members and typedef bases. Ordinary declarations
and Go and Rust definitions keep their existing naming contexts.

The independent DWARF witness uses a small relocatable object with a
same-length name replacement. The compiler accepts the fixed header with
eight static size, offset, or extent assertions. The focused Rust test
also verifies qualifiers on both a base and a pointer and preserves a
named typedef through an array declarator.
