//! Verified Go paired results execute with their original source signatures.

use crate::common;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_langfacet::LangFacet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn fixture(name: &str) -> PathBuf {
    root().join("tests/fixtures/go-pair-results").join(name)
}

fn program(binary: &Path, dwarf: bool) -> ConsoleProgram {
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    program
        .arch_mut()
        .set_kuna_option("godwarfsig", if dwarf { "on" } else { "off" })
        .unwrap();
    program
}

fn bodies(program: ConsoleProgram, names: &[&str]) -> Vec<String> {
    bodies_with(program, names, &[])
}

fn bodies_with(program: ConsoleProgram, names: &[&str], extra: &[&str]) -> Vec<String> {
    bodies_with_inferred_return(program, names, extra, false)
}

fn bodies_with_inferred_return(
    mut program: ConsoleProgram, names: &[&str], extra: &[&str], inferred: bool,
) -> Vec<String> {
    program.commit_pending_analysis().unwrap();
    let mut commands = vec!["option outlang go".to_string()];
    commands.extend(extra.iter().map(|command| command.to_string()));
    let prefix = commands.len();
    for name in names {
        commands.extend([
            format!("load function main.{name}"),
            "decompile".into(),
            "print C".into(),
        ]);
    }
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    kuna_console::kuna_console::register_kuna_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(program);
    for _ in 0..prefix {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    names
        .iter()
        .map(|name| {
            for _ in 0..2 {
                execute(&mut status);
                assert!(!status.inerror, "{name}: {}", status.optr);
            }
            if inferred {
                let data = status.get_data_mut(DECOMPILE_MODULE).unwrap()
                    .as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
                let types = data.conf.as_ref().unwrap().arch().types();
                let fd = data.fd.as_mut().unwrap();
                let calls: Vec<_> = fd.obank().iter_all()
                    .filter_map(|(_, op)| fd.get_call_specs_index(op))
                    .filter(|&index| fd.get_call_specs(index).proto().get_output_type()
                        .is_some_and(|ty| ty.get_name() == "RawCarrier"))
                    .collect();
                assert!(!calls.is_empty());
                for index in calls {
                    let proto = fd.get_call_specs_mut(index).proto_mut();
                    proto.set_output_lock(false);
                    proto.clear_unlocked_output(types).unwrap();
                }
            }
            let start = status.optr.len();
            execute(&mut status);
            assert!(!status.inerror, "{name}: {}", status.optr);
            status.optr[start..].trim().to_string()
        })
        .collect()
}

const SUPPORTED: &[&str] = &[
    "pair",
    "divmod",
    "unsignedPair",
    "branchPair",
    "consume",
    "first",
    "second",
    "forward",
    "effect",
    "consumeEffect",
    "zeroPair",
    "lowPair",
    "signedLowPair",
    "highPair",
    "onesPair",
    "choosePair",
];

#[test]
fn paired_contracts_follow_godwarfsig_and_exclude_other_result_shapes() {
    let names = [
        "pair",
        "unsignedPair",
        "narrow",
        "floats",
        "mixed",
        "triple",
        "aggregate",
    ];
    for image in ["go1.22.5-pair", "go1.27.3-pair"] {
        let binary = fixture(image);
        let off = bodies(program(&binary, false), &names);
        assert!(off[0].contains("[16]byte"), "{}", off[0]);
        let pair = bodies(program(&binary, true), &names);
        assert!(
            pair[0].contains("func pair(a int64, b int64) (int64, int64)"),
            "{}",
            pair[0]
        );
        assert!(
            pair[1].contains("func unsignedPair(a uint64, b uint64) (uint64, uint64)"),
            "{}",
            pair[1]
        );
        for (name, body) in names[2..].iter().zip(&pair[2..]) {
            assert!(!body.contains(") ("), "unsupported shape {name} gained a pair: {image}\n{body}");
        }
    }
    for image in ["compressed", "uncompressed"] {
        let binary = root()
            .join("tests/fixtures/go-dwarf-signatures")
            .join(image);
        let code = bodies(program(&binary, true), &["multiResult"]);
        assert!(
            code[0].contains("func multiResult(p uint64) (uint64, uint64)"),
            "{}",
            code[0]
        );
    }
}

#[test]
fn pair_metadata_never_overwrites_an_existing_type_or_stale_facet() {
    let binary = fixture("go1.22.5-pair");
    for collision in 0..3 {
        let program = program(&binary, true);
        let types = program.arch().types();
        let name = "__kuna_go_pair_i64_i64";
        if collision != 1 {
            types.get_type_struct(name).unwrap();
        }
        if collision != 0 {
            types
                .kuna_register_lang_facet(
                    name,
                    LangFacet::GoPairResults {
                        signed: [true, true],
                    },
                )
                .unwrap();
        }
        let body = bodies(program, &["pair"]).remove(0);
        assert!(
            body.contains("[16]byte") && !body.contains("(int64, int64)"),
            "collision {collision} must fall back to the carrier:\n{body}"
        );
    }
}

#[test]
fn malformed_or_repeated_dwarf_contracts_preserve_inference() {
    let rejected = [
        "badInput",
        "swappedResults",
        "wrongSecond",
        "missingRole",
        "interleaved",
        "narrowPair",
        "floatPair",
        "mixedPair",
        "threeResults",
        "aggregatePair",
        "duplicate",
        "sameDuplicate",
        "wrongProducer",
        "pair.abi0",
    ];
    for image in ["contracts", "contracts-plain"] {
        let binary = fixture(image);
        for (name, body) in rejected.iter().zip(bodies(program(&binary, true), &rejected)) {
            assert!(!body.contains(") ("), "{image}: rejected contract {name} gained a pair\n{body}");
        }
        let accepted = bodies(
            program(&binary, true),
            &["pair", "explicitPair", "signedUnsigned"],
        );
        for (body, signature) in accepted.iter().zip([
            "func pair(a int64, b int64) (int64, int64)",
            "func explicitPair(a int64, b int64) (int64, int64)",
            "func signedUnsigned(a int64, b int64) (int64, uint64)",
        ]) {
            assert!(body.contains(signature), "{body}");
        }
    }
}

fn run(command: &mut Command) -> Output {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{command:?}:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn complete_paired_callees_and_callers_match_native_go() {
    match Command::new("go").arg("version").output() {
        Ok(output) => assert!(output.status.success()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Go toolchain absent; native execution skipped");
            return;
        }
        Err(error) => panic!("go: {error}"),
    }
    let source = std::fs::read_to_string(fixture("pair.go")).unwrap();
    let definitions =
        &source[source.find("var calls").unwrap()..source.find("func main()").unwrap()];
    let harness = r#"
func division(a,b int64)(line string) { defer func(){if recover()!=nil{line="panic"}}();x,y:=divmod(a,b);return fmt.Sprintf("%d %d",x,y) }
func main() {
  for _,a:=range []int64{-9223372036854775808,-9223372036854775807,-65536,-17,-2,-1,0,1,2,17,65536,2147483647,4294967295,9223372036854775806,9223372036854775807} {
    for _,b:=range []int64{-9223372036854775808,-17,-1,0,1,17,9223372036854775807} {
      x,y:=pair(a,b);fmt.Println(x,y);x,y=forward(a,b);fmt.Println(x,y)
      x,y=branchPair(a,b);fmt.Println(x,y);u,v:=unsignedPair(uint64(a),uint64(b));fmt.Println(u,v)
      fmt.Println(consume(a,b),first(a,b),second(a,b));calls=0;e:=consumeEffect(a);fmt.Println(e,calls)
      fmt.Println(division(a,b))
      fmt.Println(zeroPair());fmt.Println(lowPair(uint64(a)));fmt.Println(signedLowPair(a))
      fmt.Println(highPair(uint64(a)));fmt.Println(onesPair());fmt.Println(choosePair(uint64(a),uint64(b)))
    }
  }
}
"#;
    let source_file = common::scratch_file("gopair-source", "go");
    std::fs::write(
        &source_file,
        format!("package main\nimport \"fmt\"\n{definitions}\n{harness}"),
    )
    .unwrap();
    let expected = run(Command::new("go").arg("run").arg(&source_file));
    assert_eq!(
        String::from_utf8_lossy(&expected.stdout).lines().count(),
        1365
    );
    for image in ["go1.22.5-pair", "go1.27.3-pair"] {
        let code = bodies(program(&fixture(image), true), SUPPORTED);
        let file = common::scratch_file("gopair-emitted", "go");
        std::fs::write(
            &file,
            format!(
                "package main\nimport (\"fmt\";\"unsafe\")\nvar _ unsafe.Pointer\nvar calls uint64\n{}\n{harness}",
                code.join("\n")
            ),
        )
        .unwrap();
        let actual = run(Command::new("go").arg("run").arg(&file));
        assert_eq!(actual.stdout, expected.stdout, "{image}");
        assert_eq!(actual.stderr, expected.stderr, "{image}");
    }
    let indirect = bodies_with(program(&fixture("contracts"), true), &["pair", "apply", "indirectFirst", "indirectSecond", "identityPair", "signedUnsigned", "indirectNarrow"], &[
        "map prototype main.apply struct __kuna_go_pair_i64_i64 apply(struct __kuna_go_pair_i64_i64 (*fn)(int8,int8),int8 r0,int8 r1);",
        "map prototype main.indirectFirst int8 indirectFirst(struct __kuna_go_pair_i64_i64 (*fn)(int8,int8),int8 r0,int8 r1);",
        "map prototype main.indirectSecond int8 indirectSecond(struct __kuna_go_pair_i64_i64 (*fn)(int8,int8),int8 r0,int8 r1);",
        "map prototype main.indirectNarrow int4 indirectNarrow(struct __kuna_go_pair_i64_i64 (*fn)(int8,int8),int8 r0,int8 r1);",
    ]);
    assert!(
        indirect[1].contains("fn func(int64, int64) (int64, int64)"),
        "{}",
        indirect[1]
    );
    let code = common::scratch_file("gopair-indirect", "go");
    std::fs::write(&code, format!("package main\n{}\n{}", indirect.join("\n"), r#"
func main() {
  calls:=0
  fn:=func(a,b int64)(int64,int64){calls++;return pair(a,b)}
  for _,a:=range []int64{-9223372036854775808,-4294967297,-17,-1,0,1,17,4294967297,9223372036854775807} {
    for _,b:=range []int64{-9223372036854775808,-1,0,1,9223372036854775807} {
      calls=0;x,y:=apply(fn,a,b);if x!=a+b || y!=a-b || calls!=1 {panic("indirect pair")}
      calls=0;x=indirectFirst(fn,a,b);if x!=a+b || calls!=1 {panic("indirect projection/name capture")}
      calls=0;x=indirectSecond(fn,a,b);if x!=a-b || calls!=1 {panic("indirect second projection")}
      calls=0;n:=indirectNarrow(fn,a,b);if n!=int32(a-b) || calls!=1 {panic("indirect narrowed projection")}
      x,y=identityPair(a,b);if x!=a || y!=b {panic("joined inputs")}
      x,u:=signedUnsigned(a,b);if x!=a+b || u!=uint64(a-b) {panic("signed and unsigned results")}
    }
  }
}
"#)).unwrap();
    run(Command::new("go").arg("run").arg(code));
}

#[test]
fn opaque_call_carrier_is_read_once_into_paired_results() {
    match Command::new("go").arg("version").output() {
        Ok(output) => assert!(output.status.success()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
        Err(error) => panic!("go: {error}"),
    }
    for (image, inferred) in [
        ("go1.22.5-pair", false), ("go1.27.3-pair", false),
        ("go1.22.5-pair", true), ("go1.27.3-pair", true),
    ] {
        let program = program(&fixture(image), true);
        program.arch().types().get_base_named(
            16, kuna_decomp::dtype::type_metatype::TYPE_UINT, "RawCarrier",
        ).unwrap();
        let code = bodies_with_inferred_return(program, &["forward"], &[
            "map prototype main.pair RawCarrier pair(int8 a,int8 b);",
        ], inferred);
        assert!(code[0].contains("func(v [16]byte)"), "{}", code[0]);
        let file = common::scratch_file("gopair-opaque", "go");
        std::fs::write(&file, format!("package main\nimport(\"encoding/binary\";\"unsafe\")\n{}\n{}", code[0], r#"
var calls int
func pair(a,b int64)(value [16]byte) {
  calls++
  binary.LittleEndian.PutUint64(value[0:8],uint64(a+b))
  binary.LittleEndian.PutUint64(value[8:16],uint64(a-b))
  return
}
func main() {
  for _,a:=range []int64{-9223372036854775808,-4294967297,-1,0,1,4294967297,9223372036854775807} {
    for _,b:=range []int64{-9223372036854775808,-1,0,1,9223372036854775807} {
      calls=0;x,y:=forward(a,b)
      if x!=a+b || y!=a-b || calls!=1 {panic("opaque call carrier")}
    }
  }
}
"#)).unwrap();
        run(Command::new("go").arg("run").arg(file));
    }
}
