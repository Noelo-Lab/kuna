//! `typelib`: the prebuilt type library declares the libc imports of an
//! x86-64 Linux image that no built-in table declares.
//!
//! `fixtures/typelib_x86_64` is `typelib_x86_64.c` built with
//! `gcc -O1 -fno-inline -o typelib_x86_64 typelib_x86_64.c` (no debug info).
use crate::common::{self, fixture};

fn kuna(args: &[&str]) -> String {
    let (out, stderr, code) = common::run_kuna(args);
    assert_eq!(code, 0, "{args:?}: {stderr}");
    out
}

#[test]
fn an_import_no_table_declares_takes_the_library_declaration() {
    let image = fixture("typelib_x86_64");
    let off = kuna(&["decompile", &image, "wait_readable", "--option", "typelib", "off"]);
    assert!(off.contains("(int)poll(&v1,1,a1)"), "{off}");
    assert!(off.contains("return v3 & 1;"), "{off}");

    let auto = kuna(&["decompile", &image, "wait_readable"]);
    assert!(auto.contains("pollfd v1;"), "{auto}");
    assert!(auto.contains("v1.events = 1;"), "{auto}");
    assert!(auto.contains("if (poll(&v1,1,a1) <= 0)"), "{auto}");
    assert!(auto.contains("return v1.revents & 1;"), "{auto}");
}

#[test]
fn a_library_file_replaces_the_built_in_one() {
    let image = fixture("typelib_x86_64");
    let library = fixture("typelib_custom.ktl");
    let c = kuna(&["decompile", &image, "wait_readable", "--option", "typelib", &library]);
    assert!(c.contains("return v1.got & 1;"), "{c}");
    let rlimit = kuna(&["decompile", &image, "raise_files", "--option", "typelib", &library]);
    assert!(!rlimit.contains("rlim_cur"), "{rlimit}");
}

/// `struct rusage` is laid out differently by glibc, so only an image that
/// names musl is offered the members.
#[test]
fn a_musl_image_takes_the_entries_glibc_declares_differently() {
    let image = fixture("typelib_musl_x86_64");
    let c = kuna(&["decompile", &image, "max_rss"]);
    assert!(c.contains("rusage v2;"), "{c}");
    assert!(c.contains("v2.ru_utime.tv_sec + v2.ru_maxrss"), "{c}");
    let off = kuna(&["decompile", &image, "max_rss", "--option", "typelib", "off"]);
    assert!(off.contains("long v2 [4];"), "{off}");
}

#[test]
fn a_file_that_is_not_a_library_is_refused() {
    let image = fixture("typelib_x86_64");
    let source = fixture("typelib_x86_64.c");
    let (_, stderr, code) =
        common::run_kuna(&["decompile", &image, "wait_readable", "--option", "typelib", &source]);
    assert_ne!(code, 0);
    assert!(stderr.contains("is not a type library"), "{stderr}");
}

/// A declared `qsort`, `signal` or `glob` still hands its callback slot's
/// prototype to the function passed there.
#[test]
fn a_declared_callback_slot_still_declares_its_callback() {
    let image = fixture("callbacktype_x86_64");
    let off = kuna(&["decompile-all", &image, "--option", "typelib", "off"]);
    let auto = kuna(&["decompile-all", &image]);
    assert!(auto.contains("void qsort(void *a0,size_t a1,size_t a2,int (*a3)(void *,void *))"), "{auto}");
    assert!(auto.contains("glob(\"/tmp/*\",0,warn_path,&v8);"), "{auto}");
    for declared in ["int by_key(void *a0,void *a1)", "void on_int(int a0)", "int warn_path(char *a0,int a1)"] {
        assert!(off.contains(declared), "{declared}: {off}");
        assert!(auto.contains(declared), "{declared}: {auto}");
    }
}

#[test]
fn an_image_of_another_target_is_unchanged() {
    for name in ["pe_imports.exe", "libctypes_mips32_glibc_le32", "libctypes_widths_i386"] {
        let image = fixture(name);
        let auto = kuna(&["decompile-all", &image]);
        let off = kuna(&["decompile-all", &image, "--option", "typelib", "off"]);
        assert_eq!(auto, off, "{name}");
    }
}
