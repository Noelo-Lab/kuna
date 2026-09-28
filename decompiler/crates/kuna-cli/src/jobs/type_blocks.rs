//! Merge worker type blocks in input order.
//!
//! Return a containing block verbatim, or warn and append unseen definitions.
//! Canonical printer text is borrowed; noncanonical line endings retain the
//! existing normalization used when comparing and appending definitions.

use std::borrow::Cow;

#[expect(
    clippy::disallowed_types,
    reason = "membership and subset checks only; output follows block and item order"
)]
type Definitions<'a> = std::collections::HashSet<&'a str>;

pub(crate) fn merge_type_definitions(blocks: &[String], tag: &str) -> String {
    let Some(first) = blocks.first() else {
        return String::new();
    };
    let parsed: Vec<_> = blocks.iter().map(|block| type_items(block)).collect();
    let sets: Vec<Definitions<'_>> = parsed
        .iter()
        .map(|items| items.iter().map(|(_, text)| text.as_ref()).collect())
        .collect();
    if let Some(k) = (0..blocks.len()).find(|&k| sets.iter().all(|other| other.is_subset(&sets[k]))) {
        return blocks[k].clone();
    }
    let mut seen = sets.into_iter().next().unwrap_or_default();
    eprintln!(
        "[kuna {tag}] warning: worker shards recovered different user-defined types, so the .h \
         type block is their union rather than the exact --jobs 1 rendering. Re-run with \
         --jobs 1 if the ordering matters."
    );
    let mut out = first.clone();
    for items in &parsed[1..] {
        for (spaced, item) in items {
            if seen.insert(item.as_ref()) {
                if *spaced && !out.is_empty() && !out.ends_with("\n\n") {
                    out.push('\n');
                }
                out.push_str(item);
            }
        }
    }
    out
}

fn type_items(block: &str) -> Vec<(bool, Cow<'_, str>)> {
    let mut items = Vec::new();
    let mut spaced = false;
    let mut lines = block.split_inclusive('\n');
    let mut offset = 0;
    while let Some(raw_line) = lines.next() {
        let start = offset;
        offset += raw_line.len();
        let line = line_text(raw_line);
        if line.trim().is_empty() {
            spaced = true;
            continue;
        }
        if line.ends_with('{') {
            for member in lines.by_ref() {
                offset += member.len();
                if line_text(member).starts_with('}') {
                    break;
                }
            }
        }
        let raw = &block[start..offset];
        let item = if raw.ends_with('\n') && !raw.contains("\r\n") {
            Cow::Borrowed(raw)
        } else {
            let mut normalized = String::new();
            for line in raw.lines() {
                normalized.push_str(line);
                normalized.push('\n');
            }
            Cow::Owned(normalized)
        };
        items.push((std::mem::take(&mut spaced), item));
    }
    items
}

fn line_text(raw: &str) -> &str {
    match raw.strip_suffix('\n') {
        Some(line) => line.strip_suffix('\r').unwrap_or(line),
        None => raw,
    }
}

#[cfg(test)]
#[path = "type_blocks/tests.rs"]
mod tests;
