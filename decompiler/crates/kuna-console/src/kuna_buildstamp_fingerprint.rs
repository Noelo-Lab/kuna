use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const UNHASHED_DIRS: [&str; 4] = ["tests", "benches", "examples", "target"];

#[derive(Debug)]
pub struct Fingerprint {
    pub value: u64,
    pub watched: BTreeSet<PathBuf>,
}

pub fn calculate(workspace: &Path, root: &str) -> io::Result<Fingerprint> {
    let crates = workspace.join("crates");
    let mut files = Vec::new();
    let mut watched = BTreeSet::new();
    for name in linked_crates(workspace, &crates, root)? {
        collect(&crates.join(&name), &name, true, &mut files, &mut watched)?;
    }
    for name in ["Cargo.toml", "Cargo.lock"] {
        let path = workspace.join(name);
        if !path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "required build identity input is missing: {}",
                    path.display()
                ),
            ));
        }
        watched.insert(path.clone());
        files.push((name.to_string(), path));
    }
    files.sort();

    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |b: u8| hash = (hash ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
    for (rel, path) in &files {
        rel.bytes().for_each(&mut feed);
        feed(0);
        fs::read(path)?
            .into_iter()
            .filter(|&b| b != b'\r')
            .for_each(&mut feed);
        feed(0);
    }
    Ok(Fingerprint {
        value: hash,
        watched,
    })
}

fn linked_crates(workspace: &Path, crates: &Path, root: &str) -> io::Result<BTreeSet<String>> {
    let workspace_dependencies = workspace_dependencies(&workspace.join("Cargo.toml"))?;
    let mut seen = BTreeSet::new();
    let mut todo = vec![root.to_string()];
    while let Some(name) = todo.pop() {
        if !seen.insert(name.clone()) {
            continue;
        }
        let path = crates.join(&name).join("Cargo.toml");
        let manifest = fs::read_to_string(&path)?;
        for package in manifest_dependencies(
            &manifest,
            path.parent().unwrap_or(crates),
            &workspace_dependencies,
        )? {
            if package.starts_with("kuna-") {
                let dependency = crates.join(&package).join("Cargo.toml");
                if !dependency.is_file() {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "workspace dependency {package} has no manifest at {}",
                            dependency.display()
                        ),
                    ));
                }
                todo.push(package);
            }
        }
    }
    Ok(seen)
}

