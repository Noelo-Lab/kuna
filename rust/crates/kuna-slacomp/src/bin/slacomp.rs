//! The `slacomp` binary -- the Rust `sleigh_opt` replacement.
//!
//! CLI contract (mirrors C++ `sleigh_opt`, slgh_compile.cc:3926-4090):
//!
//! ```text
//! slacomp [-options] inputfile [outputfile]
//!   -a              recurse: `inputfile` is a directory; compile every *.slaspec
//!   -y              write .sla in XML debug format
//!   -u -l -n -t -e -c -s   warning/strictness toggles (see usage below)
//!   -DNAME=VALUE    define a preprocessor macro
//! ```
//!
//! With a single `<file.slaspec>` and no output given, writes `<file>.sla` next
//! to it.  The Python differential harness (`kuna/slacomp.py`) drives this binary
//! and byte-compares the result against `sleigh_opt`'s.
//!
//! The argument parsing here is real (so the harness wiring is exercised); the
//! actual compilation is delegated to
//! [`SleighCompile::run_compilation`](kuna_slacomp::slgh_compile::SleighCompile::run_compilation),
//! whose body lands in WS4.

use std::collections::BTreeMap;
use std::process::ExitCode;

use kuna_base::filemanage::FileManage;
use kuna_slacomp::slgh_compile::SleighCompile;

const SLAEXT: &str = ".sla";
const SLASPECEXT: &str = ".slaspec";

fn usage() {
    eprintln!("USAGE: slacomp [-x] [-dNAME=VALUE] inputfile [outputfile]");
    eprintln!("   -a              scan for all slaspec files recursively where inputfile is a directory");
    eprintln!("   -y              write .sla using XML debug format");
    eprintln!("   -u              print warnings for unnecessary pcode instructions");
    eprintln!("   -l              report pattern conflicts");
    eprintln!("   -n              print warnings for all NOP constructors");
    eprintln!("   -t              print warnings for dead temporaries");
    eprintln!("   -e              enforce use of 'local' keyword for temporaries");
    eprintln!("   -c              print warnings for all constructors with colliding operands");
    eprintln!("   -s              treat register names as case sensitive");
    eprintln!("   -DNAME=VALUE    defines a preprocessor macro NAME with value VALUE");
}

/// Compile one `.slaspec` to its sibling `.sla`, applying the parsed options
/// (mirrors the single-file branch of C++ `main`, slgh_compile.cc:4048-4090).
fn compile_one(
    slaspec: &str,
    sla_out: &str,
    opts: &CompileOptions,
) -> i32 {
    let mut compiler = SleighCompile::new();
    compiler.set_all_options(
        &opts.defines,
        opts.unnecessary_pcode_warning,
        opts.lenient_conflict,
        opts.all_collision_warning,
        opts.all_nop_warning,
        opts.dead_temp_warning,
        opts.enforce_local_keyword,
        opts.case_sensitive_register_names,
        opts.debug_output,
    );
    match compiler.run_compilation(slaspec, sla_out) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Unrecoverable error: {e:?}");
            2
        }
    }
}

/// Parsed command-line options (the toggles from C++ `main`).
#[derive(Default)]
struct CompileOptions {
    defines: BTreeMap<Vec<u8>, Vec<u8>>,
    unnecessary_pcode_warning: bool,
    lenient_conflict: bool,
    all_collision_warning: bool,
    all_nop_warning: bool,
    dead_temp_warning: bool,
    enforce_local_keyword: bool,
    case_sensitive_register_names: bool,
    debug_output: bool,
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        usage();
        return ExitCode::from(2);
    }

    let mut opts = CompileOptions {
        lenient_conflict: true, // C++ default
        ..Default::default()
    };
    let mut compile_all = false;

    // Parse leading `-` options (slgh_compile.cc:3970-4009).
    let mut i = 1usize;
    while i < argv.len() {
        let arg = &argv[i];
        if !arg.starts_with('-') {
            break;
        }
        let flag = arg.as_bytes().get(1).copied().unwrap_or(b'\0');
        match flag {
            b'a' => compile_all = true,
            b'D' => {
                let preproc = &arg[2..];
                match preproc.find('=') {
                    Some(pos) => {
                        let name = preproc.as_bytes()[..pos].to_vec();
                        let value = preproc.as_bytes()[pos + 1..].to_vec();
                        opts.defines.insert(name, value);
                    }
                    None => {
                        eprintln!("Bad sleigh option: {arg}");
                        return ExitCode::from(1);
                    }
                }
            }
            b'u' => opts.unnecessary_pcode_warning = true,
            b'l' => opts.lenient_conflict = false,
            b'c' => opts.all_collision_warning = true,
            b'n' => opts.all_nop_warning = true,
            b't' => opts.dead_temp_warning = true,
            b'e' => opts.enforce_local_keyword = true,
            b's' => opts.case_sensitive_register_names = true,
            b'y' => opts.debug_output = true,
            _ => {
                eprintln!("Unknown option: {arg}");
                return ExitCode::from(1);
            }
        }
        i += 1;
    }

    let mut retval = 0i32;

    if compile_all {
        // Recurse a directory for *.slaspec (slgh_compile.cc:4011-4045).
        if i < argv.len() - 1 {
            eprintln!("Too many parameters");
            return ExitCode::from(1);
        }
        let dir = if i != argv.len() { argv[i].clone() } else { ".".to_string() };
        let mut slaspecs: Vec<String> = Vec::new();
        find_sla_specs(&mut slaspecs, &dir, SLASPECEXT);
        println!("Compiling {} slaspec files in {}", slaspecs.len(), dir);
        for spec in &slaspecs {
            let sla = format!("{}{}", &spec[..spec.len() - SLASPECEXT.len()], SLAEXT);
            println!("Compiling {spec}:");
            let res = compile_one(spec, &sla, &opts);
            if res != 0 {
                retval = res;
            }
        }
    } else {
        // Single-file mode (slgh_compile.cc:4047-4090).
        if i >= argv.len() {
            usage();
            return ExitCode::from(2);
        }
        let filein = argv[i].clone();
        // Normalize input extension to .slaspec.
        let slaspec = if filein.ends_with(SLASPECEXT) {
            filein.clone()
        } else if filein.contains('.') {
            eprintln!("Unknown input file type: {filein}");
            return ExitCode::from(1);
        } else {
            format!("{filein}{SLASPECEXT}")
        };
        // Output: explicit arg, else sibling .sla.
        let sla_out = if i + 1 < argv.len() {
            let out = &argv[i + 1];
            if out.ends_with(SLAEXT) {
                out.clone()
            } else if out.contains('.') {
                eprintln!("Unknown output file type: {out}");
                return ExitCode::from(1);
            } else {
                format!("{out}{SLAEXT}")
            }
        } else {
            format!("{}{}", &slaspec[..slaspec.len() - SLASPECEXT.len()], SLAEXT)
        };
        retval = compile_one(&slaspec, &sla_out, &opts);
    }

    ExitCode::from(retval as u8)
}

/// Recursively collect every `*.slaspec` under `dir` (`findSlaSpecs`,
/// slgh_compile.cc:3876-3888).
fn find_sla_specs(res: &mut Vec<String>, dir: &str, suffix: &str) {
    FileManage::match_list_dir(res, suffix, true, dir, false);
    let mut dirs: Vec<String> = Vec::new();
    FileManage::directory_list(&mut dirs, dir, false);
    for nextdir in &dirs {
        find_sla_specs(res, nextdir, suffix);
    }
}
