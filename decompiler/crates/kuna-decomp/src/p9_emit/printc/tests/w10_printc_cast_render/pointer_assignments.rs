//! Pointer declarations can differ from register integer carriers.
use super::*;

#[test]
fn integer_carriers_and_cast_results_assign_to_pointer_members() {
    use crate::dtype::{DatatypeKind, TypeField};
    use std::process::Command;
    for size in [4, 8] {
        for opcode in [OpCode::CPUI_COPY, OpCode::CPUI_CAST] {
            let types = Rc::new(crate::dtype::TypeFactoryImpl::new());
            let mut context = ContextArch::new(build_manager());
            context.types = Some(types);
            let space = Rc::clone(context.manage().get_space_by_name("ram").unwrap());
            let mut fd = Funcdata::new(
                "emit",
                "emit",
                Rc::new(context),
                Address::new(Rc::clone(&space), 0x1000),
                0x1000,
                0x40,
            )
            .unwrap();
            let word_name = if size == 4 {
                "unsigned int"
            } else {
                "unsigned long long"
            };
            let word = named(word_name, size, type_metatype::TYPE_UINT);
            let mut pointer = Datatype::new_with_align(size, size, type_metatype::TYPE_PTR);
            pointer.kind = DatatypeKind::Pointer {
                ptrto: named("unsigned char", 1, type_metatype::TYPE_UINT),
                spaceid: None,
                truncate: None,
                wordsize: 1,
            };
            let pointer = Rc::new(pointer);
            let input_type = if opcode == OpCode::CPUI_CAST {
                Rc::clone(&pointer)
            } else {
                Rc::clone(&word)
            };
            let input = fd.new_varnode(
                size,
                &Address::new(Rc::clone(&space), 0x400),
                Some(input_type),
            );
            // The emitted field is a pointer; the IR carrier keeps its word type.
            let output = fd.new_varnode(size, &Address::new(space, 0x500), Some(Rc::clone(&word)));
            let op = mk_op(&mut fd, 1, 0x20, opcode);
            fd.op_set_input(op, input, 0).unwrap();
            fd.op_set_output(op, output).unwrap();
            fd.set_high_level();
            let high = fd.vbank().get(input).unwrap().get_high().unwrap();
            fd.high_bank_mut()
                .get_mut(high)
                .unwrap()
                .set_kuna_name("value");
            let mut frame = Datatype::new_with_align(size * 2, size, type_metatype::TYPE_STRUCT);
            frame.name = "Frame".into();
            frame.display_name = frame.name.clone();
            frame.id = Datatype::hash_name(&frame.name);
            frame.kind = DatatypeKind::Struct {
                field: vec![
                    TypeField::new(0, 0, "length", Rc::clone(&word)),
                    TypeField::new(1, size, "slot", pointer),
                ],
                bitfield: vec![],
            };
            let high = fd.vbank().get(output).unwrap().get_high().unwrap();
            let high = fd.high_bank_mut().get_mut(high).unwrap();
            high.set_kuna_name("frame");
            high.set_symbol_type(Rc::new(frame));
            high.set_symbol_offset(size);
            let mut printer = PrintC::new();
            printer.set_output_stream();
            printer.emit_statement(&fd, &bare_arch(), op);
            let statement = printer.emit_mut().output_str();
            let argument = if opcode == OpCode::CPUI_CAST {
                "unsigned char *"
            } else {
                word_name
            };
            let call = if opcode == OpCode::CPUI_CAST {
                "bytes+13".to_owned()
            } else {
                format!("({word_name})(__UINTPTR_TYPE__)(bytes+13)")
            };
            let program = format!("typedef struct {{ {word_name} length; unsigned char *slot; }} Frame;\nstatic Frame emit({argument} value) {{ Frame frame; frame.length=0; {statement} return frame; }}\nint main(void) {{ unsigned char bytes[32]; return emit({call}).slot != bytes+13; }}\n");
            if Command::new("clang").arg("--version").output().is_err() {
                continue;
            }
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let source = std::env::temp_dir().join(format!(
                "kuna-pointer-member-{}-{nonce}.c",
                std::process::id()
            ));
            std::fs::write(&source, program).unwrap();
            for optimization in ["-O0", "-O2"] {
                let output = source.with_extension(if size == 4 { "o" } else { "exe" });
                let mut compiler = Command::new("clang");
                compiler.args(["-std=gnu11", "-Wall", "-Wextra", "-Werror", optimization]);
                if size == 4 {
                    compiler.args(["--target=i686-w64-windows-gnu", "-c"]);
                }
                let built = compiler
                    .arg(&source)
                    .arg("-o")
                    .arg(&output)
                    .output()
                    .unwrap();
                assert!(
                    built.status.success(),
                    "{opcode:?} size={size}: {statement}\n{}",
                    String::from_utf8_lossy(&built.stderr)
                );
                if size == 8 {
                    assert!(Command::new(&output).status().unwrap().success());
                }
                std::fs::remove_file(output).unwrap();
            }
            std::fs::remove_file(source).unwrap();
        }
    }
}