fn workspace_dependencies(path: &Path) -> io::Result<BTreeMap<String, String>> {
    let manifest = fs::read_to_string(path)?;
    let manifest_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut result = BTreeMap::new();
    let mut in_dependencies = false;
    let mut dependency_table: Option<(String, Option<String>)> = None;
    for raw in manifest.lines() {
        let line = without_comment(raw).trim();
        if line.starts_with('[') {
            finish_workspace_dependency_table(&mut result, dependency_table.take());
            in_dependencies = line == "[workspace.dependencies]";
            dependency_table = workspace_dependency_table_alias(line)?.map(|alias| (alias, None));
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if let Some((_, package)) = dependency_table.as_mut() {
            if let Some((key, value)) = key_value(line)? {
                let key = first_key(&key)?;
                if key == "package" {
                    *package = Some(toml_string(value)?.ok_or_else(|| {
                        invalid_data("workspace dependency package must be a string")
                    })?);
                } else if key == "path" {
                    if package.is_none() {
                        *package = package_at_path(manifest_dir, value)?;
                    }
                }
            }
            continue;
        }
        if !in_dependencies {
            continue;
        }
        let Some((key, value)) = key_value(line)? else {
            continue;
        };
        let alias = first_key(&key)?;
        let package =
            dependency_value_package(value, manifest_dir)?.unwrap_or_else(|| alias.clone());
        result.insert(alias, package);
    }
    finish_workspace_dependency_table(&mut result, dependency_table);
    Ok(result)
}

fn finish_workspace_dependency_table(
    result: &mut BTreeMap<String, String>,
    table: Option<(String, Option<String>)>,
) {
    let Some((alias, package)) = table else {
        return;
    };
    let package = package.unwrap_or_else(|| alias.clone());
    result.insert(alias, package);
}

fn manifest_dependencies(
    manifest: &str,
    manifest_dir: &Path,
    workspace_dependencies: &BTreeMap<String, String>,
) -> io::Result<BTreeSet<String>> {
    let mut result = BTreeSet::new();
    let mut in_dependencies = false;
    let mut dependency_table: Option<(String, Option<String>)> = None;
    for raw in manifest.lines() {
        let line = without_comment(raw).trim();
        if line.starts_with('[') {
            finish_dependency_table(&mut result, dependency_table.take(), workspace_dependencies);
            in_dependencies = dependency_section(line);
            dependency_table = dependency_table_alias(line)?.map(|alias| (alias, None));
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if let Some((_, package)) = dependency_table.as_mut() {
            if let Some((key, value)) = key_value(line)? {
                let key = first_key(&key)?;
                if key == "package" {
                    *package = Some(
                        toml_string(value)?
                            .ok_or_else(|| invalid_data("dependency package must be a string"))?,
                    );
                } else if key == "path" {
                    if package.is_none() {
                        *package = package_at_path(manifest_dir, value)?;
                    }
                }
            }
            continue;
        }
        if !in_dependencies {
            continue;
        }
        let Some((key, value)) = key_value(line)? else {
            continue;
        };
        let alias = first_key(&key)?;
        let package = dependency_value_package(value, manifest_dir)?;
        insert_dependency(&mut result, &alias, package, workspace_dependencies);
    }
    finish_dependency_table(&mut result, dependency_table, workspace_dependencies);
    Ok(result)
}

fn finish_dependency_table(
    result: &mut BTreeSet<String>,
    table: Option<(String, Option<String>)>,
    workspace_dependencies: &BTreeMap<String, String>,
) {
    if let Some((alias, package)) = table {
        insert_dependency(result, &alias, package, workspace_dependencies);
    }
}

fn insert_dependency(
    result: &mut BTreeSet<String>,
    alias: &str,
    package: Option<String>,
    workspace_dependencies: &BTreeMap<String, String>,
) {
    if let Some(package) = package.or_else(|| workspace_dependencies.get(alias).cloned()) {
        if package.starts_with("kuna-") {
            result.insert(package);
        }
    }
}

fn dependency_section(header: &str) -> bool {
    header == "[dependencies]"
        || (header.starts_with("[target.") && header.ends_with(".dependencies]"))
}

fn dependency_table_alias(header: &str) -> io::Result<Option<String>> {
    let Some(inner) = header.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
        return Ok(None);
    };
    let suffix = if let Some(suffix) = inner.strip_prefix("dependencies.") {
        Some(suffix)
    } else {
        inner.split_once(".dependencies.").map(|(_, suffix)| suffix)
    };
    suffix.map(first_key).transpose()
}

fn workspace_dependency_table_alias(header: &str) -> io::Result<Option<String>> {
    let Some(inner) = header.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
        return Ok(None);
    };
    inner
        .strip_prefix("workspace.dependencies.")
        .map(first_key)
        .transpose()
}

fn dependency_value_package(value: &str, manifest_dir: &Path) -> io::Result<Option<String>> {
    if let Some(package) = string_field(value, "package")? {
        return Ok(Some(package));
    }
    let Some(path) = string_field(value, "path")? else {
        return Ok(None);
    };
    package_at_path(manifest_dir, &format!("\"{path}\""))
}

fn package_at_path(manifest_dir: &Path, value: &str) -> io::Result<Option<String>> {
    let Some(relative) = toml_string(value)? else {
        return Err(invalid_data("dependency path must be a string"));
    };
    let manifest = manifest_dir.join(relative).join("Cargo.toml");
    let text = fs::read_to_string(&manifest)?;
    package_name(&text)
        .ok_or_else(|| {
            invalid_data(format!(
                "path dependency has no package name in {}",
                manifest.display()
            ))
        })
        .map(Some)
}

fn package_name(manifest: &str) -> Option<String> {
    let mut in_package = false;
    for raw in manifest.lines() {
        let line = without_comment(raw).trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package {
            continue;
        }
        let Ok(Some((key, value))) = key_value(line) else {
            continue;
        };
        if first_key(&key).ok().as_deref() == Some("name") {
            return toml_string(value).ok().flatten();
        }
    }
    None
}

fn without_comment(line: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;
    for (index, ch) in line.char_indices() {
        if quote == Some('"') && ch == '\\' && !escaped {
            escaped = true;
            continue;
        }
        if quote == Some(ch) && !escaped {
            quote = None;
        } else if quote.is_none() && (ch == '"' || ch == '\'') {
            quote = Some(ch);
        } else if ch == '#' && quote.is_none() {
            return &line[..index];
        }
        escaped = false;
    }
    line
}

