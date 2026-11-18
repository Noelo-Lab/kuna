use std::fs;
use std::process::{Command, Output};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;

struct CodeImg(Vec<u8>);

impl LoadImage for CodeImg {
    fn get_file_name(&self) -> &str {
        "code"
    }
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        for (i, b) in ptr.iter_mut().enumerate() {
            *b = *self.0.get(addr.get_offset() as usize + i).unwrap_or(&0);
        }
        Ok(())
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn disassemble(image: &[u8], code: &[u8]) -> (i32, String, String) {
    let mut sleigh = Sleigh::new(Box::new(CodeImg(code.to_vec())), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(image).unwrap();
    let space = Rc::clone(sleigh.base().manager().get_default_code_space().unwrap());
    let (mut mnemonic, mut body) = (String::new(), String::new());
    let length = sleigh
        .print_assembly_into(&Address::new(space, 0), &mut mnemonic, &mut body)
        .unwrap();
    (length, mnemonic, body)
}

const HEADER: &str = "define endian=little;\n\
    define space ram type=ram_space size=4 default;\n\
    define space register type=register_space size=4;\n\
    define register offset=0 size=4 r0;\n";

fn compile(body: &str, includes: &[(&str, &str)], strict: bool) -> (Output, Option<Vec<u8>>) {
    let scratch = tempfile::tempdir().unwrap();
    let source = scratch.path().join("main.slaspec");
    let output = scratch.path().join("compiled.sla");
    fs::write(&source, format!("{HEADER}{body}")).unwrap();
    for (name, text) in includes {
        fs::write(scratch.path().join(name), text).unwrap();
    }
    let mut command = Command::new(env!("CARGO_BIN_EXE_slacomp"));
    if strict {
        command.arg("-l");
    }
    let result = command.arg(&source).arg(&output).output().unwrap();
    (result, fs::read(output).ok())
}

#[test]
fn pattern_build_errors_report_the_undefined_operand() {
    for (body, includes, expected) in [
        (
            "define token instr(8) op=(0,7);\n:bad ghost is op=0 { r0=1; }\n",
            &[][..],
            "main.slaspec:7 - ERROR Error: ghost: operand is undefined: for table \"instruction\" constructor starting at line 6\n\nNo output produced\n",
        ),
        (
            "define token instr(8) op=(0,7);\n@include \"value.sinc\"\n:bad value is value { r0=value; }\n",
            &[("value.sinc", "value: ghost is op=0 { export r0; }\n")][..],
            "main.slaspec:8 - ERROR value.sinc:1: Problem in table 'value':Error: ghost: operand is undefined: for table \"value\" constructor starting at line 1\n\nNo output produced\n",
        ),
    ] {
        let (result, image) = compile(body, includes, false);
        assert_eq!(result.status.code(), Some(2));
        assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), expected);
        assert!(image.is_none());
    }
}

#[test]
fn operand_defined_as_itself_or_a_later_operand_is_rejected() {
    for action in ["idx = cop0209;", "idx = idx;"] {
        let (result, image) = compile(
            &format!(
                "define token instr(16) op=(8,15) cop0209=(2,9);\n\
                 :foo idx is op=0x1 & cop0209 [ {action} ] {{ r0 = idx; }}\n"
            ),
            &[],
            false,
        );
        assert_eq!(result.status.code(), Some(2));
        assert_eq!(
            std::str::from_utf8(&result.stderr).unwrap(),
            "main.slaspec:7 - ERROR Error: idx: operand is defined as itself or an operand that follows it: for table \"instruction\" constructor starting at line 6\n\nNo output produced\n"
        );
        assert!(image.is_none());
    }
}

#[test]
fn operand_defined_from_a_field_expression_or_an_earlier_operand_decodes() {
    for (display, action, expected) in [
        ("idx", "idx = cop0209 + 0;", "0x6a"),
        ("cop0209, idx", "idx = cop0209;", "0x6a, 0x6a"),
    ] {
        let (result, image) = compile(
            &format!(
                "define token instr(16) op=(8,15) cop0209=(2,9);\n\
                 :foo {display} is op=0x1 & cop0209 [ {action} ] {{ r0 = idx; }}\n"
            ),
            &[],
            false,
        );
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        let (length, mnemonic, body) = disassemble(&image.unwrap(), &[0xa8, 0x01]);
        assert_eq!((length, mnemonic.as_str(), body.as_str()), (2, "foo", expected));
    }
}

#[test]
fn unreferenced_table_warning_names_the_table() {
    let (result, image) = compile(
        "define token instr(8) op=(0,7);\n@include \"unused.sinc\"\n:good is op=0 { r0=1; }\n",
        &[("unused.sinc", "unused: \"value\" is op=1 { export r0; }\n")],
        false,
    );
    assert!(result.status.success());
    assert_eq!(
        std::str::from_utf8(&result.stderr).unwrap(),
        "WARN  unused.sinc:1: Unreferenced table 'unused'\n"
    );
    assert!(image.unwrap().starts_with(b"sla\x04"));
}

#[test]
fn pattern_conflicts_keep_both_table_and_constructor_locations() {
    for identical in [false, true] {
        let left = format!(
            "left: \"low\" is lo=1 {{ export r0; }}\nleft: \"high\" is {} {{ export r0; }}\n",
            if identical { "lo=1" } else { "hi=2" },
        );
        let right = format!(
            "right: \"low\" is lo=3 {{ export r0; }}\nright: \"high\" is {} {{ export r0; }}\n",
            if identical { "lo=3" } else { "hi=4" },
        );
        for strict in [false, true] {
            let (result, image) = compile(
                "define token instr(16) lo=(0,3) hi=(4,7) sel=(8,15);\n\
                 @include \"left.sinc\"\n\
                 @include \"right.sinc\"\n\
                 :one left is sel=0 & left { r0=left; }\n\
                 :two right is sel=1 & right { r0=right; }\n",
                &[("left.sinc", &left), ("right.sinc", &right)],
                strict,
            );
            if !identical && !strict {
                assert!(result.status.success());
                assert!(result.stderr.is_empty());
                assert!(image.unwrap().starts_with(b"sla\x04"));
                continue;
            }
            let message = if identical {
                "Constructor has identical pattern to constructor at"
            } else {
                "Constructor pattern cannot be distinguished from constructor at"
            };
            let expected = format!(
                "main.slaspec:10 - ERROR left.sinc:2: {message} left.sinc:1\n\
                 main.slaspec:10 - ERROR left.sinc:1: {message} left.sinc:2\n\
                 main.slaspec:10 - ERROR right.sinc:2: {message} right.sinc:1\n\
                 main.slaspec:10 - ERROR right.sinc:1: {message} right.sinc:2\n\
                 No output produced\n"
            );
            assert_eq!(result.status.code(), Some(2));
            assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), expected);
            assert!(image.is_none());
        }
    }
}
