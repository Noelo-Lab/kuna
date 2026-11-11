# Preserve Go integer widening

Keep Go out of the C implicit-promotion shortcut in extension emission. The
existing Go cast renderer must preserve both the narrow signed interpretation
and the wider arithmetic result. Boolean widening uses the existing explicit
single-evaluation conversion to zero or one. This is a strict spelling
correction under outlang Go; C and Rust behavior stays covered by controls.

Use ten native functions to compare emitted bodies with compiler execution
across every byte value, wider boundaries and wide wraparound inputs. Include
signed and unsigned results, narrow overflow before widening, and both 32-bit
and 64-bit boolean results. Compile the unchanged bodies under Go 1.22 and 1.27
for compressed and plain DWARF producers. A raw stage test must fail on the
baseline and pass with the correction.

Compare the full sixteen-image corpus. Independently match parsed Go expression
trees, permitting only added integer conversions or the exact boolean 0/1
wrapper. Check diagnostic differences with Go's type checker and record its
limits. Complete the repository gates and quiet paired speed measurements.

Baseline: 34bfd388764c6291089183ef7d9c305c18c68826; merged unchanged as
76905a15ee74fbd97b298ee6fc0a33082156d8bd.
