//! End-to-end gate for `funcchunks`: the function chunk model over a real
//! x86-64 ELF where two functions share one tail placed after a third.

use std::path::PathBuf;

use kuna_analysis::listing::kuna_funcchunks::ChunkModel;
use kuna_analysis::listing::Listing;
use kuna_console::engine::bootstrap_from_object;

const F: u64 = 0x401019;
const G: u64 = 0x401024;
const H: u64 = 0x401030;
const SHARED: u64 = 0x401036;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn listing_of(bin: &str, extra_seeds: &[u64]) -> Listing {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let prog = bootstrap_from_object(bin, "", &spec_roots).expect("bootstrap with built specs");
    let bytes = std::fs::read(bin).expect("read binary");
    let file = object::File::parse(&*bytes).expect("parse object");
    let mut seeds = kuna_analysis::entry::collect_entries(&file, &bytes);
    seeds.extend_from_slice(extra_seeds);
    seeds.push(object::Object::entry(&file));
    seeds.sort_unstable();
    seeds.dedup();
    let image = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(bin, &bytes)
        .expect("open loadimage");
    let arch = prog.arch();
    Listing::build(&file, &image, arch, arch.translate(), &seeds)
}

#[test]
fn shared_tail_belongs_to_both_jumpers_and_not_to_the_clip_neighbour() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/funcchunks_x86_64");
    let listing = listing_of(bin.to_str().unwrap(), &[]);
    let model = ChunkModel::build(&listing);

    assert_eq!(model.chunks(F).unwrap(), &[(F, F + 11), (SHARED, SHARED + 6)]);
    assert_eq!(model.chunks(G).unwrap(), &[(G, G + 12), (SHARED, SHARED + 6)]);
    assert_eq!(model.chunks(H).unwrap(), &[(H, H + 6)]);
    assert_eq!(model.owners(SHARED), vec![F, G]);

    let clip_h: Vec<u64> = listing.instructions_in_range(H, None).map(|(&a, _)| a).collect();
    assert!(clip_h.contains(&SHARED), "the clip hands the shared tail to h");
}

/// The inventory `size` is the chunk body under `funcchunks`: `h` is its own two
/// instructions (the clip handed it the shared tail too), `f` and `g` count the
/// tail they jump into.
#[test]
fn inventory_size_is_the_chunk_body() {
    let spec_roots = vec![repo_root().join("specs").to_str().unwrap().to_string()];
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/funcchunks_x86_64");
    let sizes = |on: bool| {
        let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots).expect("bootstrap");
        prog.arch_mut().set_kuna_option("listing", "on").unwrap();
        if on {
            prog.arch_mut().set_kuna_option("funcchunks", "on").unwrap();
        }
        prog.commit_pending_analysis().expect("commit");
        prog.function_entries_canonical()
            .into_iter()
            .map(|e| (e.addr.get_offset(), e.size))
            .collect::<std::collections::BTreeMap<u64, u64>>()
    };
    let (clip, body) = (sizes(false), sizes(true));
    assert_eq!(clip[&H], 12, "the clip runs to the end of .text");
    assert_eq!(body[&H], 6);
    assert_eq!(body[&F], 11 + 6);
    assert_eq!(body[&G], 12 + 6);
}

/// Measurement: `KUNA_FUNCCHUNKS_BINS=a:b:c cargo test ... -- --ignored --nocapture`
/// prints functions, tails, shared instructions and how many function bodies
/// differ from the `[entry, next)` clip.
#[test]
#[ignore]
fn funcchunks_stats() {
    let Ok(list) = std::env::var("KUNA_FUNCCHUNKS_BINS") else { return };
    for bin in list.split(':').filter(|s| !s.is_empty()) {
        let bytes = std::fs::read(bin).expect("read binary");
        let file = object::File::parse(&*bytes).expect("parse object");
        let syms: Vec<u64> = object::Object::symbols(&file)
            .filter(|s| object::ObjectSymbol::kind(s) == object::SymbolKind::Text)
            .map(|s| object::ObjectSymbol::address(&s))
            .filter(|&a| a != 0)
            .collect();
        let listing = listing_of(bin, &syms);
        let t = std::time::Instant::now();
        let model = ChunkModel::build(&listing);
        let elapsed = t.elapsed();
        let mut with_tails = 0usize;
        let mut differs = 0usize;
        for (&e, _) in listing.functions() {
            let chunks = model.chunks(e).unwrap_or_default();
            if chunks.len() > 1 {
                with_tails += 1;
                if with_tails <= 6 {
                    println!("  {e:#x}: {chunks:x?}");
                }
            }
            let next = listing.next_function_after(e).map(|f| f.entry);
            let clip: Vec<u64> = listing.instructions_in_range(e, next).map(|(&a, _)| a).collect();
            let mut body: Vec<u64> = chunks
                .iter()
                .flat_map(|&(s, end)| listing.instructions_in_range(s, Some(end)).map(|(&a, _)| a))
                .collect();
            body.sort_unstable();
            if body != clip {
                differs += 1;
            }
        }
        println!(
            "{bin}: functions={} with_tails={with_tails} tails={} shared_insns={} body!=clip={differs} build={elapsed:?}",
            listing.function_count(),
            model.tail_count(),
            model.shared().len()
        );
    }
}
