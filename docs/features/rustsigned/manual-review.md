# Signed primitive emission review

The signed IR operation determines interpretation. Core integer inputs and exact
8-, 16-, 32- or 64-bit widths bound the plan. Unsigned inputs and signed text
storage receive a signed cast; arithmetic shift results return to unsigned
storage when required even if the input already prints signed. Unsigned literals
keep an inner storage cast to avoid signed literal overflow during inference.
Declarations and shift counts stay unchanged, each operand prints once, and
compound assignment is expanded when it cannot represent that interpretation.
Non-core aliases, pointers, unsupported widths, unrelated operators, explicit
cast suppression and other output languages retain their behavior.

All 255 changed corpus bodies pass the independent AST comparison: only integer
casts and redundant parentheses differ, every existing cast remains at its
expression node, and other AST fields and comments match. There are 10,167 i8
casts, three u8 casts, three i64 casts and one u32 cast. Ten unchanged namespace
placeholders receive identical parser-only repairs. The three i64 casts are
redundant in two C-source functions whose declarations already spell n as i64;
their original signed jg/jle branches and source were inspected. The u32 cast
repairs an arithmetic-shift assignment to v50 in the already incomplete recovered
default_alloc_error_hook body. The raw unsigned 32-bit loop independently tests
that conversion. The full scan and single-function CLI agree on 396 casts in
core::fmt::Formatter::pad at 0x13df0.

Function metadata outside source positions, variable declarations, per-variable
addresses and global instruction address sets are exact. All 41,461 C/Go records
remain identical. Existing panic placeholders for unstructured control flow
remain; this does not claim complete runtime-body translation.

The partial compiler audit covers all 255 changed functions. Selected type,
constant and overflow diagnostics introduce no new message or compilation
failure: 201 functions have diagnostics in each arm, with 10,506 before and
10,505 after. One standalone function compiles in both arms. The sole diagnostic
difference removes one E0308 from the corrected u32 shift assignment. Unresolved
external names and existing recovery gaps limit this audit.

Eight standalone native Rust fixtures execute unchanged under three producers
and three executors: 4,718,592 returned results and 589,824 single-call checks
match. Both signed and unsigned prototypes of an in-place byte loop match the
exact native instruction bytes on all 8,192 byte/count inputs under each of
three compilers; old bodies differ on 3,968 inputs. Nine additional raw functions
cover unsigned declared inputs across four widths and 71,824 input pairs.
Their machine-code oracle agrees with three independent Rust source builds.
Unedited corrected bodies match all 1,939,248 results; the old complete programs
fail compilation on their wider shift assignments. Wider comparison controls
retain their correct behavior.
