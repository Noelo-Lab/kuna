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

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn opaque_pointer_offsets_keep_pointer_width_and_byte_scaling() {
    use crate::dtype::{flags, DatatypeKind};
    use std::process::Command;
    let compiler = ["clang", "gcc"].into_iter()
        .find(|cc| Command::new(cc).arg("--version").output().is_ok())
        .expect("opaque pointer verification requires a C compiler");
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let directory = std::env::temp_dir().join(format!("kuna-opaque-offset-{}-{nonce}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    for displacement in [8i64, 0x8000_0008, 0x1_0000_0008, -8] {
        let arch = bare_arch();
        arch.types_impl().set_default_alignment_map();
        arch.types_impl().set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
        arch.types_impl().cache_core_types().unwrap();
        let mut fd = build_fd();
        let mut opaque = Datatype::new_with_align(88, 8, type_metatype::TYPE_STRUCT);
        opaque.name = "Opaque".into();
        opaque.display_name = opaque.name.clone();
        opaque.id = Datatype::hash_name(&opaque.name);
        opaque.flags |= flags::type_incomplete;
        opaque.kind = DatatypeKind::Struct { field: vec![], bitfield: vec![] };
        let pointer = arch.types().get_type_pointer(8, Rc::new(opaque), 1).unwrap();
        let result_pointer = arch.types().get_type_pointer(8, arch.types().get_type_char(1).unwrap(), 1).unwrap();
        let input = fd.new_varnode(8, &Address::new(ram(&fd), 0x400), Some(pointer));
        let output = fd.new_varnode(8, &Address::new(ram(&fd), 0x500), Some(result_pointer));
        let offset = fd.new_constant(8, displacement as u64);
        let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_PTRSUB);
        fd.op_set_input(op, input, 0).unwrap();
        fd.op_set_input(op, offset, 1).unwrap();
        fd.op_set_output(op, output).unwrap();
        fd.set_high_level();
        let high = fd.vbank().get(input).unwrap().get_high().unwrap();
        fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name("base");
        let driver = format!(r#"
#define _GNU_SOURCE
#include <sys/mman.h>
#include <stddef.h>
typedef struct Opaque Opaque;
#ifdef EXTERNAL_ADDRESS
extern char *emitted_address(Opaque *);
#else
ADDRESS_DEFINITION
#endif
int run_boundary(void) {{
    size_t size = 0x100004000ULL;
    char *mapping = mmap(0,size,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS|MAP_NORESERVE,-1,0);
    if (mapping == MAP_FAILED) return 10;
    char *base = mapping+0x2000;
    char *expected = base+({displacement}LL);
    *expected = 37;
    char *actual = emitted_address((Opaque *)base);
    int result = actual != expected || *actual != 37;
    munmap(mapping,size);
    return result;
}}
#ifndef EXTERNAL_ADDRESS
int main(void) {{ return run_boundary(); }}
#endif
"#);
        let oracle = directory.join("oracle.c");
        std::fs::write(&oracle, driver.replace("ADDRESS_DEFINITION", "")).unwrap();
        let object = directory.join("oracle.o");
        let built = Command::new(compiler).args(["-O2", "-DEXTERNAL_ADDRESS", "-c"]).arg(&oracle).arg("-o").arg(&object).output().unwrap();
        assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
        for language in ["c", "rust"] {
            let mut printer = PrintC::new();
            printer.set_name(language);
            printer.rt_ctx.lang = printer.out_lang;
            printer.rt_ctx.enabled = true;
            printer.rt_ctx.long_is_8 = true;
            printer.rt_ctx.ctypes = true;
            printer.set_output_stream();
            printer.op_ptrsub_ir(&fd, &arch, op);
            let expression = printer.emit_mut().output_str();
            let source = directory.join(if language == "c" { "address.c" } else { "address.rs" });
            let program = if language == "c" {
                driver.replace("ADDRESS_DEFINITION", &format!("static char *emitted_address(Opaque *base) {{ return {expression}; }}"))
            } else {
                format!("#![allow(non_camel_case_types,unused_parens)]\ntype Opaque=core::ffi::c_void;\n#[no_mangle] unsafe extern \"C\" fn emitted_address(base:*mut Opaque)->*mut u8 {{ {expression} }}\nextern \"C\" {{ fn run_boundary()->i32; }}\nfn main(){{ assert_eq!(unsafe{{run_boundary()}},0); }}\n")
            };
            std::fs::write(&source, program).unwrap();
            for level in ["0", "2"] {
                let executable = directory.join(format!("{language}-{level}"));
                let mut command = Command::new(if language == "c" { compiler } else { "rustc" });
                if language == "c" {
                    command.arg(format!("-O{level}"));
                } else {
                    command.args(["--edition=2021", "-C"]).arg(format!("opt-level={level}"))
                        .arg("-C").arg(format!("link-arg={}", object.display()));
                }
                let built = command.arg(&source).arg("-o").arg(&executable).output().unwrap();
                assert!(built.status.success(), "{language} offset={displacement}: {expression}\n{}", String::from_utf8_lossy(&built.stderr));
                assert!(Command::new(&executable).status().unwrap().success(), "{language} offset={displacement}: {expression}");
            }
        }
    }
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn native_reads_survive_fusion_when_the_combined_value_is_later_discarded() {
    use crate::action::Rule;
    use std::process::Command;
    let compiler = ["clang", "gcc"].into_iter()
        .find(|cc| Command::new(cc).arg("--version").output().is_ok())
        .expect("native read transition verification requires a C compiler");
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let directory = std::env::temp_dir().join(format!("kuna-native-fusion-{}-{nonce}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    for width in [1, 2, 4, 8] {
        let arch = bare_arch();
        arch.types_impl().set_default_alignment_map();
        for (name, size) in [("unsigned char", 1), ("unsigned short", 2), ("unsigned int", 4), ("unsigned long long", 8)] {
            arch.types_impl().set_core_type(name, size, type_metatype::TYPE_UINT, false).unwrap();
        }
        arch.types_impl().cache_core_types().unwrap();
        let mut fd = build_fd();
        let byte = arch.types().get_base(1, type_metatype::TYPE_UINT).unwrap();
        let pointer = arch.types().get_type_pointer(8, byte, 1).unwrap();
        let base = fd.new_varnode(8, &Address::new(ram(&fd), 0x400), Some(Rc::clone(&pointer)));
        let next = fd.new_varnode(8, &Address::new(ram(&fd), 0x408), Some(pointer));
        let add = mk_op(&mut fd, 2, 0x10, OpCode::CPUI_INT_ADD);
        let offset = fd.new_constant(8, width as u64);
        fd.op_set_input(add, base, 0).unwrap();
        fd.op_set_input(add, offset, 1).unwrap();
        fd.op_set_output(add, next).unwrap();
        let memory = fd.new_varnode_space(&ram(&fd));
        let mut loads = Vec::new();
        let mut values = Vec::new();
        for (index, address) in [base, next].into_iter().enumerate() {
            let op = mk_op(&mut fd, 2, 0x20 + index as u64 * 4, OpCode::CPUI_LOAD);
            fd.op_set_input(op, memory, 0).unwrap();
            fd.op_set_input(op, address, 1).unwrap();
            let word = arch.types().get_base(width, type_metatype::TYPE_UINT).unwrap();
            let value = fd.new_varnode(width, &Address::new(ram(&fd), 0x500 + index as u64 * 8), Some(word));
            fd.op_set_output(op, value).unwrap();
            fd.obank_mut().get_mut(op).unwrap()
                .set_additional_flag(crate::op::pcodeop_addlflags::kuna_native_load);
            loads.push(op);
            values.push(value);
        }
        let piece = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
        fd.op_set_input(piece, values[1], 0).unwrap();
        fd.op_set_input(piece, values[0], 1).unwrap();
        let combined = fd.new_unique_out(width * 2, piece).unwrap();
        let consumer = mk_op(&mut fd, 1, 0x34, OpCode::CPUI_COPY);
        fd.op_set_input(consumer, combined, 0).unwrap();
        fd.new_unique_out(width * 2, consumer).unwrap();
        let root = fd.bblocks_ref().root.unwrap();
        let block = fd.bblocks_mut().new_block_basic(root);
        for op in [add, loads[0], loads[1], piece, consumer] { fd.op_insert_end(op, block); }
        // The concatenation is live when fusion considers its two scalar reads.
        // A later rewrite erases that consumer, then ordinary early removal runs.
        crate::double::RuleDoubleLoad::new("doubleload").apply_op(piece, &mut fd);
        fd.op_destroy(consumer);
        fd.op_destroy(piece);
        let candidates: Vec<_> = fd.obank().iter_alive().collect();
        for op in candidates {
            if fd.obank().get(op).unwrap().code() == OpCode::CPUI_LOAD {
                crate::ruleaction_1::RuleEarlyRemoval::new("earlyremoval").apply_op(op, &mut fd);
            }
        }
        fd.set_high_level();
        for (value, name) in [(base, "base"), (next, "next"), (values[0], "left"), (values[1], "right")] {
            if let Some(high) = fd.vbank().get(value).and_then(|v| v.get_high()) {
                fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name(name);
            }
        }
        let remaining: Vec<_> = fd.obank().iter_alive().collect();
        for &op in &remaining {
            if fd.obank().get(op).unwrap().code() == OpCode::CPUI_LOAD {
                let output = fd.obank().get(op).unwrap().get_out().unwrap();
                if !values.contains(&output) {
                    let high = fd.vbank().get(output).unwrap().get_high().unwrap();
                    fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name("combined_read");
                }
            }
        }
        let mut printer = PrintC::new();
        printer.rt_ctx.enabled = true;
        printer.rt_ctx.long_is_8 = true;
        printer.rt_ctx.ctypes = true;
        printer.set_output_stream();
        for op in remaining {
            if fd.obank().get(op).unwrap().code() == OpCode::CPUI_LOAD
                || crate::kuna_volatileload::dynamic_read(&fd, op).is_some()
            {
                printer.emit_statement(&fd, &arch, op);
            }
        }
        let statements = printer.emit_mut().output_str();
        let program = format!(r#"
#define _GNU_SOURCE
#include <sys/mman.h>
#include <sys/wait.h>
#include <unistd.h>
#include <signal.h>
typedef unsigned char uint1;typedef unsigned short uint2;typedef unsigned int uint4;typedef unsigned long long uint8;
typedef uint1 undefined1;typedef uint2 undefined2;typedef uint4 undefined4;typedef uint8 undefined8;typedef unsigned __int128 undefined16;
static void emitted(unsigned char *base,int active) {{
    if(active) {{ unsigned char *next=base+{width}; unsigned long long left,right;unsigned __int128 combined_read;
        {statements}
    }}
}}
static int check(unsigned char *p,int active,int fault) {{
    pid_t child=fork();if(child<0)return 1;
    if(!child){{emitted(p,active);_exit(0);}}
    int status;if(waitpid(child,&status,0)!=child)return 2;
    return fault ? !(WIFSIGNALED(status)&&WTERMSIG(status)==SIGSEGV) : !(WIFEXITED(status)&&WEXITSTATUS(status)==0);
}}
int main(void) {{
    long page=sysconf(_SC_PAGESIZE);unsigned char *p=mmap(0,page*3,PROT_NONE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);
    if(p==MAP_FAILED||mprotect(p+page,page,PROT_READ|PROT_WRITE))return 10;
    int failed=check(p,0,0)||check(p+page*2-2*{width},1,0)
        ||check(p+page*2-{width},1,1)||check(p+page-{width},1,1);
    munmap(p,page*3);return failed;
}}
"#);
        let source = directory.join(format!("transition-{width}.c"));
        std::fs::write(&source, program).unwrap();
        for level in ["0", "2"] {
            let executable = directory.join(format!("transition-{width}-{level}"));
            let built = Command::new(compiler).arg(format!("-O{level}")).arg(&source).arg("-o").arg(&executable).output().unwrap();
            assert!(built.status.success(), "{statements}\n{}", String::from_utf8_lossy(&built.stderr));
            assert!(Command::new(&executable).status().unwrap().success(), "lost native read after value erasure: width={width}, O{level}\n{statements}");
        }
    }
    std::fs::remove_dir_all(directory).unwrap();
}