fn key_value(line: &str) -> io::Result<Option<(String, &str)>> {
    let mut quote = None;
    let mut escaped = false;
    for (index, ch) in line.char_indices() {
        if quote == Some('"') && ch == '\\' && !escaped {
            escaped = true;
            continue;
        }
        if quote == Some(ch) && !escaped {
            quote = None;
        } else if quote.is_none() && (ch == '"' || ch == '\'') {
            quote = Some(ch);
        } else if ch == '=' && quote.is_none() {
            return Ok(Some((
                line[..index].trim().to_string(),
                line[index + 1..].trim(),
            )));
        }
        escaped = false;
    }
    if quote.is_some() {
        return Err(invalid_data("unterminated quoted key in Cargo manifest"));
    }
    Ok(None)
}

fn first_key(key: &str) -> io::Result<String> {
    let key = key.trim();
    if matches!(key.chars().next(), Some('"' | '\'')) {
        let quote = key.chars().next().unwrap_or('"');
        let rest = &key[quote.len_utf8()..];
        let end = rest
            .find(quote)
            .ok_or_else(|| invalid_data("unterminated quoted Cargo key"))?;
        return Ok(rest[..end].to_string());
    }
    let bare: String = key
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    if bare.is_empty() {
        return Err(invalid_data(format!("unsupported Cargo key syntax: {key}")));
    }
    Ok(bare)
}

fn string_field(value: &str, field: &str) -> io::Result<Option<String>> {
    let Some(inner) = value
        .trim()
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
    else {
        return Ok(None);
    };
    for item in inner.split(',') {
        let Some((key, value)) = key_value(item.trim())? else {
            continue;
        };
        if first_key(&key)? == field {
            return toml_string(value);
        }
    }
    Ok(None)
}

