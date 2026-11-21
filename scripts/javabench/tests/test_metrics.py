"""javabench's hand-made metric cases (README.md, "Tests").

They need tree-sitter-languages 1.10.2, networkx, numpy and scipy (and cfgutils, or decbench), and
the javac-based ones JDK 21 and 17 homes (JAVABENCH_JDK21, JAVABENCH_JDK17); without them they are
skipped. From the repo root: PYTHONPATH=. python3 -m unittest scripts.javabench.tests.test_metrics
"""

from __future__ import annotations

import os
import subprocess
import tempfile
import types
import unittest
import zipfile
from pathlib import Path

try:
    from scripts.javabench import ged as ged_module
    from scripts.javabench import projects as P
    from scripts.javabench.bytecode import listings, similarity
    from scripts.javabench.cfg import build_cfg
    from scripts.javabench.classfile import decode_code, parse_class
    from scripts.javabench.ged import ged
    from scripts.javabench.javasrc import index_source
    from scripts.javabench.jtypes import compare_types, erase, method_vars, normalize, pair_vars, signature_to_java
    from scripts.javabench.mapping import map_method, map_types

    if ged_module.IMPLEMENTATION == "vendored":
        import cfgutils  # noqa: F401
        import numpy  # noqa: F401
        import scipy  # noqa: F401
    MISSING = None
except ImportError as e:
    MISSING = f"needs the Python package {e.name}"

JDK = 21
_TMP = None
_ENV = {}


def setUpModule():
    global _TMP
    if MISSING:
        return
    _TMP = tempfile.TemporaryDirectory(prefix="javabench-tests-")
    _ENV.update({k: os.environ.get(k) for k in ("JAVABENCH_WORKSPACE",)})
    P.configure(_TMP.name)


def tearDownModule():
    for k, v in _ENV.items():
        if v is None:
            os.environ.pop(k, None)
        else:
            os.environ[k] = v
    if _TMP is not None:
        _TMP.cleanup()


def needs_jdk(*versions):
    def wrap(test):
        def run(self):
            for v in versions:
                if not (Path(P.JDKS[v]) / "bin/javac").exists():
                    self.skipTest(f"needs a JDK {v} home (JAVABENCH_JDK{v})")
            return test(self)
        run.__name__ = test.__name__
        return run
    return wrap


def fresh_dir(tag: str) -> Path:
    return Path(tempfile.mkdtemp(prefix=f"{tag}-", dir=_TMP.name))


def javac(src_dir: Path, out: Path, files: list[Path], cp: list[Path] = ()) -> subprocess.CompletedProcess:
    out.mkdir(parents=True, exist_ok=True)
    cmd = [os.path.join(P.JDKS[JDK], "bin/javac"), "-J-XX:-UsePerfData", "-g", "--release", "17", "-proc:none",
           "-d", str(out)]
    if cp:
        cmd += ["-cp", os.pathsep.join(map(str, cp))]
    return subprocess.run(cmd + [str(f) for f in files], capture_output=True, text=True)


def method_of(idx, cls: str, name: str):
    return next(m for m in idx.methods if m.owner.binary == cls and m.name == name)

ORIG = """package demo;
import java.util.List;
import java.util.ArrayList;
public class Demo {
    public int classify(int x, int y) {
        int r = 0;
        if (x > 0 && y > 0) {
            r = 1;
        } else if (x < 0) {
            r = 2;
        }
        for (int i = 0; i < y; i++) {
            if (i == 3) continue;
            r += i;
        }
        return r;
    }
    public List<String> names(int n) {
        List<String> out = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            String s = "n" + i;
            out.add(s);
        }
        return out;
    }
    public int fine(int a) {
        return a * 2 + 1;
    }
}
"""

# same logic, different CFG: the && becomes two sequential ifs and the
# continue becomes an if/else around the body
RESTRUCTURED = """package demo;
import java.util.List;
import java.util.ArrayList;
public class Demo {
    public int classify(int x, int y) {
        int r = 0;
        boolean both = false;
        if (x > 0) { if (y > 0) { both = true; } }
        if (both) { r = 1; }
        if (!both && x < 0) { r = 2; }
        for (int i = 0; i < y; i++) {
            if (i != 3) { r += i; } else { r += 0; }
        }
        return r;
    }
    public List<String> names(int n) {
        List<String> out = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            String s = "n" + i;
            out.add(s);
        }
        return out;
    }
    public int fine(int a) {
        return a * 2 + 1;
    }
}
"""

