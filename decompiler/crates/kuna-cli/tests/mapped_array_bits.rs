//! A coalesced SSA definition still writes its declared frame member.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn pe32_unique_definition_writes_frame_member() {
    check_target("i686-w64-windows-gnu");
}
#[test]
fn pe64_unique_definition_writes_frame_member() {
    check_target("x86_64-w64-windows-gnu");
}

const SAMPLE: &str = r#"
typedef struct Packet {unsigned int tag;unsigned char payload[4];} Packet;
__declspec(dllexport) float read_packet(Packet *packet) {
    if(packet->tag!=7)return 0;
    float value;__builtin_memcpy(&value,packet->payload,4);return value;
}
__declspec(dllexport) float wrap_float(float value) {
    Packet packet;packet.tag=7;
    __builtin_memcpy(packet.payload,&value,4);return read_packet(&packet);
}
"#;

fn check_target(target: &str) {
    use kuna_base::address::Address;
    use kuna_console::ifacedecomp::{
        execute, register_console_commands, register_decomp_commands, IfaceDecompData,
        DECOMPILE_MODULE,
    };
    use kuna_console::ifaceterm::IfaceTerm;
    use kuna_console::kuna_console::register_kuna_commands;
    use kuna_decomp::context::VarnodeId;
    use kuna_decomp::cover::Cover;
    use kuna_decomp::variable::{CompareNameView, HighContext, VarnodeView, VarnodeViewLoc};
    use std::collections::HashMap;

    // Inserting a high member only needs the location ordering. Other reads
    // would indicate that this adapter needs a richer snapshot.
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
        fn vn_cover(&self, _: VarnodeId) -> Option<&Cover> {
            unreachable!("location-only insertion")
        }
        fn vn_has_cover(&self, _: VarnodeId) -> bool {
            unreachable!("location-only insertion")
        }
        fn vn_name_view(&self, _: VarnodeId) -> CompareNameView {
            unreachable!("location-only insertion")
        }
    }
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("mapped-array-bits", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(&source, SAMPLE).unwrap();
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-shared", "-nostdlib", "-O0", "-Wl,--entry,0"])
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let exports = image.exports().unwrap();
    let address = |name: &[u8]| {
        exports
            .iter()
            .find(|entry| entry.name() == name)
            .unwrap()
            .address()
    };
    let read = address(b"read_packet");
    let wrap = address(b"wrap_float");
    let script=format!("load file {}\noption stackviews on\noption realtypes on\nparse line typedef struct Packet {{unsigned int tag;unsigned char payload[4];}} Packet;\nmap function 0x{read:x} read_packet\nmap function 0x{wrap:x} wrap_float\nmap prototype 0x{read:x} float __cdecl read_packet(Packet *packet)\nmap prototype 0x{wrap:x} float __cdecl wrap_float(float value)\nload function 0x{wrap:x}\ndecompile\nretype v1 Packet\ndecompile\n",binary.display());
    let mut status = IfaceTerm::into_status("", script.as_bytes());
    register_decomp_commands(&mut status);
    register_kuna_commands(&mut status);
    register_console_commands(&mut status);
    let data = status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap();
    data.spec_roots = vec![common::repo_root()
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
    let (op, old) = fd
        .obank()
        .iter_alive()
        .find_map(|id| {
            let op = fd.obank().get(id)?;
            let out = op.get_out()?;
            let value = fd.vbank().get(out)?;
            let high = fd.high_bank().get(value.get_high()?)?;
            (op.code() == kuna_num::opcodes::OpCode::CPUI_COPY
                && value.get_size() == 4
                && high.kuna_symbol_offset() == 4
                && high.kuna_link_symbol().is_some())
            .then_some((id, out))
        })
        .expect("payload definition");
    let old_value = fd.vbank().get(old).unwrap();
    let high = old_value.get_high().unwrap();
    let group = old_value.get_merge_group();
    let datatype = fd
        .get_arch()
        .types()
        .unwrap()
        .get_base(4, kuna_decomp::dtype::type_metatype::TYPE_UINT)
        .unwrap();
    // Model the emitter input after coalescing: the same high and frame Symbol,
    // but the defining Varnode lives in SSA unique space instead of on the stack.
    fd.op_unset_output(op);
    let replacement = fd.new_unique_out(4, op).unwrap();
    let temporary_high = fd.vbank().get(replacement).unwrap().get_high().unwrap();
    {
        let value = fd.vbank_mut().get_mut(replacement).unwrap();
        value.update_type(datatype);
        value.set_explicit();
    }
    let instances = fd.high_bank().get(high).unwrap();
    let mut locations = HashMap::new();
    for i in 0..instances.num_instances() {
        let vn = instances.get_instance(i);
        locations.insert(vn, fd.vbank().get(vn).unwrap().get_addr().clone());
    }
    locations.insert(
        replacement,
        fd.vbank().get(replacement).unwrap().get_addr().clone(),
    );
    let mut bank = std::mem::take(fd.high_bank_mut());
    bank.erase(temporary_high);
    let mut updates = Vec::new();
    bank.insert_member(
        high,
        replacement,
        group,
        &Locations(locations),
        &mut |vn, h, g| updates.push((vn, h, g)),
    );
    *fd.high_bank_mut() = bank;
    for (vn, h, g) in updates {
        fd.vbank_mut().get_mut(vn).unwrap().set_high(h, g);
    }
    assert_eq!(fd.high_bank().get(high).unwrap().kuna_symbol_offset(), 4);
    let arch = data.conf.as_mut().unwrap().arch_mut();
    arch.realtypes = true;
    arch.ctypes = true;
    let body = arch.with_print(|printer, arch| {
        printer.options.partial_access = true;
        printer.doc_function_full(fd, arch)
    });
    let generated = root.join("generated.c");
    let prefix = r#"
typedef struct Packet {unsigned int tag;unsigned char payload[4];} Packet;
float read_packet(Packet *p) {float value;if(p->tag!=7)return 0;__builtin_memcpy(&value,p->payload,4);return value;}
"#;
    let driver = r#"
#include <stdint.h>
#include <string.h>
float wrap_float(float value);
int main(void) {
    uint32_t state=0x3fa00000;
    const uint32_t special[]={0,0x80000000,0x3fa00000,0xbf400000};
    for(unsigned i=0;i<4096;i++) {
        state^=state<<13;state^=state>>17;state^=state<<5;
        uint32_t bits=i<4?special[i]:state&0xff7fffff,actual;
        float input;memcpy(&input,&bits,4);float result=wrap_float(input);memcpy(&actual,&result,4);
        if(actual!=bits)return 1;
    }return 0;
}
"#;
    std::fs::write(&generated, format!("{prefix}\n{body}\n")).unwrap();
    let driver_file = root.join("driver.c");
    std::fs::write(&driver_file, driver).unwrap();
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-std=gnu11", "-Werror", "-ffreestanding", "-nostdinc", "-c"])
            .arg(&generated)
            .arg("-o")
            .arg(root.join("generated.o")),
    );
    for optimization in ["-O0", "-O2"] {
        let exe = root.join(format!("check{optimization}"));
        process::required_output(
            Command::new("clang")
                .args(["-std=gnu11", optimization])
                .arg(&generated)
                .arg(&driver_file)
                .arg("-o")
                .arg(&exe),
        );
        process::required_output(&mut Command::new(exe));
    }
}
