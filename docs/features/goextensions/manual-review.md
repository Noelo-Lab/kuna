# Manual review

The native oracle checks ten functions over 1,578 input rows. Every byte value
is present, followed by 16-/32-/64-bit boundaries. Six wide inputs exercise
negative values and wraparound. Signed widening must occur before scaling;
narrow addition and multiplication must still overflow before widening. The
unsigned control is preserved. Signed-to-unsigned conversion retains its bit
pattern. Boolean results become zero or one at both 32 and 64 bits.

The old bodies fail Go type checking. The new bodies are copied unchanged into
an executable that uses the original fixture's input loop. Its stdout and
stderr must equal native execution byte for byte. Four compiler/DWARF inputs
are each executed with both Go 1.22.5 and 1.27.3.

The independent Go AST verifier accepts only inserted integer conversions and
an exact function-literal template for the single-evaluation boolean 0/1
conversion. It preserves all other AST fields, including operators, existing
narrow casts, literal spellings, declaration types and comments. Position
fields and redundant parentheses are ignored; the AST retains the expression
tree they determine. Mutation checks reject arithmetic, precedence, comment,
declaration and existing-cast changes, plus altered boolean mappings.

The first comparison found a canonical boolean conversion that the initial
integer-only verifier did not recognize. Both 32-bit and 64-bit native cases
were added before final validation. The production guard did not change.
C/Rust controls are compared byte for byte. Partial Go type diagnostics are
reviewed separately; unresolved external symbols prevent whole-program checks.