WRONG_TYPES = ORIG.replace("public List<String> names(int n) {\n        List<String> out",
                           "public List<String> names(long n) {\n        List<Object> out")

BROKEN = ORIG.replace("return a * 2 + 1;", 'return a * "two";')


def _mini_project(d: Path, orig_src: str):
    (d / "proj/sources/demo").mkdir(parents=True)
    (d / "proj/deps").mkdir()
    f = d / "proj/sources/demo/Demo.java"
    f.write_text(orig_src)
    r = javac(d / "proj/sources", d / "proj/classes", [f])
    assert r.returncode == 0, r.stderr
    with zipfile.ZipFile(d / "proj/classes.jar", "w") as z:
        for p in (d / "proj/classes").rglob("*.class"):
            z.write(p, str(p.relative_to(d / "proj/classes")))
    return types.SimpleNamespace(name="unittest", dir=d / "proj", javac_jdk=JDK, release_mode="release")


ANON = """package demo;
public class Outer {
    Runnable r1 = new Runnable() { public void run() { System.out.println(1); } };
    Object m(int k) {
        class Local { int v() { return k; } }
        Runnable r2 = new Runnable() { public void run() { System.out.println(new Local().v()); } };
        return r2;
    }
    void m(String s) { System.out.println(s); }
}
"""


