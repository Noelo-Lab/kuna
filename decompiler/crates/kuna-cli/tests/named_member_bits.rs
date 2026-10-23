//! Bit transfers use the type of the emitted named member, not a sibling view.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn pe32_named_float_member_preserves_bits() {
    check_target("i686-w64-windows-gnu", true);
}
#[test]
fn pe64_named_float_member_preserves_bits() {
    check_target("x86_64-w64-windows-gnu", true);
}
#[test]
fn pe32_named_float_member_control() {
    check_target("i686-w64-windows-gnu", false);
}
#[test]
fn pe64_named_float_member_control() {
    check_target("x86_64-w64-windows-gnu", false);
}

fn check_target(target: &str, word_view: bool) {
    use kuna_console::{
        assertions, assertsyntax,
        decompile_step::{decompile_one, DecompileSeed},
        engine::bootstrap_from_object,
    };
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("named-member-bits", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(
        &source,
        r#"
#define API __declspec(dllexport) __attribute__((noinline))
__declspec(dllexport) float bank[4]={1,2,3,4};
API unsigned int observe(void){unsigned int bits;__builtin_memcpy(&bits,&bank[2],4);return bits;}
API unsigned int store_values(float first,float second){
 bank[2]=first;unsigned int observed=observe();bank[2]=second;return observed;
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
    let exports = image.exports().unwrap();
    let address = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let store = address(b"store_values");
    let observer = address(b"observe");
    let bank = address(b"bank");
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[common::repo_root()
            .join("specs")
            .to_string_lossy()
            .into_owned()],
    )
    .unwrap();
    for option in ["stackviews", "realtypes", "ctypes"] {
        program.arch_mut().set_kuna_option(option, "on").unwrap();
    }
    program.commit_pending_analysis().unwrap();
    let facts = [
        "typedef union Store {float real[4];unsigned int bits[4];} Store;".into(),
        format!("data 0x{bank:x} Store store"),
        format!(
            "prototype 0x{store:x} unsigned int __cdecl store_values(float first,float second)"
        ),
        format!("prototype 0x{observer:x} unsigned int __cdecl observe(void)"),
    ];
    program.set_assertions(
        facts
            .iter()
            .map(|s: &String| assertsyntax::parse_directive(s).unwrap())
            .collect(),
    );
    assertions::apply_program_scoped(&mut program);
    let entry = program
        .function_entries_canonical()
        .into_iter()
        .find(|e| e.addr.get_offset() == store)
        .unwrap();
    let name = entry.name.clone();
    let seed = assertions::function_seed(&mut program, &name, &entry.addr, true);
    let mut fd = decompile_one(
        program.arch_mut(),
        &name,
        entry.addr,
        entry.size as i32,
        &DecompileSeed {
            purpose: kuna_decomp::decompile_drive::DrivePurpose::Batch,
            mapped_symbols: &[],
            usepoint_symbols: &[],
            dynamic_symbols: &[],
            object_assertions: &[],
            pending_proto: seed.pending_proto.as_ref(),
            flow_overrides: &seed.flow_overrides,
            mapped_params: &seed.mapped_params,
        },
        &[],
    )
    .result
    .unwrap();
    if word_view {
        let output = fd
            .obank()
            .iter_alive()
            .find_map(|op| {
                let out = fd.obank().get(op)?.get_out()?;
                let value = fd.vbank().get(out)?;
                (value.get_addr().get_offset() == bank + 8 && value.is_persist()).then_some(out)
            })
            .expect("global output");
        let high = fd.vbank().get(output).unwrap().get_high().unwrap();
        let h = fd.high_bank().get(high).unwrap();
        let members: Vec<_> = (0..h.num_instances()).map(|i| h.get_instance(i)).collect();
        assert!(members.len() > 1, "multiple global incarnations");
        let rep = *members
            .iter()
            .find(|id| fd.vbank().get(**id).unwrap().is_addr_tied())
            .unwrap_or(&members[0]);
        let word = fd
            .get_arch()
            .types()
            .unwrap()
            .get_base(4, kuna_decomp::dtype::type_metatype::TYPE_UINT)
            .unwrap();
        // Keep both native stores and their named float member resolution intact;
        // one merged incarnation supplies a raw word declaration representative.
        fd.vbank_mut()
            .get_mut(rep)
            .unwrap()
            .update_type_locked(word, false, true);
    }
    let types = root.join("types.h");
    std::fs::write(
        &types,
        r#"
typedef union Store {float real[4];unsigned int bits[4];} Store;
extern Store store;
unsigned int observe(void);
unsigned int _observe(void);
"#,
    )
    .unwrap();
    for partial in [false, true] {
        let arch = program.arch_mut();
        let mut printer = arch.take_print();
        printer.options.partial_access = partial;
        let body = printer.doc_function_full(&fd, arch);
        arch.put_print(printer);
        assert!(
            body.contains("store.real[2] ="),
            "named float member:\n{body}"
        );
        let generated = root.join(format!("generated-{partial}.c"));
        std::fs::write(&generated, format!("#include \"types.h\"\n{body}")).unwrap();
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
            format!(
                r#"
#include "types.h"
#include <stdint.h>
#include <string.h>
Store store;
unsigned int observe(void){{return store.bits[2];}}
unsigned int _observe(void){{return observe();}}
unsigned int {name}(float first,float second);
int main(void){{
 uint32_t state=0x3f800000;
 const uint32_t special[]={{0,0x80000000,0x3f800000,0xbf800000,0x00800000,0x7f7fffff}};
 for(unsigned i=0;i<4096;++i){{
  state^=state<<13;state^=state>>17;state^=state<<5;
  uint32_t first=i<6?special[i]:state&0xff7fffff,second=first^0x80000000;
  float a,b;memcpy(&a,&first,4);memcpy(&b,&second,4);
  if({name}(a,b)!=first||store.bits[2]!=second)return 1;
 }}return 0;
}}
"#
            ),
        )
        .unwrap();
        for opt in ["-O0", "-O2"] {
            let executable = root.join(format!("check{opt}-{partial}"));
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
    }
    let _ = std::fs::remove_dir_all(root);
}
