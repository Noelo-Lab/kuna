# Bounded extraction from Go byte-array locals

A native pair(x) returning two int64 values already decompiles to an executable
[16]byte result. Its caller still uses SUB168(v1,0)*3+SUB168(v1,8). Preserve the
producer call and recovered storage ABI while rendering each extraction as a
typed memory read.

Implement the eligibility and offset planner in P9's kuna_gobyteextract module.
Require a whole explicit nonvolatile local byte array whose actual declaration
and value type agree with the input size. Reuse local_byte_array_name for the
real declaration and complete symbol extent. Never address an implied call,
global, constant or unsupported value. Permit one-, two-, four- and eight-byte
integer results at constant bounded offsets, reversing the low-end offset for
big-endian storage. Direct stores use the rendered destination pointee's
signedness, while checking both the facing and printed types.

The existing outlang Go selection owns this spelling correction. Add a raw
stage regression plus native caller/callee, unsigned single-call and guarded
store execution checks. Run emitted bodies unchanged under Go 1.22 and 1.27
with boundary and wraparound inputs. Independently verify endian reads and
invalid bounds/types. Compare all output tokens across the image corpus,
including C/Rust controls, and inspect partial Go type diagnostics. Complete
the repository checks and paired speed measurements before publishing.

Source tuples and generalized nonaddressable expressions are separate work.
