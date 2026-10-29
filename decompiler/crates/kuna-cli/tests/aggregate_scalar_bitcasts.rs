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

#[test]
fn floating_values_copied_into_integer_fields_match_native() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let dir = common::scratch_file("aggregate-field-copy-bitcast", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let declarations = r#"
#include <string.h>
typedef struct record {int tag,count;long wide,spare;} record;
void touch(record *value);
double reload(double *value);
"#;
    let original = dir.join("original.c");
    std::fs::write(
        &original,
        format!(
            r#"
{declarations}
__attribute__((noinline)) void touch(record *value) {{
    __asm__ volatile("" : : "r"(value) : "memory");
}}
__attribute__((noinline)) double reload(double *pointer) {{
    double value;memcpy(&value,pointer,sizeof value);return value;
}}
int float_field(float x, float y, int c) {{
    record r;r.count=c;r.wide=0;r.spare=0;touch(&r);
    float v=c>3?x:y+y;memcpy(&r.tag,&v,sizeof v);touch(&r);return r.tag^r.count;
}}
double double_field(double x, int c) {{
    record r;r.tag=c;touch(&r);
    double v=c?x:x*x;memcpy(&r.wide,&v,sizeof v);touch(&r);return reload((double*)&r.wide);
}}
#ifdef SAMPLE_MAIN
int main(void) {{return 0;}}
#endif
"#
        ),
    )
    .unwrap();
    let sample = dir.join("sample");
    process::required_output(
        Command::new("gcc")
            .args([
                "-O0",
                "-g",
                "-march=x86-64",
                "-fno-tree-vectorize",
                "-fno-pie",
                "-no-pie",
                "-DSAMPLE_MAIN",
            ])
            .arg(&original)
            .arg("-o")
            .arg(&sample),
    );
    let (printed, stderr, rc) = common::run_kuna(&[
        "decompile-all",
        sample.to_str().unwrap(),
        "--functions",
        "float_field,double_field",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{printed}");
    let regenerated = dir.join("regenerated.c");
    std::fs::write(&regenerated,format!("typedef float float4;\ntypedef double float8;\n{declarations}\n#define float_field printed_float_field\n#define double_field printed_double_field\n{printed}")).unwrap();
    let driver = dir.join("driver.c");
    std::fs::write(&driver,r#"
#include <stdio.h>
#include <string.h>
int float_field(float,float,int);int printed_float_field(float,float,int);
double double_field(double,int);double printed_double_field(double,int);
int main(void){
 const float values[]={0.0f,-0.0f,1.25f,-1.5f,1234.5f,16.25f,-0.25f};
 for(unsigned i=0;i<sizeof values/sizeof values[0];i++){
  float y=values[(i+3)%7];
  for(int c=-1;c<6;c++){
   if(float_field(values[i],y,c)!=printed_float_field(values[i],y,c)){fprintf(stderr,"float_field(%a,%a,%d)\n",(double)values[i],(double)y,c);return 1;}
   double expected=double_field(values[i]*1.75,c),actual=printed_double_field(values[i]*1.75,c);
   if(memcmp(&expected,&actual,sizeof expected)){fprintf(stderr,"double_field expected=%a actual=%a\n",expected,actual);return 2;}
  }
 }
 return 0;
}
"#).unwrap();
    for cc in ["clang", "gcc"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
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

/// The pinned gcc 15 fixture reloads a union word through a register that also holds an
/// `undefined4` variable, the codegen this host's compiler may not produce.
#[test]
fn union_field_words_held_in_undefined_variables_match_native() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        return;
    }
    let dir = common::scratch_file("aggregate-undefined-word-copy", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let declarations = r#"
#include <string.h>
typedef struct reading {float pressure,temperature,altitude;} reading;
typedef struct sample {unsigned stamp;float distance,deviation;} sample;
typedef struct message {int kind;union {reading weather;sample range;int count[3];} data;} message;
int next_message(message *m);
"#;
    let original = dir.join("original.c");
    std::fs::write(
        &original,
        format!(
            r#"
{declarations}
reading latest;int last_count;int pending;message queue[4];
__attribute__((noinline)) int next_message(message *m) {{
    if (pending <= 0) return 0;
    *m = queue[--pending];return 1;
}}
float latest_weather(void) {{
    message m;reading keep = latest;
    while (next_message(&m)) {{
        if (m.kind == 1) keep = m.data.weather;
        else latest = keep;
    }}
    latest = keep;return keep.pressure;
}}
int latest_count(void) {{
    message m;int count = last_count;float pressure = latest.pressure;
    while (next_message(&m)) {{
        if (m.kind == 0) count = m.data.count[0];
        else if (m.kind == 1) pressure = m.data.weather.pressure;
    }}
    last_count = count;latest.pressure = pressure;return count;
}}
#ifdef SAMPLE_MAIN
int main(void) {{return 0;}}
#endif
"#
        ),
    )
    .unwrap();
    let driver = dir.join("driver.c");
    std::fs::write(&driver,r#"
#include <stdio.h>
#include <string.h>
typedef struct message {int kind;unsigned raw[3];} message;
extern unsigned latest[3];extern int last_count,pending;extern message queue[4];
float latest_weather(void);float printed_latest_weather(void);
int latest_count(void);int printed_latest_count(void);
static const unsigned words[]={0x3fc00000,0x40490fdb,0xbf000000,0x7f7fffff,1,0x80000000,0x12345678,0x447a0000,0x41200000};
static unsigned stored[4];
static void fill(int s){
 pending=4;
 for(int i=0;i<4;i++){queue[i].kind=(i+s)%3;for(int j=0;j<3;j++)queue[i].raw[j]=words[(i*3+j+s)%9];}
 for(int j=0;j<3;j++)latest[j]=words[(s+j+5)%9];
 last_count=s;
}
static void keep(void){memcpy(stored,latest,12);stored[3]=last_count;}
static int same(void){return !memcmp(stored,latest,12)&&stored[3]==(unsigned)last_count;}
int main(void){
 for(int s=0;s<9;s++){
  fill(s);float expected=latest_weather();keep();
  fill(s);float actual=printed_latest_weather();
  if(memcmp(&expected,&actual,4)||!same()){fprintf(stderr,"latest_weather seed %d\n",s);return 1;}
  fill(s);int count=latest_count();keep();
  fill(s);if(count!=printed_latest_count()||!same()){fprintf(stderr,"latest_count seed %d\n",s);return 2;}
 }
 return 0;
}
"#).unwrap();
    let mut samples = vec![(
        "gcc15".to_string(),
        std::path::PathBuf::from(common::fixture("aggregate_undefined_word_x86_64")),
    )];
    for sample_cc in &compilers {
        let sample = dir.join(format!("sample-{sample_cc}"));
        process::required_output(
            Command::new(sample_cc)
                .args([
                    "-O2",
                    "-g",
                    "-march=x86-64",
                    "-fno-tree-vectorize",
                    "-fno-pie",
                    "-no-pie",
                    "-DSAMPLE_MAIN",
                ])
                .arg(&original)
                .arg("-o")
                .arg(&sample),
        );
        samples.push((sample_cc.to_string(), sample));
    }
    for (sample_cc, sample) in &samples {
        let (printed, stderr, rc) = common::run_kuna(&[
            "decompile-all",
            sample.to_str().unwrap(),
            "--functions",
            "latest_weather,latest_count",
        ]);
        assert_eq!(rc, 0, "{stderr}\n{printed}");
        let regenerated = dir.join(format!("regenerated-{sample_cc}.c"));
        std::fs::write(&regenerated,format!("#include <stdbool.h>\ntypedef unsigned int undefined4;typedef unsigned long long undefined8;\n{declarations}\nextern unsigned char latest[];extern int last_count;\n#define latest_weather printed_latest_weather\n#define latest_count printed_latest_count\n{printed}")).unwrap();
        for cc in &compilers {
            for opt in ["-O0", "-O2"] {
                let exe = dir.join(format!("roundtrip-{sample_cc}-{cc}-{opt}"));
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
    }
    std::fs::remove_dir_all(dir).unwrap();
}
