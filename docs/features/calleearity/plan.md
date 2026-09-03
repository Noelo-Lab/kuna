# calleearity — design

**Slug / option:** `calleearity` (on|off) · **Phase:** P4 ·
**Sub-stage:** trial-finalization · **ElementId:** 4133 ·
**Arch flag:** `callee_arity`

## The one-line change

At `FuncCallSpecs::build_input_from_trials`, before the CALL op's input list is
written, reconcile this call's `used` trials with the argument storage a *sibling*
call to the same callee entry address already finalized.

## Where it hooks

```
ActionActiveParam                                   (per call spec, qlst order)
  check_input_trial_use          -> active / inactive / no-use per trial
  resolve_and_derive_input_map   -> fillinMap sets `used`
  build_input_from_trials
 +  kuna_calleearity::unify_with_sibling_call       <- the new decision
    op_set_all_input                                 (the argument list)
 +  fc.set_final_input_storage(...)                 <- the witness record
    activeinput.delete_unused_trials()
  clear_active_input()
```

## The witness record, and why the op's inputs are not it

The obvious implementation — read a sibling CALL op's inputs and copy their
addresses — is wrong, and measurably so: at `0x14000252c` the recovered argument
is the constant `0x28`, whose Varnode lives in the *constant* space, not in `rcx`.
A CALL input carries an argument's **value**; after constant propagation and
Varnode merging that value is routinely a constant or a temporary. The **storage**
is only ever in the `ParamTrial`, and the trials are deleted one line later.

So `build_input_from_trials` records the storage of each used trial on the call
spec (`FuncCallSpecs::final_input_storage`, a new `(Address, size)` vector) at the
moment it writes the list. That vector is the witness, and it is the only new
state this feature adds.

## The four limits, and what each one is for

| Limit | Prevents |
|---|---|
| register storage only | comparing stack slots whose *caller*-relative addresses differ per call site by that site's own stack offset |
| a real value in the trial's input slot; never a synthetic unreferenced trial | printing a read of a register nothing at this site defines |
| all-or-nothing over the whole witness | a non-contiguous parameter list (arg 2 present, arg 1 missing) |
| never subtractive | trusting a witness that is itself the site that lost an argument |

Plus the two gates that keep it off the normal path: the option, and
`!fc.is_input_locked()` — a callee with a declared prototype has the same argument
list at every site already.

## The limit the design accepts

`ActionActiveParam` finalizes each spec as soon as *that* spec is fully checked,
walking `qlst` in order, so only sites **earlier** in the function can be
witnesses. Making the direction symmetric requires deferring every finalization
until every spec is checked — which changes what `Funcdata::check_call_double_use`
sees while it is still scoring trials, i.e. a change to trial scoring itself
rather than to finalization. That is a bigger option than this one; it is recorded
in `analysis.md` rather than smuggled in here.

## Why not relax `only_op_use` instead

The direct fix for the witness is to stop `only_op_use` rejecting on a
`CPUI_CBRANCH` that is reached only through comparison/boolean ops — the value is
*tested*, not consumed as data. It was rejected on the refutation question ("would
this produce WRONG output?"), by inspection of the shape rather than by argument:
`mov rcx,[rbx]; test rcx,rcx; je skip; call foo` is structurally identical to the
witness and appears in essentially every x86-64 binary, so the relaxation would
fabricate a trailing argument at each one. No local dataflow property separates
the two. The callee does, and the sibling call site is where the callee's opinion
is already recorded.
