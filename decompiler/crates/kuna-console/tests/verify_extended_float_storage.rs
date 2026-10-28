//! Split integer loads must forward x87 extended values as storage bits.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::path::PathBuf;
use std::process::Command;

use kuna_base::xml::DocumentStorage;
use kuna_console::engine::bootstrap_from_file;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::{print_c_recompile_prelude, print_c_types};

#[test]
fn split_extended_float_fields_preserve_native_payloads() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let fixture = root.join("tests/datatests/longdouble.xml");
    let mut documents = DocumentStorage::new();
    let document = documents.parse_document(&std::fs::read(&fixture).unwrap()).unwrap();
    let script = document.get_root().get_children().iter()
        .find(|element| element.get_name() == "script").unwrap();
    // Reuse the fixture's real options, maps and declarations, not a guessed
    // record layout or a prototype that bypasses the split-load recovery path.
    let mut commands: Vec<String> = script.get_children().iter()
        .map(|command| String::from_utf8(command.get_content().to_vec()).unwrap().replace('\r', "").replace('\n', " "))
        .take_while(|command| !command.trim_start().starts_with("lo fu "))
        .collect();
    commands.extend(["load function printstruct".into(), "decompile".into(), "print C".into()]);
    let count = commands.len();
    let specs = vec![root.join("specs").to_str().unwrap().to_owned()];
    let mut program = bootstrap_from_file(fixture.to_str().unwrap(), "", &specs).unwrap();
    program.commit_pending_analysis().unwrap();
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let data = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        data.conf = Some(program);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    let code = status.optr.clone();
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let data = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    let program = data.conf.as_mut().unwrap();
    let prelude = print_c_recompile_prelude(program.arch());
    let types = print_c_types(program.arch_mut());
    let directory = crate::common::scratch_file("extended-float-storage", "dir");
    std::fs::create_dir(&directory).unwrap();
    std::fs::write(directory.join("types.h"), format!("{prelude}\n{types}")).unwrap();
    let native = directory.join("native.c");
    std::fs::write(&native, r#"
#include "types.h"
extern float10 ldarr[16];
void writeLongDouble(float10 *,float10);
int4 native_printstruct(ldstruct *value) {
    writeLongDouble(ldarr,value->a);
    writeLongDouble(ldarr,value->b);
    return (int4)value->c+(int4)value->d+(int4)value->e+(int4)value->f;
}
"#).unwrap();
    let source = directory.join("consumer.c");
    std::fs::write(&source, format!(r#"
#include "types.h"
#include <stdint.h>
#include <string.h>
#include <float.h>
_Static_assert(LDBL_MANT_DIG==64 && LDBL_MAX_EXP==16384,"native x87 extended format");
float10 ldarr[16];
static unsigned calls,bad;
static unsigned char seen[2][10];
void writeLongDouble(float10 *target,float10 value) {{
    if(target!=ldarr || calls>=2){{bad=1;return;}}
    memcpy(seen[calls++],&value,10);
}}
{code}
int4 native_printstruct(ldstruct *);
static void set_bits(float10 *value,uint64_t low,uint16_t high) {{
    memset(value,0,sizeof(*value));memcpy(value,&low,8);memcpy((char *)value+8,&high,2);
}}
int main(void) {{
    const uint64_t low[]={{0,0,UINT64_C(0x8000000000000000),UINT64_C(0x8000000000000000),
        1,UINT64_C(0x8000000000000000),UINT64_C(0xc000000000001234)}};
    const uint16_t high[]={{0,0x8000,0x3fff,0xc000,0,0x7fff,0x7fff}};
    for(unsigned i=0;i<sizeof(low)/sizeof(*low);++i) {{
        for(unsigned j=0;j<sizeof(low)/sizeof(*low);++j) {{
            ldstruct value;memset(&value,0,sizeof(value));
            value.c=-2;value.d=7;value.e=-5;value.f=11;
            set_bits(&value.a,low[i],high[i]);set_bits(&value.b,low[j],high[j]);
            calls=bad=0;
            int4 native=native_printstruct(&value);
            if(bad || calls!=2 || native!=11)return 1;
            unsigned char expected[2][10];memcpy(expected,seen,sizeof(expected));
            if(memcmp(expected[0],&value.a,10) || memcmp(expected[1],&value.b,10))return 2;
            calls=bad=0;
            int4 generated=printstruct(&value);
            if(bad || calls!=2 || generated!=native || memcmp(seen,expected,sizeof(seen)))return 3;
        }}
    }}
    return 0;
}}
"#)).unwrap();
    for compiler in ["gcc", "clang"] {
        match Command::new(compiler).arg("--version").output() {
            Ok(output) => assert!(output.status.success()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => panic!("cannot run {compiler}: {error}"),
        }
        for optimization in ["-O0", "-O2"] {
            let object = directory.join(format!("{compiler}{optimization}.o"));
            let output = Command::new(compiler).args(["-std=gnu11", optimization, "-Werror", "-c"])
                .arg(&native).arg("-o").arg(&object).output().unwrap();
            assert!(output.status.success(), "{}\n{types}", String::from_utf8_lossy(&output.stderr));
            let executable = directory.join(format!("{compiler}{optimization}"));
            let output = Command::new(compiler).args(["-std=gnu11", optimization, "-Werror"])
                .arg(&source).arg(&object).arg("-o").arg(&executable).output().unwrap();
            assert!(output.status.success(), "{}\n{code}", String::from_utf8_lossy(&output.stderr));
            assert!(Command::new(&executable).status().unwrap().success(), "{compiler} {optimization}\n{code}");
        }
    }
    std::fs::remove_dir_all(directory).unwrap();
}
