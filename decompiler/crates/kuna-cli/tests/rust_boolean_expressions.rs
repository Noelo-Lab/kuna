//! Boolean bitwise expressions retain their values, eager calls, and byte stores.
use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn boolean_loop_expressions_preserve_values_calls_and_stores() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let kuna = std::env::var_os("KUNA_TEST_BIN")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let declarations = include_str!("../../kuna-analysis/tests/fixtures/rust_boolean_expressions.kuna");
    let bodies = |language: &str| {
        let mut command = Command::new(&kuna);
        command.args([
            "decompile-all", &common::fixture("rust_boolean_expressions_x86_64"),
            "--functions", "loop_xor,loop_and,loop_or,loop_eager,loop_and_call,loop_or_call,loop_store",
            "--language", language, "--json", "--assert-strict",
        ]);
        for declaration in declarations.lines() {
            command.args(["--assert", declaration]);
        }
        let output = process::required_output(&mut command);
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(document["error"].is_null(), "{document}");
        for assertion in document["assertions"].as_array().unwrap() {
            assert_eq!(assertion["status"], "applied", "{assertion}");
        }
        let functions = document["functions"].as_array().unwrap();
        assert_eq!(functions.len(), 7);
        functions.iter().map(|function| {
            assert!(function["error"].is_null(), "{function}");
            function["code"].as_str().unwrap()
        }).collect::<Vec<_>>().join("\n")
    };
    let c = bodies("c");
    let rust = bodies("rust");
    let c_source = common::scratch_file("rust_boolean_expressions_reference", "c");
    let rust_source = common::scratch_file("rust_boolean_expressions_emitted", "rs");
    std::fs::write(&c_source, format!(r#"
#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <limits.h>
#include <stdio.h>
static unsigned barriers, observations;
static uint64_t trace;
void barrier(void) {{ ++barriers; }}
bool observe(int value) {{ ++observations; trace=trace*17+(unsigned)value+1; return (value&1)!=0; }}
{c}
int main(void) {{
  const int values[]={{INT_MIN,-1,0,1,3,4,INT_MAX}};
  bool (*functions[])(int,int,int,void *)={{loop_xor,loop_and,loop_or,loop_eager,loop_and_call,loop_or_call}};
  unsigned char marker=7;
  for (unsigned ai=0;ai<7;++ai) for (unsigned bi=0;bi<7;++bi)
    for (int count=-2;count<=8;++count) for (unsigned null=0;null<2;++null) {{
      int a=values[ai],b=values[bi]; unsigned n=count>0?(unsigned)count:0;
      bool initial=a==b,flip=n>=4;
      bool expected[]={{(initial^flip)&null,(initial&&!flip)|null,(initial||flip)^null,
                        (initial^((n/2)%2!=0))&null,(initial&&n==0)&null,(initial||n>=2)&null}};
      uint64_t expected_trace=0;
      for (unsigned i=n;i>0;--i) expected_trace=expected_trace*17+i;
      for (unsigned kind=0;kind<6;++kind) {{
        barriers=observations=0; trace=0;
        bool result=functions[kind](a,b,count,null?NULL:&marker);
        assert(result==expected[kind]);
        assert(barriers==(kind<3?n:0) && observations==(kind<3?0:n));
        assert(trace==(kind<3?0:expected_trace));
        printf("%u %u %u %llu\n",result,barriers,observations,(unsigned long long)trace);
      }}
      for (unsigned seed=0;seed<2;++seed) {{
        struct {{unsigned char before; bool value; unsigned char after;}} cell={{0xa5,seed!=0,0x5a}};
        barriers=observations=0; trace=0;
        loop_store(&cell.value,a,b,count);
        assert(cell.before==0xa5 && cell.after==0x5a);
        assert(cell.value==(n?((initial^flip)&&(a==4)):(seed!=0)));
        assert(barriers==n && observations==0 && trace==0);
        printf("%u %u %u %llu\n",cell.value,barriers,observations,(unsigned long long)trace);
      }}
    }}
}}
"#)).unwrap();
    std::fs::write(&rust_source, format!(r#"
use std::sync::atomic::{{AtomicU64,AtomicUsize,Ordering}};
static BARRIERS:AtomicUsize=AtomicUsize::new(0);
static OBSERVATIONS:AtomicUsize=AtomicUsize::new(0);
static TRACE:AtomicU64=AtomicU64::new(0);
unsafe fn barrier() {{BARRIERS.fetch_add(1,Ordering::SeqCst);}}
unsafe fn observe(value:i32)->bool {{
  OBSERVATIONS.fetch_add(1,Ordering::SeqCst);
  TRACE.store(TRACE.load(Ordering::SeqCst)*17+value as u64+1,Ordering::SeqCst);
  value&1!=0
}}
fn reset() {{
  BARRIERS.store(0,Ordering::SeqCst);OBSERVATIONS.store(0,Ordering::SeqCst);
  TRACE.store(0,Ordering::SeqCst);
}}
fn observations()->(usize,usize,u64) {{
  (BARRIERS.load(Ordering::SeqCst),OBSERVATIONS.load(Ordering::SeqCst),TRACE.load(Ordering::SeqCst))
}}
#[repr(C)]
struct Cell {{before:u8,value:bool,after:u8}}
{rust}
fn main() {{unsafe {{
  let functions:[unsafe fn(i32,i32,i32,*mut())->bool;6]=
    [loop_xor,loop_and,loop_or,loop_eager,loop_and_call,loop_or_call];
  let values=[i32::MIN,-1,0,1,3,4,i32::MAX];let mut marker=7u8;
  for a in values {{for b in values {{for count in -2i32..=8 {{for null in [false,true] {{
    let n=count.max(0) as usize;let initial=a==b;let flip=n>=4;
    let expected=[(initial^flip)&null,(initial&&!flip)|null,(initial||flip)^null,
                  (initial^((n/2)%2!=0))&null,(initial&&n==0)&null,(initial||n>=2)&null];
    let mut expected_trace=0u64;
    for i in (1..=n).rev() {{expected_trace=expected_trace*17+i as u64;}}
    let p=if null {{core::ptr::null_mut()}} else {{&mut marker as *mut u8 as *mut ()}};
    for (kind,function) in functions.iter().enumerate() {{
      reset();let result=function(a,b,count,p);let (barriers,calls,trace)=observations();
      assert_eq!(result,expected[kind]);
      assert_eq!((barriers,calls,trace),if kind<3 {{(n,0,0)}} else {{(0,n,expected_trace)}});
      println!("{{}} {{}} {{}} {{}}",result as u8,barriers,calls,trace);
    }}
    for seed in [false,true] {{
      let mut cell=Cell{{before:0xa5,value:seed,after:0x5a}};
      reset();loop_store(&mut cell.value,a,b,count);let (barriers,calls,trace)=observations();
      assert_eq!((cell.before,cell.after),(0xa5,0x5a));
      assert_eq!(cell.value,if n>0 {{(initial^flip)&&(a==4)}} else {{seed}});
      assert_eq!((barriers,calls,trace),(n,0,0));
      println!("{{}} {{}} {{}} {{}}",cell.value as u8,barriers,calls,trace);
    }}
  }}}}}}}}
}}}}
"#)).unwrap();
    let c_binary = c_source.with_extension("out");
    let rust_binary = rust_source.with_extension("out");
    let mut expected = None;
    for optimization in ["0", "2"] {
        process::required_output(Command::new("cc")
            .args(["-std=c11", "-Wall", "-Wextra", "-Werror", "-Wno-parentheses", &format!("-O{optimization}")])
            .arg(&c_source).arg("-o").arg(&c_binary));
        let reference = process::required_output(&mut Command::new(&c_binary)).stdout;
        assert_eq!(String::from_utf8_lossy(&reference).lines().count(), 8624);
        if let Some(previous) = &expected { assert_eq!(&reference, previous); }
        process::required_output(Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "rust_boolean_expressions_emitted",
                "-C", &format!("opt-level={optimization}"), "-C", "overflow-checks=yes"])
            .arg(&rust_source).arg("-o").arg(&rust_binary));
        let actual = process::required_output(&mut Command::new(&rust_binary)).stdout;
        assert_eq!(actual, reference);
        expected = Some(reference);
    }
    for path in [c_source,rust_source,c_binary,rust_binary] {
        std::fs::remove_file(path).unwrap();
    }
}