fn toml_string(value: &str) -> io::Result<Option<String>> {
    let value = value.trim();
    let Some(quote) = value.chars().next().filter(|c| *c == '"' || *c == '\'') else {
        return Ok(None);
    };
    let rest = &value[quote.len_utf8()..];
    let end = rest
        .find(quote)
        .ok_or_else(|| invalid_data("unterminated Cargo string"))?;
    Ok(Some(rest[..end].to_string()))
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn collect(
    dir: &Path,
    rel: &str,
    top: bool,
    files: &mut Vec<(String, PathBuf)>,
    watched: &mut BTreeSet<PathBuf>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') || (top && UNHASHED_DIRS.contains(&name.as_str())) {
            continue;
        }
        let path = entry.path();
        if top {
            watched.insert(path.clone());
        }
        let rel = format!("{rel}/{name}");
        if path.is_dir() {
            collect(&path, &rel, false, files, watched)?;
        } else if path.is_file() {
            files.push((rel, path));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "kuna_buildstamp_fingerprint_{}_{}",
            std::process::id(),
            tag
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("crates/kuna-console/src")).unwrap();
        fs::create_dir_all(root.join("crates/kuna-core/src")).unwrap();
        fs::create_dir_all(root.join("crates/kuna-harness/src")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/*\"]\n\
             [workspace.dependencies]\n\
             kuna-core = { path = \"crates/kuna-core\" }\n\
             kuna-console = { path = \"crates/kuna-console\" }\n",
        )
        .unwrap();
        fs::write(root.join("Cargo.lock"), "version = 4\n").unwrap();
        fs::write(
            root.join("crates/kuna-console/Cargo.toml"),
            "[package]\nname = \"kuna-console\"\n[dependencies]\nkuna-core.workspace = true\n",
        )
        .unwrap();
        fs::write(root.join("crates/kuna-console/src/lib.rs"), "console\n").unwrap();
        fs::write(
            root.join("crates/kuna-core/Cargo.toml"),
            "[package]\nname = \"kuna-core\"\n",
        )
        .unwrap();
        fs::write(root.join("crates/kuna-core/src/lib.rs"), "core\n").unwrap();
        fs::write(
            root.join("crates/kuna-harness/Cargo.toml"),
            "[package]\nname = \"kuna-harness\"\n[dependencies]\nkuna-console.workspace = true\n",
        )
        .unwrap();
        fs::write(root.join("crates/kuna-harness/src/lib.rs"), "harness\n").unwrap();
        root
    }

    #[test]
    fn component_graphs_cover_their_sources_and_workspace_inputs() {
        let root = workspace("coverage");
        let engine_fingerprint = calculate(&root, "kuna-console").unwrap();
        assert!(engine_fingerprint
            .watched
            .contains(&root.join("Cargo.toml")));
        assert!(engine_fingerprint
            .watched
            .contains(&root.join("Cargo.lock")));
        let engine = engine_fingerprint.value;
        let harness = calculate(&root, "kuna-harness").unwrap().value;

        fs::write(
            root.join("crates/kuna-harness/src/lib.rs"),
            "changed harness\n",
        )
        .unwrap();
        assert_eq!(calculate(&root, "kuna-console").unwrap().value, engine);
        assert_ne!(calculate(&root, "kuna-harness").unwrap().value, harness);

        let harness = calculate(&root, "kuna-harness").unwrap().value;
        fs::write(root.join("crates/kuna-core/src/lib.rs"), "changed core\n").unwrap();
        assert_ne!(calculate(&root, "kuna-console").unwrap().value, engine);
        assert_ne!(calculate(&root, "kuna-harness").unwrap().value, harness);

        let engine = calculate(&root, "kuna-console").unwrap().value;
        let harness = calculate(&root, "kuna-harness").unwrap().value;
        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
        )
        .unwrap();
        assert_ne!(calculate(&root, "kuna-console").unwrap().value, engine);
        assert_ne!(calculate(&root, "kuna-harness").unwrap().value, harness);

        let engine = calculate(&root, "kuna-console").unwrap().value;
        let harness = calculate(&root, "kuna-harness").unwrap().value;
        fs::write(root.join("Cargo.lock"), "version = 4\n# changed\n").unwrap();
        assert_ne!(calculate(&root, "kuna-console").unwrap().value, engine);
        assert_ne!(calculate(&root, "kuna-harness").unwrap().value, harness);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn missing_required_inputs_fail_closed() {
        let root = workspace("missing");
        fs::remove_file(root.join("Cargo.lock")).unwrap();
        let err = calculate(&root, "kuna-console").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn valid_cargo_dependency_forms_do_not_hide_workspace_crates() {
        let forms = [
            (
                "quoted",
                "[dependencies]\n\"kuna-core\" = { path = \"../kuna-core\" }\n",
                None,
            ),
            (
                "renamed",
                "[dependencies]\nengine = { package = \"kuna-core\", path = \"../kuna-core\" }\n",
                None,
            ),
            (
                "table",
                "[dependencies.engine]\npackage = \"kuna-core\"\npath = \"../kuna-core\"\n",
                None,
            ),
            (
                "quoted_table",
                "[dependencies.\"kuna-core\"]\npath = \"../kuna-core\"\n",
                None,
            ),
            (
                "workspace_alias",
                "[dependencies]\nengine.workspace = true\n",
                Some(
                    "[workspace]\nmembers = [\"crates/*\"]\n\
                     [workspace.dependencies]\n\
                     engine = { package = \"kuna-core\", path = \"crates/kuna-core\" }\n",
                ),
            ),
        ];
        for (tag, console_dependencies, workspace_manifest) in forms {
            let root = workspace(tag);
            if let Some(manifest) = workspace_manifest {
                fs::write(root.join("Cargo.toml"), manifest).unwrap();
            }
            fs::write(
                root.join("crates/kuna-console/Cargo.toml"),
                format!("[package]\nname = \"kuna-console\"\n{console_dependencies}"),
            )
            .unwrap();
            let before = calculate(&root, "kuna-console").unwrap().value;
            fs::write(root.join("crates/kuna-core/src/lib.rs"), "changed core\n").unwrap();
            assert_ne!(
                calculate(&root, "kuna-console").unwrap().value,
                before,
                "dependency form {tag} omitted kuna-core",
            );
            let _ = fs::remove_dir_all(root);
        }
    }

    #[test]
    fn kuna_looking_aliases_for_external_packages_stay_external() {
        let root = workspace("external_alias");
        fs::write(
            root.join("crates/kuna-console/Cargo.toml"),
            "[package]\nname = \"kuna-console\"\n\
             [dependencies.kuna-shim]\n\
             package = \"serde\"\n\
             version = \"1\"\n",
        )
        .unwrap();
        calculate(&root, "kuna-console").unwrap();

        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/*\"]\n\
             [workspace.dependencies.kuna-shim]\n\
             package = \"serde\"\n\
             version = \"1\"\n",
        )
        .unwrap();
        fs::write(
            root.join("crates/kuna-console/Cargo.toml"),
            "[package]\nname = \"kuna-console\"\n\
             [dependencies]\n\
             kuna-shim.workspace = true\n",
        )
        .unwrap();
        calculate(&root, "kuna-console").unwrap();
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn crlf_and_lf_sources_have_the_same_fingerprint() {
        let root = workspace("newlines");
        let path = root.join("crates/kuna-core/src/lib.rs");
        fs::write(&path, "one\ntwo\n").unwrap();
        let lf = calculate(&root, "kuna-console").unwrap().value;
        fs::write(&path, "one\r\ntwo\r\n").unwrap();
        assert_eq!(calculate(&root, "kuna-console").unwrap().value, lf);
        let _ = fs::remove_dir_all(root);
    }
}
