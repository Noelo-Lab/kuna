//! Callback writes update a by-value struct spilled across entry SP=0.
#![cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::common;
use common::process;
use std::process::Command;

const PROTOTYPES: &[(&str, &str)] = &[
    ("readtemp0", "short readtemp0(struct dev d)"),
    ("readtemp1", "short readtemp1(int a,struct dev d)"),
    ("readtemp2", "short readtemp2(int a,int b,struct dev d)"),
    (
        "readtemp3",
        "short readtemp3(int a,int b,int c,struct dev d)",
    ),
    (
        "readtemp4",
        "short readtemp4(int a,int b,int c,int e,struct dev d)",
    ),
    ("readfields", "int readfields(struct dev d)"),
    ("readother", "short readother(struct other d)"),
    ("readboth", "short readboth(struct dev d,struct other e)"),
];

#[test]
fn wrapped_struct_spills_preserve_callback_memory_effects() {
    let source = std::fs::read_to_string(common::fixture("wrappedstackaggregate.c")).unwrap();
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "the round trip requires a C compiler"
    );
    let pieces = regex::Regex::new(r"(\w+)\._(\d+)_(\d+)_ = (\w+)\._(\d+)_(\d+)_;").unwrap();
    for target in ["thumb", "arm_be"] {
        for opt in ["O0", "O2"] {
            let fixture = format!("wrappedstackaggregate_clang_{opt}_{target}.o");
            let names: Vec<_> = PROTOTYPES.iter().map(|(name, _)| *name).collect();
            let mut args = vec![
                "decompile-all".to_owned(),
                common::fixture(&fixture),
                "--functions".into(),
                names.join(","),
                "--assert-strict".into(),
            ];
            for name in ["dev", "other"] {
                args.extend(["--assert".into(), format!("typedef struct {name} {{ void (*temp)(struct {name} *,short *);int pad[16];short t; }};")]);
            }
            for (name, proto) in PROTOTYPES {
                args.extend(["--assert".into(), format!("prototype {name} {proto}")]);
            }
            let (printed, stderr, code) =
                common::run_kuna(&args.iter().map(String::as_str).collect::<Vec<_>>());
            assert_eq!(code, 0, "{fixture}: {stderr}");
            let printed = pieces.replace_all(&printed, |m: &regex::Captures<'_>| {
                assert_eq!(m[2], m[5]);
                assert_eq!(m[3], m[6]);
                format!(
                    "__builtin_memcpy((char *)&{}+{},(char *)&{}+{},{});",
                    &m[1], &m[2], &m[4], &m[5], &m[3]
                )
            });
            let mut reference = source.clone();
            for name in names {
                reference = reference.replace(&format!("{name}("), &format!("source_{name}("));
            }
            let mut checks = String::new();
            for prefix in 0..5 {
                let mut args: Vec<_> = (0..prefix).map(|i| (i + 2).to_string()).collect();
                args.push("d".into());
                let args = args.join(",");
                checks.push_str(&format!(
                    "if(readtemp{prefix}({args})!=source_readtemp{prefix}({args}))return {};",
                    prefix + 10
                ));
            }
            let program = format!(
                r#"
typedef struct dev dev; typedef struct other other;
{reference}
void *memcpy(void*d,const void*s,unsigned n){{char*t=d;const char*f=s;for(unsigned i=0;i<n;i++)t[i]=f[i];return d;}}
{printed}
static int bad;
void touch_dev(dev*d){{d->t+=19;}}
static void change(dev*d,short*t){{if(t!=&d->t)bad=1;for(int i=0;i<16;i++){{if(d->pad[i]!=100+i)bad=2;d->pad[i]+=i;}}*t+=31;d->t+=7;}}
static void other_change(other*d,short*t){{if(t!=&d->t)bad=3;*t+=13;}}
static int driver(void){{for(int c=0;c<2;c++)for(int t=-3;t<=3;t++){{
dev d;d.temp=c?change:0;for(int i=0;i<16;i++)d.pad[i]=100+i;d.t=t;
{checks}
if(readfields(d)!=source_readfields(d))return 20;
other e;e.temp=c?other_change:0;e.t=9;for(int i=0;i<16;i++)e.pad[i]=200+i;
if(readother(e)!=source_readother(e))return 21;
if(readboth(d,e)!=source_readboth(d,e))return 22;
}}return bad;}}
void _start(void){{int r=driver();__asm__ volatile("mov %0,%%ebx;mov $1,%%eax;int $0x80"::"r"(r):"eax","ebx");}}
"#
            );
            let src = common::scratch_file("wrappedstackaggregate", "c");
            let exe = common::scratch_file("wrappedstackaggregate", "exe");
            std::fs::write(&src, program).unwrap();
            for cc in &compilers {
                for host_opt in ["-O0", "-O2"] {
                    let compile = Command::new(cc)
                        .args([
                            "-m32",
                            host_opt,
                            "-fno-builtin",
                            "-fno-stack-protector",
                            "-fno-pie",
                            "-no-pie",
                            "-nostdlib",
                            "-static",
                            "-Wl,-e,_start",
                        ])
                        .arg(&src)
                        .arg("-o")
                        .arg(&exe)
                        .output()
                        .expect("compile printed C");
                    assert!(
                        compile.status.success(),
                        "{fixture} {cc} {host_opt}: {}\n{printed}",
                        String::from_utf8_lossy(&compile.stderr)
                    );
                    let run = Command::new(&exe).output().expect("run printed C");
                    assert!(
                        run.status.success(),
                        "{fixture} {cc} {host_opt}: {:?}\n{printed}",
                        run.status
                    );
                }
            }
            let _ = std::fs::remove_file(src);
            let _ = std::fs::remove_file(exe);
        }
    }
}

#[test]
fn an_untyped_caller_of_a_typed_struct_pointer_keeps_decompiling() {
    for target in ["thumb", "arm_be"] {
        for opt in ["O0", "O2"] {
            let fixture = format!("wrappedstackaggregate_clang_{opt}_{target}.o");
            let (printed, stderr, code) = common::run_kuna(&[
                "decompile",
                &common::fixture(&fixture),
                "untyped",
                "--assert-strict",
                "--assert",
                "typedef struct dev {void (*temp)(struct dev *,short *);int pad[16];short t;};",
                "--assert",
                "prototype touch_dev void touch_dev(struct dev *d)",
            ]);
            assert_eq!(code, 0, "{fixture}: {stderr}");
            assert!(printed.contains("untyped("), "{fixture}: {printed}");
            assert!(!printed.contains("LOSS-131"), "{fixture}: {printed}");
        }
    }
}
