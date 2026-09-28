//! The build-time schema for `phases.toml`; array order is catalog order.

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    pub group: Vec<Group>,
    pub subphase: Vec<Subphase>,
    pub surface: Vec<Surface>,
    pub settable: Vec<Settable>,
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

    const OTHER_TABLES: &str = "subphase = []\nsurface = []\nsettable = []\n";
    const GROUP: &str = "[[group]]\ngroup = 'first'\nphase = 'P0'\nsubphase = ''\nnote = ''\n";

    #[test]
    fn preserves_row_order_and_decodes_toml_strings() {
        let text = format!(
            "{OTHER_TABLES}{GROUP}[[group]]\ngroup = 'second'\nphase = 'P1'\nsubphase = ''\nnote = \"line\\n\\U0001F680\" # comment\n"
        );
        let registry: Registry = toml::from_str(&text).unwrap();
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
                toml::from_str::<Registry>(&format!("{OTHER_TABLES}{group}")).is_err(),
                "accepted invalid group: {group}"
            );
        }
    }

    #[test]
    fn requires_all_tables_and_rejects_unknown_tables() {
        assert!(toml::from_str::<Registry>(OTHER_TABLES).is_err());
        assert!(
            toml::from_str::<Registry>(&format!("{OTHER_TABLES}group = []\nunknown = []\n"))
                .is_err()
        );
    }
}
