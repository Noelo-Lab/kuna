# Go small-map result evidence

`runtime.makemap_small` allocates a new empty map and seeds it. The helper
takes no descriptor, so its call alone cannot identify the source map
type. A direct wrapper's sole Go DWARF result can supply that identity.
[Pinned named-map result predicate](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_go_map_creation.py#L124-L126)

On Go 1.22.10 and 1.27.2, `NewRegistry` has one direct call to the runtime
helper, followed by a return of its unmodified RAX. Its complete DWARF
result refers to `Registry`. The linked runtime descriptor identifies the
same eight-byte map value, its four-byte named key and its eight-byte
named value. The two compiler versions use different private runtime
map implementations. Both provide the same source contract.

| Control | Distinguishing evidence |
| --- | --- |
| `NewArchive` | Identical allocation and return shape. Its different nominal result has no live runtime descriptor in this input. |
| `RawAddress` | The allocation survives, but the declared result exposes its raw pointer. |
| Nil or conditional result | The nil path performs no allocation. |
| Effect after allocation or two allocations | The original effects and their order both matter. |
| Dynamic hint | A different runtime producer receives the descriptor and the live hint. |

Stock can recover the physical RAX return in a single-function view. A
batched view can retain the call and lose the result. An explicit pointer
prototype for the wrapper preserves it on both entry paths. Declaring
only the runtime helper can still leave a selected batched wrapper void.
On main `760a27b63` neither loss reproduces with this fixture: on Go 1.22.10
the single-function run and `decompile-all --functions` selections print
`func NewRegistry() uint64`, the whole-image run prints `*runtime.hmap`, and
declaring only the helper gives `unsafe.Pointer`. Go 1.27.2 prints the
runtime-private `*internal/runtime/maps.Map`. None of them names `Registry`.
The [proposal](proposal.md) keeps physical result recovery and Go spelling
grounded in the same verified allocation.
