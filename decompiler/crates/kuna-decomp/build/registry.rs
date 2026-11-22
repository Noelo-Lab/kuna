//! The build-time schema for `phases.toml` and `phases.d/`; row order is catalog order.

use serde::Deserialize;

pub struct Registry {
    pub group: Vec<Group>,
    pub subphase: Vec<Subphase>,
    pub surface: Vec<Surface>,
    pub settable: Vec<Settable>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Tables {
    group: Vec<Group>,
    subphase: Vec<Subphase>,
    surface: Vec<Surface>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionFile {
    settable: Vec<Settable>,
}

impl Registry {
    /// Parse `phases.toml` plus the `(file name, text)` option rows of `phases.d/`,
    /// which are taken in file-name order. Each option file is `NNNN-<option>.toml`
    /// holding exactly the one row whose `option` is `<option>`.
    pub fn parse(tables: &str, mut options: Vec<(String, String)>) -> Result<Registry, String> {
        let tables: Tables = toml::from_str(tables).map_err(|e| format!("phases.toml: {e}"))?;
        options.sort_by(|a, b| a.0.cmp(&b.0));
        let mut settable = Vec::with_capacity(options.len());
        for (name, text) in options {
            let file: OptionFile =
                toml::from_str(&text).map_err(|e| format!("phases.d/{name}: {e}"))?;
            let [row]: [Settable; 1] = file.settable.try_into().map_err(|rows: Vec<_>| {
                format!("phases.d/{name}: holds {} rows, need exactly one", rows.len())
            })?;
            let stem = name
                .strip_suffix(".toml")
                .and_then(|s| s.split_once('-'))
                .filter(|(n, _)| n.len() == 4 && n.bytes().all(|b| b.is_ascii_digit()))
                .map(|(_, opt)| opt);
            if stem != Some(row.option.as_str()) {
                return Err(format!(
                    "phases.d/{name}: file name must be NNNN-{}.toml",
                    row.option
                ));
            }
            settable.push(row);
        }
        Ok(Registry {
            group: tables.group,
            subphase: tables.subphase,
            surface: tables.surface,
            settable,
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    pub group: String,
    pub phase: String,
    pub subphase: String,
    pub note: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subphase {
    pub name: String,
    pub phase: String,
    pub decision: String,
    pub assertion: String,
    pub strength: String,
    pub rewind: String,
    pub latent: bool,
    pub exposure: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Surface {
    pub surface: String,
    pub phase: String,
    pub subphase: String,
    pub note: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settable {
    pub option: String,
    pub values: String,
    pub default: String,
    pub destructive: bool,
    pub phase: String,
    pub subphase: String,
    pub strength: String,
    pub rewind: String,
    pub issue: String,
    pub summary: String,
    pub use_when: String,
    pub example: String,
    pub source_decompiler: String,
    pub inspiration: String,
    pub change_kind: String,
    pub tier: String,
    pub symptoms: String,
    pub live_field: Option<String>,
    pub live_true: Option<String>,
    pub live_false: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Registry;

    const OTHER_TABLES: &str = "subphase = []\nsurface = []\n";
    const GROUP: &str = "[[group]]\ngroup = 'first'\nphase = 'P0'\nsubphase = ''\nnote = ''\n";

    fn row(option: &str) -> String {
        format!(
            "# a comment\n[[settable]]\noption = '{option}'\nvalues = 'on|off'\ndefault = 'on'\n\
             destructive = false\nphase = 'P1'\nsubphase = ''\nstrength = 'HARD'\nrewind = 'P1'\n\
             issue = ''\nsummary = ''\nuse_when = ''\nexample = ''\nsource_decompiler = ''\n\
             inspiration = ''\nchange_kind = ''\ntier = 'core'\nsymptoms = ''\n"
        )
    }

    fn parse(tables: &str, options: &[(&str, String)]) -> Result<Registry, String> {
        let options = options.iter().map(|(n, t)| (n.to_string(), t.clone())).collect();
        Registry::parse(tables, options)
    }

    #[test]
    fn preserves_row_order_and_decodes_toml_strings() {
        let text = format!(
            "{OTHER_TABLES}{GROUP}[[group]]\ngroup = 'second'\nphase = 'P1'\nsubphase = ''\nnote = \"line\\n\\U0001F680\" # comment\n"
        );
        let registry = parse(&text, &[]).unwrap();
        assert_eq!(registry.group[0].group, "first");
        assert_eq!(registry.group[1].group, "second");
        assert_eq!(registry.group[1].note, "line\n🚀");
    }

    #[test]
    fn rejects_duplicate_missing_unknown_and_wrongly_typed_fields() {
        for group in [
            format!("{GROUP}phase = 'P1'\n"),
            GROUP.replace("note = ''\n", ""),
            format!("{GROUP}notes = 'typo'\n"),
            GROUP.replace("note = ''", "note = false"),
            GROUP.replace("note = ''", "note = \"\\uD800\""),
        ] {
            assert!(
                parse(&format!("{OTHER_TABLES}{group}"), &[]).is_err(),
                "accepted invalid group: {group}"
            );
        }
    }

    #[test]
    fn requires_all_tables_and_rejects_unknown_tables() {
        assert!(parse(OTHER_TABLES, &[]).is_err());
        assert!(parse(&format!("{OTHER_TABLES}group = []\nunknown = []\n"), &[]).is_err());
        let stray = format!("{OTHER_TABLES}group = []\n{}", row("a"));
        assert!(parse(&stray, &[]).is_err(), "an option row belongs in phases.d/");
    }

    #[test]
    fn option_rows_follow_file_name_order() {
        let tables = format!("{OTHER_TABLES}group = []\n");
        let registry = parse(
            &tables,
            &[("0002-b.toml", row("b")), ("0001-z.toml", row("z")), ("0002-a.toml", row("a"))],
        )
        .unwrap();
        let order: Vec<_> = registry.settable.iter().map(|r| r.option.as_str()).collect();
        assert_eq!(order, ["z", "a", "b"]);
    }

    #[test]
    fn an_option_file_is_one_row_named_after_its_option() {
        let tables = format!("{OTHER_TABLES}group = []\n");
        for (name, text) in [
            ("0001-a.toml", format!("{}{}", row("a"), row("a"))),
            ("0001-a.toml", String::new()),
            ("0001-a.toml", row("b")),
            ("a.toml", row("a")),
            ("01-a.toml", row("a")),
            ("0001-a.txt", row("a")),
            ("0001-a.toml", format!("{}[[group]]\n", row("a"))),
        ] {
            assert!(parse(&tables, &[(name, text.clone())]).is_err(), "accepted {name}: {text}");
        }
    }
}
