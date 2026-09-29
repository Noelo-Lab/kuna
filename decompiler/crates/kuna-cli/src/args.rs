//! Shared argument consumption for the CLI's command-specific parsers.

pub(crate) fn take_value(argv: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    let value = argv
        .get(*i + 1)
        .ok_or_else(|| format!("{flag} requires a value"))?;
    *i += 1;
    Ok(value.clone())
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