@unittest.skipIf(MISSING, MISSING or "")
class MetricTests(unittest.TestCase):
    @needs_jdk(21)
    def test_identical_code_is_perfect_on_all_three(self):
        d = fresh_dir("identical")
        (d / "src/demo").mkdir(parents=True)
        f = d / "src/demo/Demo.java"
        f.write_text(ORIG)
        r = javac(d / "src", d / "orig", [f])
        assert r.returncode == 0, r.stderr
        (d / "dec/demo").mkdir(parents=True)
        g = d / "dec/demo/Demo.java"
        g.write_text(ORIG)  # "decompiled" == original
        r = javac(d / "dec", d / "re", [g])
        assert r.returncode == 0, r.stderr
        so = index_source(f.read_bytes())
        de = index_source(g.read_bytes())
        ocf = parse_class((d / "orig/demo/Demo.class").read_bytes())
        rcf = parse_class((d / "re/demo/Demo.class").read_bytes())
        for name in ("classify", "names", "fine"):
            sm, dm = method_of(so, "demo.Demo", name), method_of(de, "demo.Demo", name)
            assert ged(build_cfg(sm.body).to_json(), build_cfg(dm.body).to_json())["value"] == 0.0
            for s, v in pair_vars(method_vars(sm), method_vars(dm)):
                assert compare_types(s.type_text, {}, v.type_text, {}) == (True, True)
            om = next(m for m in ocf.methods if m.name == name)
            rm = next(m for m in rcf.methods if m.name == name)
            (osim, oex), (rsim, rex) = listings(ocf, om), listings(rcf, rm)
            assert oex == rex
            assert similarity(osim, rsim)[0] == 1.0

    def test_restructured_method_has_nonzero_ged(self):
        so = index_source(ORIG.encode())
        de = index_source(RESTRUCTURED.encode())
        res = ged(build_cfg(method_of(so, "demo.Demo", "classify").body).to_json(),
                  build_cfg(method_of(de, "demo.Demo", "classify").body).to_json())
        assert res["value"] >= 1.0 and res["method"] == "vj_ged"
        # untouched methods stay isomorphic
        for name in ("names", "fine"):
            r = ged(build_cfg(method_of(so, "demo.Demo", name).body).to_json(),
                    build_cfg(method_of(de, "demo.Demo", name).body).to_json())
            assert r["value"] == 0.0

    def test_short_circuit_and_nested_if_are_the_same_structure(self):
        a = index_source(b"class A { int f(int x){ if (x > 0 && x < 9) return 1; return 2; } }")
        b = index_source(b"class A { int f(int x){ if (x > 0) { if (x < 9) return 1; } return 2; } }")
        c = index_source(b"class A { int f(int x){ while (true) { if (x > 9) break; x++; } return x; } }")
        d = index_source(b"class A { int f(int x){ for (;;) { if (x > 9) break; x++; } return x; } }")
        cf = lambda i: build_cfg(i.methods[0].body).to_json()
        assert ged(cf(a), cf(b))["value"] == 0.0
        assert ged(cf(c), cf(d))["value"] == 0.0
        assert ged(cf(a), cf(c))["value"] > 0.0

    def test_wrong_local_type_is_a_type_miss(self):
        so = index_source(ORIG.encode())
        de = index_source(WRONG_TYPES.encode())
        sm, dm = method_of(so, "demo.Demo", "names"), method_of(de, "demo.Demo", "names")
        res = {s.name: compare_types(s.type_text, {}, v.type_text if v else None, {})
               for s, v in pair_vars(method_vars(sm), method_vars(dm))}
        assert res["n"] == (False, False)  # int vs long: miss at both levels
        assert res["out"] == (False, True)  # List<String> vs List<Object>: erasure-level match only
        assert res["s"] == (True, True)
        assert res["<return>"] == (True, True)

    def test_normalization_rules(self):
        assert normalize("java.util.Map.Entry<K, V>") == "Entry<K,V>"
        assert normalize("final @Nullable String") == "String"
        assert normalize("Object...") == "Object[]"
        assert normalize("List<? extends Object>") == "List<?>"
        assert erase(normalize("List<T>"), {"T": "Comparable<? super T>"}) == "List"
        assert erase("T[]", {"T": "Comparable<? super T>"}) == "Comparable[]"
        assert erase("T", {}) == "T"
        assert signature_to_java("Ljava/util/Map<Ljava/lang/String;+Ljava/lang/Number;>;") == "Map<String,?extendsNumber>"
        assert signature_to_java("[TT;") == "T[]"

    @needs_jdk(17, 21)
    def test_method_javac_rejects_is_compile_failure_and_salvage_keeps_the_rest(self):
        from scripts.javabench.recompile import recompile_and_compare

        d = fresh_dir("broken")
        proj = _mini_project(d, ORIG)
        run_dir = d / "run"
        (run_dir / "decompiled/demo").mkdir(parents=True)
        dec = run_dir / "decompiled/demo/Demo.java"
        dec.write_text(BROKEN)
        idx = index_source(dec.read_bytes())
        keys = {}
        descs = {"classify": "(II)I", "names": "(I)Ljava/util/List;", "fine": "(I)I"}
        for name, desc in descs.items():
            m = method_of(idx, "demo.Demo", name)
            keys[f"demo.Demo#{name}{desc}"] = [m.body.start_byte, m.body.end_byte]
        files, per_key = recompile_and_compare(
            proj, run_dir, [{"rel": "demo/Demo.java", "tops": ["demo.Demo"], "keys": keys, "release": 17}], 2, "-g")
        fr = files["demo/Demo.java"]
        assert fr["raw_compiles"] is False
        assert fr["compiles_after_salvage"] is True and fr["stubbed_methods"] == 1
        assert "operator.cant.be.applied.1" in fr["error_categories"]
        assert per_key["demo.Demo#fine(I)I"]["compiled"] is False
        assert per_key["demo.Demo#fine(I)I"]["similarity"] == 0.0
        assert per_key["demo.Demo#classify(II)I"]["exact"] is True
        assert per_key["demo.Demo#names(I)Ljava/util/List;"]["exact"] is True

    @needs_jdk(21)
    def test_listing_is_constant_pool_layout_independent(self):
        d = fresh_dir("cplayout")
        (d / "a/p").mkdir(parents=True)
        (d / "b/p").mkdir(parents=True)
        a = d / "a/p/X.java"
        b = d / "b/p/X.java"
        body = "int f(String s){ return s.length() + g(); }"
        a.write_text("package p; class X { String pad = \"zz\"; int g(){return 7;} " + body + " }")
        b.write_text("package p; class X { " + body + " long q(){return 99L;} int g(){return 7;} String pad = \"zz\"; }")
        assert javac(d / "a", d / "ao", [a]).returncode == 0
        assert javac(d / "b", d / "bo", [b]).returncode == 0
        ca = parse_class((d / "ao/p/X.class").read_bytes())
        cb = parse_class((d / "bo/p/X.class").read_bytes())
        fa = next(m for m in ca.methods if m.name == "f")
        fb = next(m for m in cb.methods if m.name == "f")
        assert fa.code.code != fb.code.code or True  # raw bytes may differ by CP indices
        assert listings(ca, fa)[1] == listings(cb, fb)[1]
        assert decode_code(fa.code.code)[0].op == "aload_1"

    @needs_jdk(21)
    def test_anonymous_local_and_overload_mapping_both_sides(self):
        d = fresh_dir("anon")
        (d / "src/demo").mkdir(parents=True)
        f = d / "src/demo/Outer.java"
        f.write_text(ANON)
        assert javac(d / "src", d / "out", [f]).returncode == 0
        classes = {}
        for p in (d / "out").rglob("*.class"):
            cf = parse_class(p.read_bytes())
            classes[cf.name] = cf
        assert set(classes) == {"demo/Outer", "demo/Outer$1", "demo/Outer$2", "demo/Outer$1Local"}
        idx = index_source(f.read_bytes())
        for use_lines in (True, False):
            tmap = map_types(classes, [idx], use_lines=use_lines)
            assert set(tmap) == set(classes), use_lines
            assert tmap["demo/Outer$2"].start_line == 6
            assert tmap["demo/Outer$1Local"].name == "Local"
            outer = classes["demo/Outer"]
            for m in outer.methods:
                if m.name == "m":
                    md, why = map_method(tmap["demo/Outer"], outer, m, use_lines)
                    assert md is not None, why
                    assert (md.params[0].type_text == "int") == (m.descriptor.startswith("(I"))

    def test_generic_outer_qualifier_is_the_same_type(self):
        assert normalize("TypeToken<T>.TypeSet") == normalize("TypeSet")
        assert normalize("com.x.LocalCache<K, V>.WriteThroughEntry") == "WriteThroughEntry"
        assert normalize("List<? extends java.lang.Number>") == "List<?extendsNumber>"

    def test_break_and_continue_are_pure_jumps(self):
        # `if (c) break;` and `if (!c) { rest }` inside a loop have the same structure
        a = index_source(b"class A { void f(int x){ while (x > 0) { if (x == 5) break; x--; } } }")
        b = index_source(b"class A { void f(int x){ while (x > 0) { if (x == 5) { break; } else { x--; } } } }")
        cf = lambda i: build_cfg(i.methods[0].body).to_json()
        assert ged(cf(a), cf(b))["value"] == 0.0

    @needs_jdk(21)
    def test_loose_fallback_never_crosses_primitive_reference(self):
        d = fresh_dir("enumvalueof")
        (d / "src/demo").mkdir(parents=True)
        f = d / "src/demo/E.java"
        f.write_text("package demo; public enum E { A, B; public static E valueOf(int i) { return values()[i]; } }")
        assert javac(d / "src", d / "out", [f]).returncode == 0
        cf = parse_class((d / "out/demo/E.class").read_bytes())
        idx = index_source(f.read_bytes())
        tmap = map_types({cf.name: cf}, [idx], use_lines=True)
        for m in cf.methods:
            if m.name == "valueOf":
                md, why = map_method(tmap[cf.name], cf, m, True)
                if m.descriptor.startswith("(I"):
                    assert md is not None
                else:  # implicit valueOf(String) has no declaration
                    assert md is None

    @needs_jdk(21)
    def test_omitted_default_constructor_counts_as_implicit(self):
        from scripts.javabench.score import _implicit_ctor_params

        d = fresh_dir("implicitctor")
        (d / "src/demo").mkdir(parents=True)
        f = d / "src/demo/C.java"
        f.write_text("package demo; public class C { public C() {} int g() { return 1; } }")
        assert javac(d / "src", d / "out", [f]).returncode == 0
        cf = parse_class((d / "out/demo/C.class").read_bytes())
        dec = index_source(b"package demo; public class C { int g() { return 1; } }")  # ctor omitted
        tmap = map_types({cf.name: cf}, [dec], use_lines=False)
        init = next(m for m in cf.methods if m.name == "<init>")
        assert map_method(tmap[cf.name], cf, init, False)[0] is None
        assert _implicit_ctor_params(tmap[cf.name], cf, init) == []

    @needs_jdk(21)
    def test_omitted_record_canonical_constructor_counts_as_implicit(self):
        from scripts.javabench.score import _implicit_ctor_params

        d = fresh_dir("implicitrecord")
        (d / "src/demo").mkdir(parents=True)
        f = d / "src/demo/R.java"
        f.write_text("package demo; public record R(String a, int b) { public R {} public R(String a) { this(a, 0); } }")
        assert javac(d / "src", d / "out", [f]).returncode == 0
        cf = parse_class((d / "out/demo/R.class").read_bytes())
        dec = index_source(b"package demo; public record R(String a, int b) { public R(String a) { this(a, 0); } }")
        tmap = map_types({cf.name: cf}, [dec], use_lines=False)
        canon = next(m for m in cf.methods if m.name == "<init>" and m.descriptor == "(Ljava/lang/String;I)V")
        assert map_method(tmap[cf.name], cf, canon, False)[0] is None
        assert [p.name for p in _implicit_ctor_params(tmap[cf.name], cf, canon)] == ["a", "b"]


if __name__ == "__main__":
    unittest.main()
