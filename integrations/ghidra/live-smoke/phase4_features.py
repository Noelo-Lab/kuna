#!/usr/bin/env python3
"""Live Phase-4 feature verification: drive the NATIVE Ghidra GUI features the
full `decompileAt` response encode lights up, against a REAL Ghidra with
`kuna_ghidra` swapped in as the decompiler core.

What it proves (each section prints PASS/FAIL and lands in PHASE4_REPORT.md):

  1. signature   — the recovered prototype renders (model + return type +
                   parameters from the `<prototype>` + `<localdb>` cat-0
                   symbols), and `getC()` carries CLEAN type spellings (no
                   `unsigned_long__`-style `IllegalCharCppTransformer` mangle).
  2. symbols     — the HighFunction's LocalSymbolMap decoded params + locals
                   (ids, storage) from kuna's `<localdb>`/`<highlist>`.
  3. rename      — the KEYSTONE: `HighFunctionDBUtil.updateDBVariable` renames
                   a kuna-delivered local HighSymbol; verified against the DB
                   variable AT THE SAME STORAGE (a wrong symbol id silently
                   renames the wrong thing), then re-decompiled to show the
                   new name in the C.
  4. retype      — `updateDBVariable(sym, None, dtype)` on the same symbol.
  5. param rename— rename a parameter (cat-0 + index + exact storage: a
                   mismatch forces a whole-signature commit — r2 §4).
  6. jumptables  — `toggleJumpLoads` + `toggleSyntaxTree(false)` +
                   `toggleCCode(false)` (the switch-analyzer configuration):
                   `HighFunction.getJumpTables()` decodes kuna's
                   `<jumptablelist>` (cases + loadtables), and
                   `DecompilerSwitchAnalysisCmd` re-creates the jump
                   references from it after they are deleted.
  7. paramid     — the `paramid` action + parammeasures toggle:
                   `DecompileResults.getHighParamID()` decodes kuna's
                   `<parammeasures>`.

Environment: as kuna_vs_stock.py (GHIDRA_INSTALL_DIR, KUNA_GHIDRA_EXE,
KUNA_SMOKE_BINARY, KUNA_SMOKE_OUT), plus KUNA_SMOKE_FUNCTION (default: main).
"""

import os
import re
import sys
import traceback

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
KUNA_GHIDRA = os.environ.get(
    "KUNA_GHIDRA_EXE",
    os.path.join(REPO, "decompiler", "target", "release", "kuna_ghidra"),
)
BINARY = os.environ.get("KUNA_SMOKE_BINARY")
FUNCTION = os.environ.get("KUNA_SMOKE_FUNCTION", "main")
OUT = os.path.abspath(os.environ.get("KUNA_SMOKE_OUT", "live-phase4-out"))

REPORT = []


def log(line=""):
    print(line)
    REPORT.append(line)


def check(name, ok, detail=""):
    log(f"[{'PASS' if ok else 'FAIL'}] {name}" + (f" — {detail}" if detail else ""))
    return ok


