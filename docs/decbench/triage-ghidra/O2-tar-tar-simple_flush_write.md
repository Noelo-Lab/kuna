---
case_id: O2-tar-tar-simple_flush_write
status: angr-feature-pending
gap_survives: true
recorded_kuna_ged: 59
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: noreturn-robustness
---

## Source (buffer.c:1851, GED oracle)

```c
static void
simple_flush_write (size_t level __attribute__((unused)))
{
  ssize_t status;
  status = _flush_write ();
  if (status != record_size)
    archive_write_error (status);      /* noreturn: loops forever */
  else
    { records_written++; bytes_written += status; }
}
```

`archive_write_error` (sub_da70 @ 0xda70) is genuinely no-return — it contains an
unconditional infinite loop and never falls off the end:

```c
void sub_da70(unsigned long a0) {
  if (dat_82a69 != '\0') goto label_da9d;
  do {
    sub_da10(*dat_82828,a0,dat_82b98);
label_da9d:
    v2 = (void *)__errno_location(); v1 = *v2;
    sub_cc30(); *v2 = v1;
  } while( true );          // <- no exit edge: noreturn
}
```

## Current kuna vs ghidra

Ghidra (13 LOC, 0 gotos, 1 if, 0 loops) — concludes sub_da70 is no-return and truncates,
matching the source exactly (GED 0):

```c
void simple_flush_write(void)
{
  long lVar1;
  lVar1 = FUN_0010c180();                       // _flush_write()
  if (DAT_00182b98 == lVar1) {                  // status == record_size
    DAT_00181f30 = DAT_00181f30 + 1;            // records_written++
    DAT_001821a0 = (double)lVar1 + DAT_001821a0;// bytes_written += status
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010da70(lVar1);                          // archive_write_error(status) [noreturn]
}
```

Current kuna (80 LOC, 2 gotos, 3 labels, 13 ifs, 1 loop) — does NOT conclude sub_da70 is
no-return, so it treats the call as returning and the decoder falls through the call site
straight into the next function's bytes (the fopen / __isoc99_fscanf /
"contains invalid volume number" body of `_gnu_flush_write`), inflating the function:

```c
void sub_dac0(void)
{
  ...
  v2 = sub_c180();
  if (dat_82b98 == v2) {
    dat_81f30 = dat_81f30 + 1;
    dat_821a0 = (float8)v2 + dat_821a0;
    return;
  }
  sub_da70();                 // <- treated as RETURNING; should end the function here
  sub_10cf0();                // <- fall-through garbage: bytes of the NEXT function
  dat_82164 = 0;
  if (dat_81f20 == '\0') goto label_db3c;
  if (dat_82170 == 0) goto label_db3c;
  ...
  v3 = fopen(dat_82a48,"r");
  if ((__isoc99_fscanf(v3,0x669e4,0x7f010) != 1) || (dat_7f010 < 0)) {
    ...
    error(0,0,dcgettext(0,"%s: contains invalid volume number",5),v4);
    sub_16fd0();
  }
  ... (~55 more lines of unrelated body) ...
}
```

## Divergence experiment

The symptom is a no-return over-run (decoder falls through a call that never returns), so I
flipped every no-return-related default-ON lever OFF. None collapse the blob; the
`sub_10cf0`/fopen fall-through survives every time:

| lever off            | LOC | collapsed? |
|----------------------|-----|-----------|
| (default, all on)    | 80  | no        |
| noreturn_propagate   | 79  | no        |
| noreturn_extern      | 80  | no        |
| noreturn_externmatch | 80  | no        |
| tailcalljump         | 160 | no (worse)|

Winning lever: **none.** This gap is not caused by a default-ON divergence being *too
aggressive* — it is the opposite. The fix requires kuna to conclude MORE no-return facts,
which no OFF flip can provide.

## Analysis / verdict

Root cause: kuna's internal-function no-return analysis fails to conclude that
`archive_write_error` (sub_da70) is no-return. That function is no-return because it contains
an **unconditional infinite loop** (`do { … } while(true)` with no break/return edge), not
because it tail-calls a known-no-return extern like `exit`/`abort`. Kuna concludes no-return
for functions that reach a known-no-return *sink*; the "loops forever / cold tail with no
return edge" case is not caught. Because the fact is never established, `noreturn_propagate`
(default-on, and the F1 listing IS enabled on this `decompile-all` surface) has nothing to
propagate — the call to sub_da70 is modeled as returning, the decoder continues past the
call into the adjacent function's bytes, and the function balloons from ghidra's 13 LOC to
80 LOC (GED 59).

This is precisely the `noreturn-robustness` angr-feature bucket: an internal no-return
wrapper still not concluded no-return even with the listing on — specifically the *looping*
sub-case. It is not a runtime choice (kuna is unambiguously worse than ghidra here and there
is no shape where the fall-through is desirable), and it is not toggled by any existing
option. The pending session-2 no-return-robustness work (detect functions with no return
edge — infinite-loop tails, all-paths-reach-noreturn — and mark them no-return so
noreturn_propagate can truncate the caller) closes this case. gap_survives = true.
