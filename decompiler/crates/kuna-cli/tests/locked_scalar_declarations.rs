//! A locked scalar Symbol must survive a weaker merged storage representative.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn pe32_locked_float_survives_unknown_storage_member() {
    check_target("i686-w64-windows-gnu", true, false);
}
#[test]
fn pe64_locked_float_survives_unknown_storage_member() {
    check_target("x86_64-w64-windows-gnu", true, false);
}
#[test]
fn pe32_locked_float_control() {
    check_target("i686-w64-windows-gnu", false, false);
}
#[test]
fn pe64_locked_float_control() {
    check_target("x86_64-w64-windows-gnu", false, false);
}

#[test]
fn pe32_detached_binding_keeps_locked_float() {
    check_target("i686-w64-windows-gnu", true, true);
}
#[test]
fn pe64_detached_binding_keeps_locked_float() {
    check_target("x86_64-w64-windows-gnu", true, true);
}

fn check_target(target: &str, weaker: bool, detached: bool) {
    use kuna_console::ifacedecomp::{
        execute, register_console_commands, register_decomp_commands, IfaceDecompData,
        DECOMPILE_MODULE,
    };
    use kuna_console::ifaceterm::IfaceTerm;
    use kuna_console::kuna_console::register_kuna_commands;
    use kuna_decomp::dtype::type_metatype;
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("locked-scalar-declarations", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(
        &source,
        r#"
__declspec(dllexport) unsigned int choose_word(unsigned int first,unsigned int second,int flag) {
 unsigned int value=first;if(flag)value=second;return value;
}
"#,
    )
    .unwrap();
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-O0", "-shared", "-nostdlib", "-Wl,--entry,0"])
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let entry = image
        .exports()
        .unwrap()
        .into_iter()
        .find(|e| e.name() == b"choose_word")
        .unwrap()
        .address();
    let script=format!("load file {}\noption stackviews on\noption realtypes on\nmap function 0x{entry:x} choose_word\nmap prototype 0x{entry:x} unsigned int __cdecl choose_word(unsigned int first,unsigned int second,int flag)\nload function 0x{entry:x}\ndecompile\nretype v1 float\ndecompile\n",binary.display());
    let mut status = IfaceTerm::into_status("", script.as_bytes());
    register_decomp_commands(&mut status);
    register_kuna_commands(&mut status);
    register_console_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .spec_roots = vec![common::repo_root()
        .join("specs")
        .to_string_lossy()
        .into_owned()];
    while !status.is_stream_finished() && !status.done {
        execute(&mut status);
        assert!(!status.is_in_error(), "{}", status.optr);
        status.optr.clear();
    }
    let data = status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap();
    let fd = data.fd.as_mut().unwrap();
    let high = fd
        .high_bank()
        .iter()
        .find(|(_, h)| {
            h.kuna_name() == Some("v1")
                && h.kuna_symbol_type()
                    .is_some_and(|t| t.get_metatype() == type_metatype::TYPE_FLOAT)
        })
        .map(|(id, _)| id)
        .expect("explicit float local");
    let symbol = fd
        .high_bank()
        .get(high)
        .unwrap()
        .kuna_link_symbol()
        .expect("mapped Symbol");
    assert!(fd
        .get_scope_local()
        .unwrap()
        .database()
        .symbol(symbol)
        .is_type_locked());
    if weaker {
        // Model a merged high whose stack representative kept UNKNOWN4 while
        // its explicitly locked local Symbol still states float. No operations
        // or native storage are changed.
        let stack_index = fd
            .get_arch()
            .manage()
            .get_stack_space()
            .unwrap()
            .get_index();
        let members = fd.high_bank().get(high).unwrap();
        let storage: Vec<_> = (0..members.num_instances())
            .map(|i| members.get_instance(i))
            .filter(|id| {
                fd.vbank().get(*id).is_some_and(|v| {
                    v.get_size() == 4 && !v.is_input() && v.get_space().get_index() == stack_index
                })
            })
            .collect();
        assert!(!storage.is_empty(), "frame representative");
        let unknown = fd
            .get_arch()
            .types()
            .unwrap()
            .get_base(4, type_metatype::TYPE_UNKNOWN)
            .unwrap();
        let storage = if detached { vec![storage[0]] } else { storage };
        for id in storage {
            assert!(fd.vbank_mut().get_mut(id).unwrap().update_type_locked(
                unknown.clone(),
                false,
                true
            ));
        }
    }
    if detached {
        use kuna_base::address::Address;
        use kuna_decomp::context::VarnodeId;
        use kuna_decomp::cover::Cover;
        use kuna_decomp::variable::{CompareNameView, HighContext, VarnodeView, VarnodeViewLoc};
        use std::collections::HashMap;
        struct Locations(HashMap<VarnodeId, Address>);
        impl HighContext for Locations {
            fn vn_loc_view(&self, vn: VarnodeId) -> VarnodeViewLoc {
                VarnodeViewLoc {
                    addr: self.0[&vn].clone(),
                }
            }
            fn vn_view(&self, _: VarnodeId) -> VarnodeView {
                unreachable!("location-only insertion")
            }
            fn vn_cover(&self, _: VarnodeId) -> Option<Cover> {
                unreachable!("location-only insertion")
            }
            fn vn_has_cover(&self, _: VarnodeId) -> bool {
                unreachable!("location-only insertion")
            }
            fn vn_name_view(&self, _: VarnodeId) -> CompareNameView {
                unreachable!("location-only insertion")
            }
        }
        let old = fd.high_bank().get(high).unwrap();
        let ty = old.kuna_symbol_type().unwrap().clone();
        let members: Vec<_> = (0..old.num_instances())
            .map(|i| old.get_instance(i))
            .collect();
        let mut temporaries = Vec::new();
        let members: Vec<_> = members
            .into_iter()
            .map(|old| {
                let op = fd
                    .vbank()
                    .get(old)
                    .unwrap()
                    .get_def()
                    .expect("local definition");
                fd.op_unset_output(op);
                let value = fd.new_unique_out(4, op).unwrap();
                temporaries.push(fd.vbank().get(value).unwrap().get_high().unwrap());
                fd.total_replace(old, value).unwrap();
                fd.vbank_mut().get_mut(value).unwrap().set_explicit();
                value
            })
            .collect();
        assert!(members.len() >= 2, "merged local");
        fd.vbank_mut()
            .get_mut(members[1])
            .unwrap()
            .update_type_locked(ty.clone(), true, true);
        let locations = Locations(
            members
                .iter()
                .map(|id| (*id, fd.vbank().get(*id).unwrap().get_addr().clone()))
                .collect(),
        );
        // Model coalescing that retains the declaration's type and a locked
        // member, but carries no concrete Symbol link into the resulting high.
        let mut bank = std::mem::take(fd.high_bank_mut());
        bank.erase(high);
        for temporary in temporaries {
            bank.erase(temporary);
        }
        let replacement = bank.new_high(members[0]);
        let mut updates = vec![(members[0], replacement, 0)];
        for id in members.iter().skip(1) {
            bank.insert_member(replacement, *id, 0, &locations, &mut |vn, h, g| {
                updates.push((vn, h, g))
            });
        }
        let h = bank.get_mut(replacement).unwrap();
        h.set_kuna_name("v1");
        h.set_symbol_type(ty);
        h.set_symbol_offset(-1);
        *fd.high_bank_mut() = bank;
        for (vn, h, g) in updates {
            fd.vbank_mut().get_mut(vn).unwrap().set_high(h, g);
        }
        assert!(fd
            .high_bank()
            .get(replacement)
            .unwrap()
            .kuna_link_symbol()
            .is_none());
    }
    let arch = data.conf.as_mut().unwrap().arch_mut();
    arch.realtypes = true;
    arch.ctypes = true;
    let body = arch.with_print(|printer, arch| printer.doc_function_full(fd, arch));
    assert!(body.contains("float v1;"), "locked type was lost:\n{body}");
    let generated = root.join("generated.c");
    std::fs::write(&generated, &body).unwrap();
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-std=gnu11", "-ffreestanding", "-nostdinc", "-Werror", "-c"])
            .arg(&generated)
            .arg("-o")
            .arg(root.join("generated.o")),
    );
    let driver = root.join("driver.c");
    std::fs::write(
        &driver,
        r#"
#include <stdint.h>
unsigned int choose_word(unsigned int first,unsigned int second,int flag);
int main(void) {
 uint32_t state=0x3f800000;
 const uint32_t special[]={0,0x80000000,0x3f800000,0xbf800000,0x00800000,0x7f7fffff};
 for(unsigned i=0;i<4096;++i){
  state^=state<<13;state^=state>>17;state^=state<<5;
  uint32_t first=i<6?special[i]:state&0xff7fffff;
  uint32_t second=first^0x80000000;
  for(int flag=0;flag<2;++flag)if(choose_word(first,second,flag)!=(flag?second:first))return 1;
 }return 0;
}
"#,
    )
    .unwrap();
    for opt in ["-O0", "-O2"] {
        let executable = root.join(format!("check{opt}"));
        process::required_output(
            Command::new("clang")
                .args(["-std=gnu11", opt])
                .arg(&generated)
                .arg(&driver)
                .arg("-o")
                .arg(&executable),
        );
        process::required_output(&mut Command::new(executable));
    }
    let _ = std::fs::remove_dir_all(root);
}

