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
    if name == "kernelcache" {
        kuna_decomp::kuna_kernelcache::parse_kernelcache(value)
            .map_err(|e| format!("option kernelcache: {}", e.explain()))?;
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
}
