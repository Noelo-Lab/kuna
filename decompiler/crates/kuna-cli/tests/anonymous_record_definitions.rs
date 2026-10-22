//! Anonymous records keep compilable definitions and their shared type identity.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use kuna_console::grammar::{parse_c, DataOrg};
use kuna_decomp::dtype::{type_metatype as Meta, Datatype, TypeFactory, TypeFactoryImpl};
use kuna_decomp::kuna_langtypes::SpellCtx;
use std::{collections::HashSet, process::Command, rc::Rc};

#[test]
fn nested_anonymous_members_compile() {
    check("typedef struct Envelope {struct {uint4 first;uint4 second;} nested;union {uint4 word;float4 value;} choice;} Envelope;","Envelope","Envelope e={0};e.nested.first=7;e.nested.second=11;e.choice.word=0x3f800000;return e.nested.first+e.nested.second==18&&e.choice.value==1.0f?0:1;");
}
#[test]
fn shared_anonymous_members_keep_one_type() {
    check("typedef struct Pair {struct {uint4 value;} first,second;} Pair;","Pair","_Static_assert(__builtin_types_compatible_p(__typeof__(((Pair*)0)->first),__typeof__(((Pair*)0)->second)),\"shared type\");Pair p={0};p.first.value=19;p.second=p.first;return p.second.value==19?0:1;");
}
#[test]
fn distinct_anonymous_members_keep_distinct_types() {
    check("typedef struct Split {struct {uint4 value;} first;struct {uint4 value;} second;} Split;","Split","_Static_assert(!__builtin_types_compatible_p(__typeof__(((Split*)0)->first),__typeof__(((Split*)0)->second)),\"distinct types\");Split p={0};p.first.value=17;p.second.value=23;return p.first.value+p.second.value==40?0:1;");
}
#[test]
fn anonymous_union_typedef_compiles() {
    check(
        "typedef union {uint4 word;float4 value;} Choice;",
        "Choice",
        "Choice c={0};c.word=0x3f800000;return c.value==1.0f?0:1;",
    );
}

#[test]
fn named_member_control() {
    check(
        "typedef struct Named {struct Child {uint4 value;} nested;} Named;",
        "Named",
        "Named v={0};v.nested.value=29;return v.nested.value==29?0:1;",
    );
}

#[test]
fn anonymous_struct_typedef_keeps_its_name() {
    check(
        "typedef struct {uint4 value;} Box;",
        "Box",
        "Box b={0};b.value=31;return b.value==31?0:1;",
    );
}

#[test]
fn anonymous_names_do_not_depend_on_declaration_order() {
    let first = "typedef struct First {struct {uint4 value;} nested;} First;";
    let second = "typedef struct Second {union {uint4 bits;float4 value;} nested;} Second;";
    let render = |declarations: &[&str]| {
        let f = factory();
        for declaration in declarations {
            parse_c(
                declaration,
                &f,
                DataOrg {
                    addr_size: 4,
                    word_size: 1,
                },
                &[],
                |_, _| Ok(()),
            )
            .unwrap();
        }
        let mut text = String::new();
        definitions(
            &f.find_by_name("First").unwrap().unwrap(),
            &mut HashSet::new(),
            &mut text,
        );
        text
    };
    assert_eq!(render(&[first, second]), render(&[second, first]));
}

#[test]
fn repeated_anonymous_type_spelling_reuses_its_identity() {
    let f = factory();
    let parse = || {
        kuna_console::grammar::parse_type(
            "struct {uint4 value;}",
            &f,
            DataOrg {
                addr_size: 4,
                word_size: 1,
            },
        )
        .unwrap()
        .0
    };
    let first = parse();
    let second = parse();
    assert!(!first.get_name().is_empty());
    assert_eq!(first.get_name(), second.get_name());
}

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    for (name, size, meta) in [
        ("void", 0, Meta::TYPE_VOID),
        ("uint4", 4, Meta::TYPE_UINT),
        ("float4", 4, Meta::TYPE_FLOAT),
    ] {
        f.set_core_type(name, size, meta, false).unwrap();
    }
    f.cache_core_types().unwrap();
    f
}
fn definitions(ty: &Rc<Datatype>, seen: &mut HashSet<*const Datatype>, text: &mut String) {
    if !seen.insert(Rc::as_ptr(ty)) {
        return;
    }
    if let Some(base) = ty.get_typedef() {
        definitions(base, seen, text);
    }
    if let Some(base) = ty.get_ptr_to() {
        definitions(&base, seen, text);
    }
    if let Some(base) = ty.get_array_base() {
        definitions(&base, seen, text);
    }
    let mut i = 0;
    while let Some(field) = ty.get_field(i) {
        definitions(&field.field_type, seen, text);
        i += 1;
    }
    if kuna_decomp::kuna_structdefs::is_definable(ty) {
        if ty.get_typedef().is_none() {
            let keyword = match ty.get_metatype() {
                Meta::TYPE_STRUCT => Some("struct"),
                Meta::TYPE_UNION => Some("union"),
                _ => None,
            };
            if let Some(keyword) = keyword {
                text.push_str(&format!("typedef {keyword} {0} {0};\n", ty.get_name()));
            }
        }
        text.push_str(&kuna_decomp::kuna_structdefs::definition_text(
            ty,
            SpellCtx::OFF,
        ));
    }
}
fn check(declaration: &str, name: &str, body: &str) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let f = factory();
    parse_c(
        declaration,
        &f,
        DataOrg {
            addr_size: 4,
            word_size: 1,
        },
        &[],
        |_, _| Ok(()),
    )
    .unwrap();
    let ty = f.find_by_name(name).unwrap().unwrap();
    let mut text=String::from("typedef unsigned int uint4;typedef float float4;typedef unsigned int undefined4;typedef unsigned long long undefined8;\n");
    definitions(&ty, &mut HashSet::new(), &mut text);
    text.push_str(&format!("\nint main(void){{{body}}}\n"));
    println!("{text}");
    let root = common::scratch_file("anonymous-record-definitions", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    std::fs::write(&source, &text).unwrap();
    for target in ["i686-w64-windows-gnu", "x86_64-w64-windows-gnu"] {
        process::required_output(
            Command::new("clang")
                .arg(format!("--target={target}"))
                .args(["-std=gnu11", "-ffreestanding", "-nostdinc", "-Werror", "-c"])
                .arg(&source)
                .arg("-o")
                .arg(root.join("sample.o")),
        );
    }
    for opt in ["-O0", "-O2"] {
        let exe = root.join(format!("check{opt}"));
        process::required_output(
            Command::new("clang")
                .args(["-std=gnu11", opt])
                .arg(&source)
                .arg("-o")
                .arg(&exe),
        );
        process::required_output(&mut Command::new(exe));
    }
    let _ = std::fs::remove_dir_all(root);
}
