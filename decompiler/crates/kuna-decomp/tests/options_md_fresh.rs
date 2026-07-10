//! Freshness fence for the generated option catalog: `docs/options.md` must be
//! byte-identical to what `kuna catalog --markdown` (which renders in-process
//! via `emit_catalog_markdown`) would emit. Regenerate after ANY settableTable
//! change:
//!
//! ```sh
//! decompiler/target/release/kuna catalog --markdown > docs/options.md
//! ```

use kuna_decomp::kuna_phases::emit_catalog_markdown;

const COMMITTED: &str = include_str!("../../../../docs/options.md");

#[test]
fn options_md_matches_the_generator_byte_for_byte() {
    let generated = emit_catalog_markdown();
    assert!(
        generated == COMMITTED,
        "docs/options.md is stale -- regenerate with \
         `decompiler/target/release/kuna catalog --markdown > docs/options.md` \
         (generated {} bytes, committed {} bytes)",
        generated.len(),
        COMMITTED.len()
    );
}
