# Logical variable lifetimes

The lifetime proposal lets names and types describe individual values even when
native code reuses a register or the same frame bytes. `stackviews` is opt-in;
register assertion fixes also apply with it disabled. Overlapping logical objects
retain one physical backing object, so an escaped pointer still observes later
writes to those bytes.

## Reproducing shared stack storage

Run this self-contained example with an upstream `kuna` executable on x86-64 Linux.
It needs a C compiler with the GNU assembler and matching Kuna processor specs.
The same input and assertions work with the lifetime proposal; add the options
shown below to expose the two logical objects. The reference native result is 67.

```sh
cat >/tmp/reuse.S <<'ASM'
.intel_syntax noprefix
.text
.global reuse, read_pair, read_handle
.type reuse,@function
reuse:
 push rbx
 sub rsp,16
 mov dword ptr [rsp],5
 mov dword ptr [rsp+4],7
 mov rdi,rsp
 call read_pair
 mov ebx,eax
 lea rax,[rip+datum]
 mov [rsp],rax
 mov rdi,rsp
 call read_handle
 add eax,ebx
 add rsp,16
 pop rbx
 ret
.size reuse,.-reuse
.type read_pair,@function
read_pair: mov eax,[rdi]; add eax,[rdi+4]; ret
.size read_pair,.-read_pair
.type read_handle,@function
read_handle: mov rax,[rdi]; mov eax,[rax]; ret
.size read_handle,.-read_handle
.data
.global datum
datum: .long 55
.section .note.GNU-stack,"",@progbits
ASM
cc -nostdlib -no-pie \
  -Wl,--build-id=none,-Ttext=0x401000,-Tdata=0x402000,-e,reuse \
  /tmp/reuse.S -o /tmp/reuse
cat >/tmp/reuse.kuna <<'ASSERT'
typedef struct Pair { int x; int y; };
typedef struct Handle { const int *value; };
prototype reuse int reuse(void)
prototype read_pair int read_pair(const struct Pair *pair)
prototype read_handle int read_handle(const struct Handle *handle)
data 0x402000 int datum
ASSERT
kuna decompile /tmp/reuse reuse --mode reliable --assert-strict \
  --assert @/tmp/reuse.kuna --json
# On this branch, add --option stackviews on --option structdefs on.
```

The JSON exposes `object_401017_1` and `object_40102c_1`, both at stack offset -24, size 8. Add these assertions to obtain the named output above:

```sh
--assert 'name reuse::object_401017_1 initial_pair' \
--assert 'name reuse::object_40102c_1 replacement_handle'
```

The authored native code and the actual generated C return 67 under GCC and Clang at both `-O0` and `-O2`.

## Conservative callee memory proofs

A declared `const` pointer is not evidence that the callee neither writes nor
retains its address. The proposal uses bounded direct-callee evidence and keeps
unknown calls, partial pointer provenance and observable home slots as barriers.
In particular, a pointer loaded from global memory may refer to a future callee's
stack home: an unresolved load or store through it must retain the saved lanes.
Literal global addresses proved disjoint from that home remain a positive control.

The focused regressions live in `saved_xmm_msabi.rs`,
`kuna_calleehomes/tests.rs` and `kuna_calleememory.rs`. The native saved-XMM test
compares original assembly with generated C using GCC and Clang at O0 and O2.

## Performance scope

Earlier release measurements against upstream `919984b8` found small default-mode
CPU differences on three authored fixtures (+0.3%, -0.4%, -0.8%). They are too small
and too narrow to establish a general speed improvement or bound. Enabling
`stackviews` on one seven-function mixed-control-flow workload had a repeatable
+7.6% paired CPU cost (three measured pairs, +6.9% to +8.6%), or 1.13–1.40 extra CPU
seconds per invocation. The feature remains opt-in. These historical results do
not measure the refreshed current-main head. `speed.py` shows the interleaved
measurement method; future broad comparisons can use `scripts/nightly/` with
fixed inputs and matching build/spec pairs.
