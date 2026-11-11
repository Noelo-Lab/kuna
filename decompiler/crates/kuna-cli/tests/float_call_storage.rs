//! Explicit word storage preserves a floating call result, and a native float op's result,
//! through native float operations.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::process::Command;
#[test]
fn pe32_float_call_word_storage() {
    check("i686-w64-windows-gnu", true, false, false);
}
#[test]
fn pe64_float_call_word_storage() {
    check("x86_64-w64-windows-gnu", true, false, false);
}
#[test]
fn pe32_float_call_control() {
    check("i686-w64-windows-gnu", false, false, false);
}
#[test]
fn pe64_float_call_control() {
    check("x86_64-w64-windows-gnu", false, false, false);
}
#[test]
fn pe32_float_arith_word_storage() {
    check("i686-w64-windows-gnu", true, false, true);
}
#[test]
fn pe64_float_arith_word_storage() {
    check("x86_64-w64-windows-gnu", true, false, true);
}
#[test]
fn pe32_double_arith_word_storage() {
    check("i686-w64-windows-gnu", true, true, true);
}
#[test]
fn pe64_double_arith_word_storage() {
    check("x86_64-w64-windows-gnu", true, true, true);
}
#[test]
fn pe32_double_call_word_storage() {
    check("i686-w64-windows-gnu", true, true, false);
}
#[test]
fn pe64_double_call_word_storage() {
    check("x86_64-w64-windows-gnu", true, true, false);
}
#[test]
fn pe32_double_call_control() {
    check("i686-w64-windows-gnu", false, true, false);
}
#[test]
fn pe64_double_call_control() {
    check("x86_64-w64-windows-gnu", false, true, false);
}
fn check(target: &str, word: bool, double: bool, arith: bool) {
    let scalar = if double { "double" } else { "float" };
    let chosen = if arith { "value-second" } else { "second" };
    let storage = if double {
        "unsigned long long"
    } else {
        "unsigned int"
    };
    use kuna_console::ifacedecomp::{
        execute, register_console_commands, register_decomp_commands, IfaceDecompData,
        DECOMPILE_MODULE,
    };
    use kuna_console::ifaceterm::IfaceTerm;
    use kuna_console::kuna_console::register_kuna_commands;
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("float-call-storage", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(&source,r#"
#define API __declspec(dllexport) __attribute__((noinline))
API float adjust_value(float value){return value+0.125f;}
API float choose_value(float first,float second){float value=adjust_value(first);if(value>second)value=CHOSEN;return value;}
"#.replace("float", scalar).replace("CHOSEN", chosen)).unwrap();
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
    let addr = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let choose = addr(b"choose_value");
    let adjust = addr(b"adjust_value");
    let script=format!("load file {}\noption stackviews on\noption realtypes on\noption ctypes on\nmap function 0x{choose:x} choose_value\nmap prototype 0x{choose:x} float __cdecl choose_value(float first,float second)\nmap function 0x{adjust:x} adjust_value\nmap prototype 0x{adjust:x} float __cdecl adjust_value(float value)\nload function 0x{choose:x}\ndecompile\n{}",binary.display(),if word{format!("retype v1 {storage}\ndecompile\n")}else{String::new()});
    let script = script.replace("float ", &format!("{scalar} "));
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
    let fd = data.fd.as_ref().unwrap();
    let arch = data.conf.as_mut().unwrap().arch_mut();
    let body = arch.with_print(|printer, arch| printer.doc_function_full(fd, arch));
    println!("{target} word={word}\n{body}");
    let generated = root.join("generated.c");
    std::fs::write(
        &generated,
        format!("{scalar} adjust_value({scalar});\n{body}"),
    )
    .unwrap();
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
float adjust_value(float x){return x+0.125f;}
float choose_value(float,float);
int main(void){
for(int i=-64;i<=64;i++)for(int j=-8;j<=8;j++){
float a=i*0.0625f,b=j*0.25f,expected=adjust_value(a);if(expected>b)expected=CHOSEN;
if(choose_value(a,b)!=expected)return 1;
}return 0;}
"#
        .replace("float", scalar)
        .replace("CHOSEN", &chosen.replace("value", "expected").replace("second", "b")),
    )
    .unwrap();
    for opt in ["-O0", "-O2"] {
        let exe = root.join(format!("check{opt}"));
        process::required_output(
            Command::new("clang")
                .args(["-std=gnu11", opt])
                .arg(&generated)
                .arg(&driver)
                .arg("-o")
                .arg(&exe),
        );
        process::required_output(&mut Command::new(exe));
    }
    let _ = std::fs::remove_dir_all(root);
}
