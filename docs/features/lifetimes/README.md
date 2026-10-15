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

## Reproducing detached member emission

This neutral tree fixture keeps one frame range live through both node-pointer and pointer-slot accesses. Run from the repository root with a built `kuna` executable on `PATH` and matching x86 specs in `specs/`:

```sh
fixture=decompiler/crates/kuna-analysis/tests/fixtures/tree_range_members.c
image=/tmp/kuna-member-tree-range.elf
gcc -O2 -no-pie -fno-stack-protector -fcf-protection=none "$fixture" -o "$image"
tree_range_addr=$(nm -n "$image" | awk '$3 == "tree_range" { print $1 }')
consume_range_addr=$(nm -n "$image" | awk '$3 == "consume_range" { print $1 }')
tree_range_addr=$(printf '%x' "0x$tree_range_addr")
consume_range_addr=$(printf '%x' "0x$consume_range_addr")
kuna decompile "$image" tree_range \
  --target x86:LE:64:default:windows --mode reliable --json --assert-strict \
  --option structdefs on --option stackviews on --option stackalias off \
  --sleighpath specs \
  --assert 'typedef struct Node { struct Node *left; struct Node *parent; struct Node *right; unsigned char color; bool sentinel; unsigned char padding[2]; int key; int tag; };' \
  --assert 'typedef struct Tree { Node *head; unsigned long long size; };' \
  --assert 'typedef struct Range { Node *first; Node *last; };' \
  --assert "prototype 0x${tree_range_addr} void tree_range(Tree *tree,int key,int tag)" \
  --assert "prototype 0x${consume_range_addr} void consume_range(Tree *tree,const Range *range)" \
  | jq -r '.functions[0].code | split("\n")[] | select(test("->sentinel|&v5->left"))'
```

Without the member-base cast, this printed member expressions such as `v1 = v3->sentinel;` while `v3` is declared `uint8`, and `&v5->left` while `v5` is `Node **`. GCC reports `invalid type argument of '->'`; Clang reports that the member-reference base is not a pointer or a structure. The emitter now prints `v1 = ((Node *)v3)->sentinel;` and `&((Node *)v5)->left`. The focused integration test compiles and executes the generated C with GCC and Clang at `-O0` and `-O2`, with stack views enabled and disabled.

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

## Reproducing disjoint private spills

This neutral x86-64 Linux input uses a Win64 helper. The assembly saves XMM6–XMM9
in the caller's frame, then passes a separate 16-byte rectangle to a helper
whose four writes do not overlap those saves. The checked-in source files are
[`savedxmm_msabi.S`](../../../decompiler/crates/kuna-analysis/tests/fixtures/savedxmm_msabi.S)
and [`savedxmm_msabi.c`](../../../decompiler/crates/kuna-analysis/tests/fixtures/savedxmm_msabi.c).
With Clang and matching x86-64 specs, build and decompile it with:

```sh
cargo build --manifest-path decompiler/Cargo.toml -p kuna-cli
kuna_bin=decompiler/target/debug/kuna
image=/tmp/savedxmm-clang-o0
clang -std=c11 -O0 -no-pie \
  -Wl,--section-start=.saved_xmm_rect=0x700000 \
  decompiler/crates/kuna-analysis/tests/fixtures/savedxmm_msabi.S \
  decompiler/crates/kuna-analysis/tests/fixtures/savedxmm_msabi.c \
  -o "$image"
"$image"
symbol() { nm -n "$image" | awk -v wanted="$1" '$3 == wanted { print "0x" $1; exit }'; }
target_addr=$(symbol fixture_spills_rect)
copy_addr=$(symbol fixture_copy_rect)
rect_addr=$(symbol rect_source)
"$kuna_bin" decompile "$image" fixture_spills_rect \
  --target x86:LE:64:default:windows --mode reliable --json --assert-strict \
  --option stackviews on --option structdefs on --sleighpath specs \
  --assert 'typedef struct Rect { float x; float y; float width; float height; };' \
  --assert "prototype $target_addr float fixture_spills_rect(void)" \
  --assert "prototype $copy_addr void fixture_copy_rect(Rect *destination,const Rect *source)" \
  --assert "data $rect_addr Rect rect_source"
```

On the alias-only baseline, the emitted function contains sixteen referenced
`unsigned int` locals for the save lanes, including:

```c
unsigned int v18; // stack - 0x48 | xmm6_da
unsigned int v19; // stack - 0x44 | xmm6_db
/* ... */
unsigned int v32; // stack - 0x10 | xmm9_dc
unsigned int v33; // stack - 0xc | xmm9_dd
```

With the private-spill proof, those locals and their copy assignments disappear
from emitted code; the caller-owned rectangle remains:

```c
Rect v1 [2]; // stack - 0x68 | xmm0_da
fixture_copy_rect(v1,&rect_source);
return v1[0].x + v1[0].y;
```

The native fixture checks all sixteen seeded XMM values and the 3.75 result.

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