def main():
    if not BINARY:
        print("usage: KUNA_SMOKE_BINARY=/path/to/prog python3 phase4_features.py")
        sys.exit(1)
    if not os.path.exists(KUNA_GHIDRA):
        print(f"FATAL: kuna_ghidra not built at {KUNA_GHIDRA}")
        sys.exit(1)
    os.makedirs(OUT, exist_ok=True)
    # The probe MUTATES the program database (renames/retypes/reference
    # edits); a reused project would accumulate them and skew every check.
    # Always analyze into a fresh project.
    import shutil
    shutil.rmtree(os.path.join(OUT, "ghidra_project"), ignore_errors=True)

    import pyghidra
    pyghidra.start()
    from ghidra.app.decompiler import DecompInterface, DecompileProcessFactory
    from ghidra.program.model.pcode import HighFunctionDBUtil
    from ghidra.program.model.symbol import SourceType
    from ghidra.program.model.data import AbstractIntegerDataType
    from ghidra.util.task import ConsoleTaskMonitor

    field = DecompileProcessFactory.class_.getDeclaredField("exepath")
    field.setAccessible(True)

    failures = 0
    with pyghidra.open_program(
        BINARY,
        project_location=os.path.join(OUT, "ghidra_project"),
        project_name="kuna_phase4",
        analyze=True,
    ) as flat:
        program = flat.getCurrentProgram()
        monitor = ConsoleTaskMonitor()
        fm = program.getFunctionManager()
        func = None
        for f in fm.getFunctions(True):
            if str(f.getName()) == FUNCTION:
                func = f
                break
        if func is None:
            print(f"FATAL: no function named {FUNCTION}")
            sys.exit(1)

        field.set(None, KUNA_GHIDRA)
        log(f"# Phase-4 live feature report — {os.path.basename(BINARY)}::{FUNCTION}")
        log(f"core: {KUNA_GHIDRA}")
        log()

        # ---- 1+2: full decompile: signature, symbols, clean spellings ------
        from ghidra.app.decompiler import DecompileOptions
        ifc = DecompInterface()
        ifc.setOptions(DecompileOptions())  # the GUI always sends options
        assert ifc.openProgram(program), ifc.getLastMessage()
        res = ifc.decompileFunction(func, 120, monitor)
        if not check("decompileCompleted", res.decompileCompleted(),
                     str(res.getErrorMessage())):
            failures += 1
        hf = res.getHighFunction()
        dfunc = res.getDecompiledFunction()
        c = str(dfunc.getC()) if dfunc else ""
        with open(os.path.join(OUT, f"kuna_{FUNCTION}.c"), "w") as fh:
            fh.write(c)
        sig = str(dfunc.getSignature()) if dfunc else ""
        proto = hf.getFunctionPrototype()
        log(f"signature: {sig}")
        log(f"prototype: model={proto.getModelName()} extrapop={proto.getExtraPop()} "
            f"ret={proto.getReturnType()} nparams={proto.getNumParams()}")
        if not check("signature renders with parameters",
                     bool(sig) and proto.getNumParams() >= 1, sig):
            failures += 1
        # The type-token mangle signature specifically ("unsigned long *" as
        # ONE <type> token reads back as `unsigned_long__…`) — NOT the
        # legitimate double underscores of Ghidra's string-literal symbols.
        mangles = re.findall(
            r"\b(?:unsigned|signed)_\w+"
            r"|\b(?:u?long|u?int|u?short|u?char|byte|bool|float|double"
            r"|void|code|undefined\d*)__\w*",
            c,
        )
        if not check("getC() carries no IllegalChar-mangled type tokens",
                     not mangles, f"{mangles[:5]}"):
            failures += 1

        lsm = hf.getLocalSymbolMap()
        params = [lsm.getParamSymbol(i) for i in range(lsm.getNumParams())]
        locals_ = [s for s in lsm.getSymbols() if not s.isParameter()]
        log(f"localdb: {lsm.getNumParams()} params, {len(locals_)} locals")
        for p in params:
            log(f"  param {p.getCategoryIndex()}: {p.getName()} @ {p.getStorage()} "
                f"id={p.getId():#x} type={p.getDataType()}")
        for s in locals_[:8]:
            log(f"  local: {s.getName()} @ {s.getStorage()} id={s.getId():#x} "
                f"type={s.getDataType()}")
        if not check("locals decoded as HighSymbols", len(locals_) >= 1):
            failures += 1

        # ---- 3: the rename keystone ----------------------------------------
        # Prefer stack storage, then a >=2-byte register (a 1-byte flag local
        # is renameable but a poor retype target), then anything sane.
        def storage_rank(s):
            st = s.getStorage()
            if st is None or st.isBadStorage() or not str(s.getName()):
                return None
            if st.isStackStorage():
                return 0
            if st.isRegisterStorage() and st.size() >= 2:
                return 1
            return 2

        ranked = sorted(
            (s for s in locals_ if storage_rank(s) is not None),
            key=storage_rank,
        )
        target = ranked[0] if ranked else None
        if target is None:
            failures += 1
            check("rename: a renameable local exists", False)
        else:
            old_name = str(target.getName())
            storage = target.getStorage()
            pcaddr = target.getPCAddress()
            new_name = "kuna_ren_probe"
            # The collateral set must be non-trivial to mean anything: on a
            # fresh analysis the DB has no committed locals, so ALSO commit the
            # decoded locals first (HighFunctionDBUtil.commitLocalNamesToDatabase
            # is what the GUI does on any local edit) — then a stray rename has
            # something to collide with.
            tid0 = program.startTransaction("phase4 commit locals")
            try:
                HighFunctionDBUtil.commitLocalNamesToDatabase(hf, SourceType.ANALYSIS)
            except Exception:
                log(traceback.format_exc())
            finally:
                program.endTransaction(tid0, True)
            before = {str(v.getName()) for v in func.getAllVariables()}
            if not check("DB carries committed locals to detect collateral renames",
                         len(before) >= 2, f"{len(before)} DB variables"):
                failures += 1
            tid = program.startTransaction("phase4 rename")
            err = None
            try:
                HighFunctionDBUtil.updateDBVariable(
                    target, new_name, None, SourceType.USER_DEFINED)
            except Exception:
                err = traceback.format_exc()
            finally:
                program.endTransaction(tid, err is None)
            if err:
                log(err)
            hit = [v for v in func.getAllVariables()
                   if str(v.getName()) == new_name]
            same_storage = bool(hit) and str(hit[0].getVariableStorage()) == str(storage)
            after = {str(v.getName()) for v in func.getAllVariables()}
            collateral = (before - {old_name}) - after
            if not check(
                "rename round-trip writes the RIGHT DB variable",
                err is None and same_storage and not collateral,
                f"{old_name} @ {storage} (pc {pcaddr}) -> {new_name}; "
                f"db-hit={[str(v.getVariableStorage()) for v in hit]}; "
                f"collateral-renames={sorted(collateral)}",
            ):
                failures += 1
            # Optional deep-debug: capture the full wire session (query answers
            # included) around the re-decompile via DecompInterface.enableDebug.
            if os.environ.get("KUNA_SMOKE_DEBUGXML"):
                from java.io import File as JFile
                dbg_path = os.path.join(OUT, "post_rename_session.xml")
                ifc.enableDebug(JFile(dbg_path))
            # Event-driven re-decompile: the new name must come back in the C.
            res2 = ifc.decompileFunction(func, 120, monitor)
            c2 = str(res2.getDecompiledFunction().getC())
            with open(os.path.join(OUT, f"kuna_{FUNCTION}_renamed.c"), "w") as fh:
                fh.write(c2)
            lsm2 = res2.getHighFunction().getLocalSymbolMap()
            names2 = sorted(str(s.getName()) for s in lsm2.getSymbols())
            log(f"post-rename localdb: {names2}")
            if not check("re-decompile shows the new name", new_name in c2):
                failures += 1

            # ---- 4: retype the same symbol ---------------------------------
            res3 = ifc.decompileFunction(func, 120, monitor)
            lsm3 = res3.getHighFunction().getLocalSymbolMap()
            tgt3 = None
            for s in lsm3.getSymbols():
                if str(s.getName()) == new_name:
                    tgt3 = s
                    break
            if tgt3 is None:
                failures += 1
                check("retype: renamed symbol resolvable after re-decompile", False)
            else:
                # Retype to the same-size unsigned integer (a size-changing
                # retype of register storage is a GUI error case, not a wire
                # test).
                new_dt = AbstractIntegerDataType.getUnsignedDataType(
                    tgt3.getSize(), program.getDataTypeManager())
                tid = program.startTransaction("phase4 retype")
                err = None
                try:
                    HighFunctionDBUtil.updateDBVariable(
                        tgt3, None, new_dt, SourceType.USER_DEFINED)
                except Exception:
                    err = traceback.format_exc()
                finally:
                    program.endTransaction(tid, err is None)
                if err:
                    log(err)
                res4 = ifc.decompileFunction(func, 120, monitor)
                c4 = str(res4.getDecompiledFunction().getC())
                retyped = [str(s.getDataType())
                           for s in res4.getHighFunction().getLocalSymbolMap().getSymbols()
                           if str(s.getName()) == new_name]
                if not check("retype round-trip",
                             err is None and retyped == [str(new_dt.getName())],
                             f"wanted {new_dt.getName()}, got {retyped}"):
                    failures += 1
                with open(os.path.join(OUT, f"kuna_{FUNCTION}_retyped.c"), "w") as fh:
                    fh.write(c4)

        # ---- 5: parameter rename (cat-0 + index + exact storage) -----------
        if params:
            p0 = params[0]
            old_p = str(p0.getName())
            # An in-place rename is only meaningful if the decoded prototype
            # ALREADY agrees with the database — otherwise getDatabaseParameter
            # force-commits kuna's whole signature and the rename "lands"
            # regardless (the vacuous PASS this check used to be).  Commit the
            # signature first, then assert storage+ordinal equality decoded-vs-DB
            # BEFORE renaming.
            tidp = program.startTransaction("phase4 commit params")
            try:
                HighFunctionDBUtil.commitParamsToDatabase(
                    hf, True, HighFunctionDBUtil.ReturnCommitOption.NO_COMMIT,
                    SourceType.ANALYSIS)
            except Exception:
                log(traceback.format_exc())
            finally:
                program.endTransaction(tidp, True)
            res_p = ifc.decompileFunction(func, 120, monitor)
            lsm_p = res_p.getHighFunction().getLocalSymbolMap()
            decoded = [lsm_p.getParamSymbol(i) for i in range(lsm_p.getNumParams())]
            db_params = list(func.getParameters())
            storage_ok = len(decoded) == len(db_params) and all(
                str(d.getStorage()) == str(db.getVariableStorage())
                and d.getCategoryIndex() == i
                for i, (d, db) in enumerate(zip(decoded, db_params))
            )
            if not check(
                "encoded param storage/ordinals match the DB (no forced full commit)",
                storage_ok,
                f"decoded={[(s.getName(), str(s.getStorage()), s.getCategoryIndex()) for s in decoded]} "
                f"db={[(p.getName(), str(p.getVariableStorage())) for p in db_params]}",
            ):
                failures += 1
            p0 = decoded[0] if decoded else p0
            tid = program.startTransaction("phase4 param rename")
            err = None
            try:
                HighFunctionDBUtil.updateDBVariable(
                    p0, "kuna_param_probe", None, SourceType.USER_DEFINED)
            except Exception:
                err = traceback.format_exc()
            finally:
                program.endTransaction(tid, err is None)
            if err:
                log(err)
            db_params = [str(p.getName()) for p in func.getParameters()]
            if not check("param rename lands on the DB parameter",
                         err is None and "kuna_param_probe" in db_params,
                         f"{old_p} -> DB params now {db_params}"):
                failures += 1
        ifc.dispose()

        # ---- 6: the switch-analyzer configuration --------------------------
        ifc_sw = DecompInterface()
        ifc_sw.toggleCCode(False)
        ifc_sw.toggleSyntaxTree(False)
        ifc_sw.toggleJumpLoads(True)
        assert ifc_sw.openProgram(program), ifc_sw.getLastMessage()
        res_sw = ifc_sw.decompileFunction(func, 120, monitor)
        hf_sw = res_sw.getHighFunction()
        jts = list(hf_sw.getJumpTables()) if hf_sw else []
        detail = "; ".join(
            f"@{jt.getSwitchAddress()} cases={len(list(jt.getCases()))} "
            f"labels={len(list(jt.getLabelValues()))} "
            f"loadtables={len(list(jt.getLoadTables()))}"
            for jt in jts)
        if not check("HighFunction.getJumpTables decodes <jumptablelist>",
                     bool(jts), detail):
            failures += 1
        # loadtables are model-dependent (collected only when table recovery
        # emulates the memory walk) — report, don't gate; the in-tree sim
        # gate (ghidra_sim_faillog_switch_analyzer_shape) covers the jumpload
        # plumbing.
        log(f"loadtables per table: "
            f"{[len(list(jt.getLoadTables())) for jt in jts]}")

        # DecompilerSwitchAnalysisCmd end-to-end: delete the computed jump's
        # references, then let the analyzer command rebuild them from the
        # DecompileResults kuna produced under the analyzer's configuration.
        if jts:
            from ghidra.app.cmd.function import DecompilerSwitchAnalysisCmd
            sw_addr = jts[0].getSwitchAddress()
            instr = program.getListing().getInstructionAt(sw_addr)
            refmgr = program.getReferenceManager()
            if instr is None:
                failures += 1
                check("switch instruction exists at the jumptable address", False,
                      str(sw_addr))
            else:
                tid = program.startTransaction("phase4 clear switch refs")
                try:
                    for ref in list(refmgr.getReferencesFrom(sw_addr)):
                        refmgr.delete(ref)
                finally:
                    program.endTransaction(tid, True)
                cleared = len(list(refmgr.getReferencesFrom(sw_addr)))
                # 12.1.2 vintage: the Cmd consumes the DecompileResults from
                # the analyzer-configured interface (noc/notree/jumpload).
                cmd = DecompilerSwitchAnalysisCmd(res_sw)
                _ = instr
                tid = program.startTransaction("phase4 switch analysis")
                ok = False
                try:
                    ok = cmd.applyTo(program, monitor)
                finally:
                    program.endTransaction(tid, True)
                rebuilt = len(list(refmgr.getReferencesFrom(sw_addr)))
                if not check(
                    "DecompilerSwitchAnalysisCmd rebuilds the case references",
                    ok and rebuilt >= 2,
                    f"cleared to {cleared}, rebuilt {rebuilt} refs "
                    f"(status: {cmd.getStatusMsg()})",
                ):
                    failures += 1
        ifc_sw.dispose()

        # ---- 7: the paramid action -----------------------------------------
        ifc_pid = DecompInterface()
        ifc_pid.setSimplificationStyle("paramid")
        ifc_pid.toggleCCode(False)
        ifc_pid.toggleSyntaxTree(False)
        ifc_pid.toggleParamMeasures(True)
        assert ifc_pid.openProgram(program), ifc_pid.getLastMessage()
        res_pid = ifc_pid.decompileFunction(func, 120, monitor)
        hpid = res_pid.getHighParamID()
        if not check("getHighParamID decodes <parammeasures>",
                     hpid is not None and hpid.getNumInputs() >= 1,
                     f"inputs={hpid.getNumInputs() if hpid else 'n/a'} "
                     f"outputs={hpid.getNumOutputs() if hpid else 'n/a'}"):
            failures += 1
        ifc_pid.dispose()

        field.set(None, None)

    with open(os.path.join(OUT, "PHASE4_REPORT.md"), "w") as fh:
        fh.write("\n".join(REPORT) + "\n")
    print(f"\n[+] report: {os.path.join(OUT, 'PHASE4_REPORT.md')}")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
