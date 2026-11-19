use super::*;

#[test]
fn pointer_carried_stack_counts_compile_and_use_native_word_sizes() {
    use crate::dtype::DatatypeKind;
    use std::process::Command;

    if Command::new("clang").arg("--version").output().is_err() {
        return;
    }
    let arch = bare_arch();
    let types = arch.types_impl();
    types.set_max_basetype_size(8);
    types.set_default_alignment_map();
    types.set_size_of_int(4);
    types.set_size_of_long_long(8);
    for (name, width) in [("uint2", 2), ("uint4", 4), ("uint8", 8)] {
        types.set_core_type(name, width, type_metatype::TYPE_UINT, false).unwrap();
    }
    types.cache_core_types().unwrap();
    let mut program = String::from("typedef unsigned short uint2; typedef unsigned int uint4; typedef unsigned long long uint8;\n");
    let mut checks = String::new();
    let mut index = 0;
    for width in [2, 4, 8] {
        for integer_carrier in [false, true] {
            for scoped in [false, true] {
                let mut context = ContextArch::new(build_manager());
                context.types = Some(Rc::new(crate::dtype::TypeFactoryImpl::new()));
                context.stack_alloc = true;
                let space = Rc::clone(context.manage().get_space_by_name("ram").unwrap());
                let mut fd = Funcdata::new("sample", "sample", Rc::new(context),
                    Address::new(Rc::clone(&space), 0x1000), 0x1000, 0x40).unwrap();
                let mut pointer = Datatype::new_with_align(width, width, type_metatype::TYPE_PTR);
                pointer.kind = DatatypeKind::Pointer {
                    ptrto: named("unsigned char", 1, type_metatype::TYPE_UINT),
                    spaceid: None, truncate: None, wordsize: 1,
                };
                let pointer = Rc::new(pointer);
                let carrier = if integer_carrier {
                    named(match width { 2 => "unsigned short", 4 => "unsigned int", _ => "unsigned long long" },
                        width, type_metatype::TYPE_UINT)
                } else { Rc::clone(&pointer) };
                let count = fd.new_varnode(width, &Address::new(Rc::clone(&space), 0x400), Some(carrier));
                let output = fd.new_varnode(width, &Address::new(space, 0x500), Some(Rc::clone(&pointer)));
                let op = mk_op(&mut fd, if scoped { 3 } else { 2 }, 0x20, OpCode::CPUI_CALLOTHER);
                let builtin = fd.new_constant(4, crate::p3_dataflow::kuna_stackalloc::BUILTIN_STACK_ALLOC as u64);
                fd.op_set_input(op, builtin, 0).unwrap();
                fd.op_set_input(op, count, 1).unwrap();
                fd.op_set_output(op, output).unwrap();
                if scoped {
                    let marker = fd.new_constant(width, 0);
                    fd.op_set_input(op, marker, 2).unwrap();
                    fd.kuna_stack_scopes.push((op, fd.bblocks_root_pub()));
                }
                fd.set_high_level();
                for (value, name) in [(count, "value"), (output, "allocation")] {
                    let high = fd.vbank().get(value).unwrap().get_high().unwrap();
                    let high = fd.high_bank_mut().get_mut(high).unwrap();
                    high.set_kuna_name(name);
                    high.set_symbol_type(Rc::clone(&pointer));
                }
                let mut printer = PrintC::new();
                printer.set_output_stream();
                printer.emit_statement(&fd, &arch, op);
                let extent_check = if scoped {
                    format!("if(sizeof({}) != n) return 3;", printer.stack_scope_array_name(&fd, op))
                } else { String::new() };
                let statement = printer.emit_mut().output_str();
                let mask = if width == 8 { u64::MAX } else { (1u64 << (width * 8)) - 1 };
                program.push_str(&format!("static int sample{index}(unsigned char *value) {{ unsigned char *allocation; {statement} unsigned long long n=((__UINTPTR_TYPE__)value)&{mask}ULL; {extent_check} allocation[0]=7; allocation[n-1]=9; return allocation[0]+allocation[n-1]; }}\n"));
                for count in [8, 32, 127] {
                    checks.push_str(&format!("if(sample{index}((unsigned char *)(__UINTPTR_TYPE__){count})!=16) return 1;\n"));
                }
                if width < 8 && usize::BITS > (width * 8) as u32 {
                    let wrapped = (1u64 << (width * 8)) + 32;
                    checks.push_str(&format!("if(sample{index}((unsigned char *)(__UINTPTR_TYPE__){wrapped}ULL)!=16) return 2;\n"));
                }
                index += 1;
            }
        }
    }
    program.push_str(&format!("int main(void) {{ {checks} return 0; }}\n"));
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let source = std::env::temp_dir().join(format!("kuna-stack-count-{}-{nonce}.c", std::process::id()));
    std::fs::write(&source, program).unwrap();
    for optimization in ["-O0", "-O2"] {
        let executable = source.with_extension(format!("{}-bin", &optimization[1..]));
        let build = Command::new("clang").args(["-std=gnu11", "-Werror=int-conversion", optimization])
            .arg(&source).arg("-o").arg(&executable).output().unwrap();
        assert!(build.status.success(), "{}", String::from_utf8_lossy(&build.stderr));
        let run = Command::new(&executable).output().unwrap();
        assert!(run.status.success(), "{optimization}: {:?}", run.status);
        std::fs::remove_file(executable).unwrap();
    }
    std::fs::remove_file(source).unwrap();
}
