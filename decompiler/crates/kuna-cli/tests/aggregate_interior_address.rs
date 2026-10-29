//! Explicit aggregate mappings must relocate interior field addresses too.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
use crate::common;
use common::process;
use object::{Object, ObjectSymbol};
use std::process::Command;

#[test]
fn mapped_struct_array_fields_recompile_with_independent_storage() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("aggregate-interior-address", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let original = root.join("original.c");
    std::fs::write(&original, r#"
struct pair {float real,imag;};
__attribute__((used)) static const struct pair samples[4]={{1,2},{3,5},{7,11},{13,17}};
__attribute__((noinline)) float imaginary(unsigned index) {return samples[index&3].imag;}
__attribute__((noinline)) float real(unsigned index) {return samples[index&3].real;}
__attribute__((noinline)) unsigned long numeric(unsigned index) {return (unsigned long)index+(unsigned long)&samples[0].imag;}
#ifdef FIXTURE_MAIN
int main(void){return 0;}
#endif
"#).unwrap();
    let sample = root.join("sample");
    process::required_output(
        Command::new("clang")
            .args(["-O1", "-fno-pie", "-no-pie", "-DFIXTURE_MAIN"])
            .arg(&original)
            .arg("-o")
            .arg(&sample),
    );
    let bytes = std::fs::read(&sample).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let address = image
        .symbols()
        .find(|s| s.name().ok() == Some("samples"))
        .unwrap()
        .address();
    let project = root.join("project");
    let mapping = format!("data 0x{address:x} sample_pair validation_samples[4]");
    let (_, stderr, rc) = common::run_kuna(&[
        "decompile-project",
        sample.to_str().unwrap(),
        "--functions",
        "imaginary,real,numeric",
        "--assert",
        "typedef struct sample_pair {float real;float imag;} sample_pair;",
        "--assert",
        &mapping,
        "--assert",
        "prototype imaginary float imaginary(unsigned int index)",
        "--assert",
        "prototype real float real(unsigned int index)",
        "--assert",
        "prototype numeric unsigned long numeric(unsigned int index)",
        "--assert-strict",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(rc, 0, "{stderr}");
    let printed = std::fs::read_to_string(project.join("sample.c")).unwrap();
    let memory_functions = printed.split("// Function: numeric").next().unwrap();
    assert!(
        !memory_functions.contains(&format!("0x{:x}", address + 4)),
        "{printed}"
    );
    let regenerated = project.join("regenerated.c");
    std::fs::write(
        &regenerated,
        format!(
            r#"
#define imaginary printed_imaginary
#define real printed_real
#define numeric printed_numeric
{printed}
sample_pair validation_samples[4]={{{{19,23}},{{29,31}},{{37,41}},{{43,47}}}};
"#
        ),
    )
    .unwrap();
    let driver = root.join("driver.c");
    std::fs::write(
        &driver,
        r#"
float imaginary(unsigned);float real(unsigned);
float printed_imaginary(unsigned);float printed_real(unsigned);
unsigned long numeric(unsigned);unsigned long printed_numeric(unsigned);
int main(void){
 const float expected_real[]={19,29,37,43},expected_imag[]={23,31,41,47};
 for(unsigned i=0;i<256;++i){
  if(printed_real(i)!=expected_real[i&3] || printed_imaginary(i)!=expected_imag[i&3])return 1;
  if(printed_real(i)==real(i) || printed_imaginary(i)==imaginary(i))return 2;
  if(printed_numeric(i)!=EXPECTED_ADDRESS+4+i)return 3;
 }
 return 0;
}
"#,
    )
    .unwrap();
    for cc in ["clang", "gcc"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        for opt in ["-O0", "-O2"] {
            let exe = root.join(format!("roundtrip-{cc}-{opt}"));
            process::required_output(
                Command::new(cc)
                    .arg(opt)
                    .arg(format!("-DEXPECTED_ADDRESS={address}UL"))
                    .arg(&original)
                    .arg(&regenerated)
                    .arg(&driver)
                    .arg("-o")
                    .arg(&exe),
            );
            process::required_output(&mut Command::new(exe));
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