/// A DWARF function-pointer local merged with a `void *` register keeps its
/// declared type, so the indirect call through it is valid C.
#[test]
#[cfg(target_arch = "x86_64")]
fn dwarf_function_pointer_local_keeps_its_declaration() {
    let source = common::scratch_file("locked-function-pointer", "c");
    let object = source.with_extension("o");
    let helpers = "static int add(int a,int b){return a+b;}\nstatic int sub(int a,int b){return a-b;}\n";
    std::fs::write(
        &source,
        format!("typedef int (*binop)(int,int);\n{helpers}int apply(int k,int a,int b){{binop f=add;if(k&1)f=sub;if(k&2)f=(binop)0;return f?f(a,b):-1;}}\n"),
    )
    .unwrap();
    if process::optional_output(
        Command::new("gcc").args(["-g", "-O0", "-fno-pic", "-c"]).arg(&source).arg("-o").arg(&object),
    )
    .is_none()
    {
        return;
    }
    let (body, err, status) = common::run_kuna(&["decompile", object.to_str().unwrap(), "apply"]);
    assert_eq!(status, 0, "{err}");
    assert!(body.contains("int (*f)(int,int);"), "function-pointer type was lost:\n{body}");
    let generated = common::scratch_file("locked-function-pointer-generated", "c");
    std::fs::write(
        &generated,
        format!("#include <stddef.h>\n{helpers}{body}\nint main(void){{for(int k=0;k<4;k++)if(apply(k,7,3)!=(k&2?-1:k&1?4:10))return 1;return 0;}}\n"),
    )
    .unwrap();
    let executable = source.with_extension("bin");
    process::required_output(Command::new("gcc").args(["-O0", "-Werror=implicit-function-declaration"]).arg(&generated).arg("-o").arg(&executable));
    process::required_output(&mut Command::new(&executable));
    for path in [source, object, generated, executable] {
        let _ = std::fs::remove_file(path);
    }
}
