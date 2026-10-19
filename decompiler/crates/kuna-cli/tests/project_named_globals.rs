//! Named references retain whole-object declarations in project exports.
use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(target_os = "linux")]
fn named_global_objects_compile_and_match_the_original_shared_library() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    for (fixture, object, initializer, peek_expression) in [
        ("static unsigned counter=37; unsigned sample(unsigned amount){counter+=amount;return counter;}", "counter", "37", "counter"),
        ("struct Pair {unsigned pad; unsigned value;}; static struct Pair record={13,37}; unsigned sample(unsigned amount){record.value+=amount;return record.value+record.pad;}", "record", "{13,37}", "record.value+record.pad"),
        ("static unsigned values[3]={13,37,5}; unsigned sample(unsigned amount){values[1]+=amount;return values[1]+values[2];}", "values", "{13,37,5}", "values[1]+values[2]"),
        ("static unsigned char bytes[4]={37,0,0,0}; unsigned sample(unsigned amount){unsigned value;__builtin_memcpy(&value,bytes,4);value+=amount;__builtin_memcpy(bytes,&value,4);return value;}", "bytes", "{37,0,0,0}", "({unsigned value;__builtin_memcpy(&value,bytes,4);value;})"),
    ] {
    let source = common::scratch_file("project-named-globals", "c");
    let binary = source.with_extension("so");
    let project = source.with_extension("project");
    let address_expression = match object {
        "record" => "&record.value",
        "values" => "&values[1]",
        "bytes" => "&bytes[0]",
        _ => "&counter",
    };
    std::fs::write(&source, format!("{fixture}\nunsigned peek(void){{return {peek_expression};}}\nvoid *address(void){{return {address_expression};}}\nconst char *text(void){{return \"fresh literal\";}}\n")).unwrap();
    if object == "bytes" {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new().append(true).open(&source).unwrap();
        writeln!(file, "void store(unsigned value){{__builtin_memcpy(bytes,&value,4);}}").unwrap();
    }
    let built = Command::new("gcc")
        .args(["-O0", "-shared", "-fPIC", "-g"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let (out, err, status) = common::run_kuna(&[
        "decompile-project",
        binary.to_str().unwrap(),
        "--jobs",
        "2",
        "--functions",
        "sample,peek,address,text",
        "--option",
        "bytearrayc",
        "on",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{out}\n{err}");
    let name = binary.file_name().unwrap().to_str().unwrap();
    let header_name = format!("{name}.h");
    let header = std::fs::read_to_string(project.join(&header_name)).unwrap();
    let globals: Vec<_> = header
        .lines()
        .filter_map(|line| line.strip_prefix("extern "))
        .collect();
    assert_eq!(globals.len(), 1, "{header}");
    let global = globals[0].strip_suffix(';').unwrap();
    assert!(global.contains(object), "{header}");
    let literal_project = source.with_extension("literal-project");
    let (literal_out, literal_err, literal_status) = common::run_kuna(&[
        "decompile-project", binary.to_str().unwrap(), "--functions", "text",
        "-o", literal_project.to_str().unwrap(),
    ]);
    assert_eq!(literal_status, 0, "{literal_out}\n{literal_err}");
    let literal_header = std::fs::read_to_string(literal_project.join(&header_name)).unwrap();
    assert!(!literal_header.lines().any(|line| line.starts_with("extern ")), "{literal_header}");
    let mut selections = vec!["address"];
    if object == "bytes" { selections.extend(["peek", "store"]); }
    for selected in selections {
        let isolated = source.with_extension(format!("isolated-{selected}"));
        let (isolated_out, isolated_err, isolated_status) = common::run_kuna(&[
            "decompile-project", binary.to_str().unwrap(), "--functions", selected,
            "--option", "bytearrayc", "on", "-o", isolated.to_str().unwrap(),
        ]);
        assert_eq!(isolated_status, 0, "{isolated_out}\n{isolated_err}");
        let isolated_header = std::fs::read_to_string(isolated.join(&header_name)).unwrap();
        let declarations: Vec<_> = isolated_header.lines().filter(|line| line.starts_with("extern ")).collect();
        assert_eq!(declarations.len(), 1, "{selected}: {isolated_header}");
        assert!(declarations[0].contains(object), "{selected}: {isolated_header}");
        let compiled_object = isolated.join("isolated.o");
        let compiled = Command::new("gcc").args(["-std=c11", "-Wall", "-Werror", "-c"])
            .arg(isolated.join(format!("{name}.c"))).arg("-o").arg(&compiled_object).output().unwrap();
        assert!(compiled.status.success(), "{selected}: {}", String::from_utf8_lossy(&compiled.stderr));
        std::fs::remove_dir_all(isolated).unwrap();
    }
    let initial_type = if object == "bytes" { "unsigned char" } else { "unsigned int" };
    let initial_values = if object == "counter" { "{37}" } else { initializer };
    let driver = source.with_extension("driver.c");
    let executable = source.with_extension("exe");
    std::fs::write(
        &driver,
        format!(
            r#"
#include "{header_name}"
#include <dlfcn.h>
#include <string.h>
{global};
static const {initial_type} initial[] = {initial_values};
_Static_assert(sizeof({object}) == sizeof(initial), "whole mapped object extent");
int main(int argc, char **argv) {{
    if (argc != 2) return 2;
    memcpy(&{object},initial,sizeof(initial));
    void *module=dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!module) return 3;
    unsigned (*original)(unsigned)=(unsigned (*)(unsigned))dlsym(module,"sample");
    if (!original) return 4;
    unsigned (*original_peek)(void)=(unsigned (*)(void))dlsym(module,"peek");
    void *(*original_address)(void)=(void *(*)(void))dlsym(module,"address");
    if (!original_peek || !original_address || strcmp(text(), "fresh literal")) return 5;
    for (int i=0;i<100;++i) {{
        int amount=i%16;
        if ((unsigned)sample(amount)!=original((unsigned)amount)) return 1;
        if (peek()!=original_peek()) return 6;
        unsigned actual_address_value, original_address_value;
        memcpy(&actual_address_value,address(),4);
        memcpy(&original_address_value,original_address(),4);
        if (actual_address_value!=original_address_value) return 7;
    }}
    dlclose(module);
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        for opt in ["-O0", "-O2"] {
            let built = Command::new(cc)
                .args(["-std=c11", "-Wall", "-Werror", opt])
                .arg("-I")
                .arg(&project)
                .arg(project.join(format!("{name}.c")))
                .arg(&driver)
                .arg("-ldl")
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{}\n{header}",
                String::from_utf8_lossy(&built.stderr)
            );
            assert!(
                Command::new(&executable)
                    .arg(&binary)
                    .status()
                    .unwrap()
                    .success(),
                "{cc} {opt}"
            );
        }
    }
    for path in [source, binary, driver, executable] {
        std::fs::remove_file(path).unwrap();
    }
    std::fs::remove_dir_all(project).unwrap();
    std::fs::remove_dir_all(literal_project).unwrap();
}
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn typed_interior_pointers_relocate_with_the_named_object() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("typed-global-pointer", "c");
    let binary = source.with_extension("so");
    let project = source.with_extension("project");
    std::fs::write(
        &source,
        r#"
unsigned char bytes[64];
int consume(int *p) { int value; __builtin_memcpy(&value,p,sizeof value); return value+11; }
int sample(void) { return consume((int *)(bytes+16)); }
int last(void) { return consume((int *)(bytes+60)); }
"#,
    )
    .unwrap();
    let built = Command::new("gcc")
        .args(["-O0", "-g", "-shared", "-fPIC", "-Wl,-Bsymbolic-functions"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let (_, error, status) = common::run_kuna(&[
        "decompile-project",
        binary.to_str().unwrap(),
        "--functions",
        "sample,last",
        "--assert",
        "prototype sample int sample(void)",
        "--assert",
        "prototype last int last(void)",
        "--assert",
        "prototype consume int consume(int *input)",
        "--assert-strict",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{error}");
    let name = binary.file_name().unwrap().to_str().unwrap();
    let generated = project.join(format!("{name}.c"));
    let header = std::fs::read_to_string(generated.with_extension("h")).unwrap();
    assert!(
        header.lines().any(|line| line.starts_with("extern ")
            && line.contains("bytes")
            && line.contains("64")),
        "{header}"
    );
    let global = header.lines().find_map(|line| {
        line.strip_prefix("extern ").filter(|decl| decl.contains("bytes") && decl.contains("64"))
    }).unwrap();
    let driver = source.with_extension("driver.c");
    let executable = source.with_extension("exe");
    std::fs::write(
        &driver,
        format!(
            r#"
#include <dlfcn.h>
#include <stdint.h>
#include <string.h>
int consume(int *input) {{ int value; memcpy(&value,input,4); return value+11; }}
#define sample regenerated_sample
#define last regenerated_last
#include "{}"
#undef sample
#undef last
{global}
int main(int argc,char **argv) {{
    if(argc!=2)return 90;
    void *image=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);
    if(!image)return 91;
    int (*original)(void)=dlsym(image,"sample");
    int (*original_last)(void)=dlsym(image,"last");
    unsigned char *reference=dlsym(image,"bytes");
    if(!original||!original_last||!reference||(void *)reference==(void *)bytes)return 92;
    uint32_t state=0x32645178;
    for(unsigned i=0;i<512;++i){{
        state=state*1664525u+1013904223u;
        int value=(int)(state%1000000u)-500000;
        memcpy(reference+16,&value,4);memcpy(bytes+16,&value,4);
        memcpy(reference+60,&value,4);memcpy(bytes+60,&value,4);
        if(regenerated_sample()!=original()||regenerated_last()!=original_last())return 1;
        int distinct=value+37;memcpy(bytes+16,&distinct,4);
        if(regenerated_sample()!=distinct+11||original()!=value+11)return 2;
    }}
    return 0;
}}
"#,
            generated.display()
        ),
    )
    .unwrap();
    let mut ran = false;
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        ran = true;
        for level in ["-O0", "-O2"] {
            let built = Command::new(compiler)
                .args(["-std=gnu11", "-Wall", "-Werror", level])
                .arg(&driver)
                .arg("-ldl")
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{compiler} {level}: {}",
                String::from_utf8_lossy(&built.stderr)
            );
            assert!(
                Command::new(&executable)
                    .arg(&binary)
                    .status()
                    .unwrap()
                    .success(),
                "{compiler} {level}: global pointers did not relocate"
            );
        }
    }
    assert!(ran);
    for file in [source, binary, driver, executable] {
        std::fs::remove_file(file).unwrap();
    }
    std::fs::remove_dir_all(project).unwrap();
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn named_scalars_are_declared_at_the_type_the_bodies_read_them() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("named-scalar-types", "c");
    let binary = source.with_extension("so");
    let project = source.with_extension("project");
    std::fs::write(
        &source,
        r#"
int count;
long total = -5;
double scale = 1.5;
long loaded;
int step(int x) { count += x; if (count < 0) return -1; return count / 3; }
long shift(long x) { total = total * x - 7; return total >> 2; }
double grow(double k) { scale = scale * k - k; return scale; }
long load(long *p, long b) { loaded = *p; if (loaded < b) return loaded / 3; return loaded >> 1; }
"#,
    )
    .unwrap();
    let built = Command::new("gcc")
        .args(["-O0", "-shared", "-fPIC", "-fno-strict-aliasing"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
    let (out, err, status) = common::run_kuna(&[
        "decompile-project",
        binary.to_str().unwrap(),
        "--functions",
        "step,shift,grow,load",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{out}\n{err}");
    let name = binary.file_name().unwrap().to_str().unwrap();
    let header = std::fs::read_to_string(project.join(format!("{name}.h"))).unwrap();
    let definitions: Vec<_> = header
        .lines()
        .filter_map(|line| line.strip_prefix("extern "))
        .filter(|decl| ["count;", "total;", "scale;", "loaded;"].iter().any(|n| decl.ends_with(&format!(" {n}"))))
        .collect();
    for expected in ["int count;", "long total;", "double scale;", "long loaded;"] {
        assert!(definitions.contains(&expected), "{expected}\n{header}");
    }
    let driver = source.with_extension("driver.c");
    let executable = source.with_extension("exe");
    std::fs::write(
        &driver,
        format!(
            r#"
#include "{name}.h"
#include <dlfcn.h>
#include <string.h>
{}
int main(int argc, char **argv) {{
    void *image = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!image) return 90;
    memcpy(&count, dlsym(image, "count"), sizeof count);
    memcpy(&total, dlsym(image, "total"), sizeof total);
    memcpy(&scale, dlsym(image, "scale"), sizeof scale);
    int (*original_step)(int) = (int (*)(int))dlsym(image, "step");
    long (*original_shift)(long) = (long (*)(long))dlsym(image, "shift");
    double (*original_grow)(double) = (double (*)(double))dlsym(image, "grow");
    long (*original_load)(long *, long) = (long (*)(long *, long))dlsym(image, "load");
    long (*regenerated_load)(long *, long) = (long (*)(long *, long))load;
    for (int i = 0; i < 200; ++i) {{
        int amount = (i * 37) % 21 - 10;
        if (step(amount) != original_step(amount)) return 1;
        if (shift(amount % 3) != original_shift(amount % 3)) return 2;
        if (grow(0.5 + (i % 3)) != original_grow(0.5 + (i % 3))) return 3;
        long seed = amount * 1000003L;
        if (regenerated_load(&seed, amount % 5) != original_load(&seed, amount % 5)) return 4;
        if (memcmp(&loaded, dlsym(image, "loaded"), sizeof loaded)) return 5;
    }}
    return 0;
}}
"#,
            definitions.join("\n")
        ),
    )
    .unwrap();
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        for opt in ["-O0", "-O2"] {
            let built = Command::new(cc)
                .args(["-std=c11", "-Wall", "-Werror", "-fno-strict-aliasing", opt])
                .arg("-I")
                .arg(&project)
                .arg(project.join(format!("{name}.c")))
                .arg(&driver)
                .arg("-ldl")
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(built.status.success(), "{cc} {opt}: {}\n{header}", String::from_utf8_lossy(&built.stderr));
            let ran = Command::new(&executable).arg(&binary).status().unwrap();
            assert!(ran.success(), "{cc} {opt}: {ran}\n{header}");
        }
    }
    for path in [source, binary, driver, executable] {
        std::fs::remove_file(path).unwrap();
    }
    std::fs::remove_dir_all(project).unwrap();
}
