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
    *i += 2;
    Ok((name.clone(), value.clone()))
}
