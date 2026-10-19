//! Scalar fields retain their bit representation inside a wider aggregate.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn floating_stores_in_aggregate_fields_match_native() {
    let compilers: Vec<_> = ["clang", "gcc"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        return;
    }
    let dir = common::scratch_file("aggregate-scalar-bitcast", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let declarations = r#"
#include <string.h>
typedef struct first_record {int tag,count;} first_record;
typedef struct second_record {int code,status;} second_record;
typedef union buffer {first_record first;second_record second;} buffer;
void reuse(buffer *record);
float consume(float *value);
"#;
    let original = dir.join("original.c");
    std::fs::write(
        &original,
        format!(
            r#"
{declarations}
__attribute__((noinline)) void reuse(buffer *record) {{
    __asm__ volatile("" : : "r"(record) : "memory");
}}
__attribute__((noinline)) float consume(float *pointer) {{
    float value;memcpy(&value,pointer,sizeof value);return value;
}}
float overlay_x87(float value) {{
    buffer record;record.first.tag=7;reuse(&record);
    __asm__ volatile("flds %1; fld1; faddp; fstps %0" : "=m"(record) : "m"(value) : "st");
    return consume((float*)&record);
}}
float overlay_copy(float value) {{
    buffer record;record.first.tag=7;reuse(&record);
    memcpy(&record,&value,sizeof value);return consume((float*)&record);
}}
float overlay_reload(float value) {{
    buffer record;memcpy(&record,&value,sizeof value);reuse(&record);
    float result;memcpy(&result,&record,sizeof result);return result;
}}
int numeric(float value) {{return (int)value;}}
#ifdef SAMPLE_MAIN
int main(void) {{return 0;}}
#endif
"#
        ),
    )
    .unwrap();
    let sample = dir.join("sample");
    process::required_output(
        Command::new(compilers[0])
            .args(["-O0", "-g", "-fno-pie", "-no-pie", "-DSAMPLE_MAIN"])
            .arg(&original)
            .arg("-o")
            .arg(&sample),
    );
    let (printed, stderr, rc) = common::run_kuna(&[
        "decompile-all",
        sample.to_str().unwrap(),
        "--functions",
        "overlay_x87,overlay_copy,overlay_reload,numeric",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{printed}");
    let regenerated = dir.join("regenerated.c");
    std::fs::write(&regenerated,format!("typedef float float4;\ntypedef long double float10;\n{declarations}\n#define overlay_x87 printed_overlay_x87\n#define overlay_copy printed_overlay_copy\n#define overlay_reload printed_overlay_reload\n#define numeric printed_numeric\n{printed}")).unwrap();
    let driver = dir.join("driver.c");
    std::fs::write(&driver,r#"
#include <stdio.h>
#include <string.h>
float overlay_x87(float);float printed_overlay_x87(float);
float overlay_copy(float);float printed_overlay_copy(float);
float overlay_reload(float);float printed_overlay_reload(float);
int numeric(float);int printed_numeric(float);
int main(void){
 const float values[]={0.0f,-0.0f,1.25f,-1.5f,1234.5f,16.25f,-0.25f};
 for(unsigned i=0;i<sizeof values/sizeof values[0];i++){
  float expected=overlay_x87(values[i]),actual=printed_overlay_x87(values[i]);
  if(memcmp(&expected,&actual,4)){fprintf(stderr,"x87 expected=%a actual=%a\n",(double)expected,(double)actual);return 1;}
  expected=overlay_copy(values[i]);actual=printed_overlay_copy(values[i]);
  if(memcmp(&expected,&actual,4))return 2;
  expected=overlay_reload(values[i]);actual=printed_overlay_reload(values[i]);
  if(memcmp(&expected,&actual,4))return 3;
  if(numeric(values[i])!=printed_numeric(values[i]))return 4;
 }
 return 0;
}
"#).unwrap();
    for cc in compilers {
        for opt in ["-O0", "-O2"] {
            let exe = dir.join(format!("roundtrip-{cc}-{opt}"));
            process::required_output(
                Command::new(cc)
                    .args([opt, "-fno-fast-math", "-ffp-contract=off"])
                    .arg(&original)
                    .arg(&regenerated)
                    .arg(&driver)
                    .arg("-o")
                    .arg(&exe),
            );
            process::required_output(&mut Command::new(exe));
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
