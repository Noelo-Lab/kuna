//! Shared argument consumption for the CLI's command-specific parsers.

pub(crate) fn take_value(argv: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    let value = argv
        .get(*i + 1)
        .ok_or_else(|| format!("{flag} requires a value"))?;
    *i += 1;
    Ok(value.clone())
}

/// Remove a global `--image NAME` from `argv` and export it to the loader,
/// which rebuilds that image when the input is a dyld shared cache. Every
/// subcommand and every `decomp_dbg` subprocess it spawns reads the same
/// variable, the way `--slice` reaches them.
pub(crate) fn take_global_image(argv: &mut Vec<String>) -> Result<(), String> {
    let mut i = 0;
    while i < argv.len() {
        let value = if argv[i] == "--image" {
            let value = argv.get(i + 1).cloned().ok_or("--image requires a value")?;
            argv.drain(i..i + 2);
            value
        } else if let Some(value) = argv[i].strip_prefix("--image=") {
            let value = value.to_string();
            argv.remove(i);
            value
        } else {
            i += 1;
            continue;
        };
        kuna_decomp::kuna_dyldcache::set_dyldimage_env(&value);
    }
    Ok(())
}

/// Rewrite the global `--deobfuscate LIST` as the `--option deobfuscate LIST`
/// it abbreviates, so every subcommand that takes options takes it.  LIST is
/// `auto`, `off`, or deobfuscation techniques separated by commas.
pub(crate) fn lower_deobfuscate(argv: &mut Vec<String>) -> Result<(), String> {
    const FLAG: &str = "--deobfuscate";
    let mut i = 0;
    while i < argv.len() {
        let value = if argv[i] == FLAG {
            let value = argv.get(i + 1).cloned().ok_or("--deobfuscate requires a value")?;
            argv.remove(i + 1);
            value
        } else if let Some(value) = argv[i].strip_prefix("--deobfuscate=") {
            value.to_string()
        } else {
            i += 1;
            continue;
        };
        check_deobfuscate(FLAG, &value)?;
        argv.splice(i..=i, ["--option".to_string(), "deobfuscate".to_string(), value]);
        i += 3;
    }
    Ok(())
}

fn check_deobfuscate(what: &str, value: &str) -> Result<(), String> {
    kuna_decomp::kuna_deobf::Selection::parse(value).map(drop).map_err(|e| format!("{what}: {}", e.explain()))
}

pub(crate) fn report(result: Result<i32, String>) -> i32 {
    match result {
        Ok(status) => status,
        Err(error) => {
            eprintln!("error: {error}");
            2
        }
    }
}

pub(crate) fn take_option(argv: &[String], i: &mut usize) -> Result<(String, String), String> {
    if *i + 2 >= argv.len() {
        return Err("--option requires NAME VALUE".into());
    }
    let name = &argv[*i + 1];
    crate::optname::check(name)?;
    let value = &argv[*i + 2];
    if name == "kernelcache" {
        kuna_decomp::kuna_kernelcache::parse_kernelcache(value)
            .map_err(|e| format!("option kernelcache: {}", e.explain()))?;
    }
    if name == kuna_decomp::kuna_deobf::Selection::NAME {
        check_deobfuscate("option deobfuscate", value)?;
    }
    *i += 2;
    Ok((name.clone(), value.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `kernelcache` value is refused before it can reach a console script,
    /// which would read only its first word.
    #[test]
    fn kernelcache_values_are_checked_whole() {
        let argv = |v: &str| ["--option", "kernelcache", v].map(String::from).to_vec();
        let mut i = 0;
        assert!(take_option(&argv("two words"), &mut i).unwrap_err().contains("kernelcache"));
        let mut i = 0;
        assert_eq!(take_option(&argv("IOSurface"), &mut i).unwrap().1, "IOSurface");
    }

    /// `--deobfuscate LIST`, in either spelling, becomes the option it
    /// abbreviates, and a technique that does not exist is refused by name.
    #[test]
    fn deobfuscate_lowers_to_its_option() {
        let lower = |args: &[&str]| {
            let mut argv: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            lower_deobfuscate(&mut argv).map(|()| argv.join(" "))
        };
        assert_eq!(lower(&["bin", "--deobfuscate", "devirtualize", "f"]).unwrap(), "bin --option deobfuscate devirtualize f");
        assert_eq!(lower(&["bin", "f", "--deobfuscate=off"]).unwrap(), "bin f --option deobfuscate off");
        assert_eq!(lower(&["bin", "--deobfuscate", "auto", "--deobfuscate=devirtualize"]).unwrap(), "bin --option deobfuscate auto --option deobfuscate devirtualize");
        assert_eq!(lower(&["bin", "f"]).unwrap(), "bin f");
        let err = lower(&["bin", "--deobfuscate", "devirtualize,unpack"]).unwrap_err();
        assert!(err.starts_with("--deobfuscate:") && err.contains("\"unpack\"") && err.contains("devirtualize"), "{err}");
        assert!(lower(&["bin", "--deobfuscate"]).unwrap_err().contains("requires a value"));
        let argv: Vec<String> = ["--option", "deobfuscate", "on"].map(String::from).to_vec();
        assert!(take_option(&argv, &mut 0).unwrap_err().starts_with("option deobfuscate:"));
    }
}
