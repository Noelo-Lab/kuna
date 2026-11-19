//! `kuna unpack <cache> --list | --image NAME`: the dyld shared cache arm.
//!
//! A cache is not packed, but it is the same kind of first step: no other
//! surface can read a system library until it is taken out of the cache.
//! `--list` prints the images; `--image NAME` (the global flag, read through
//! the loader's environment variable) writes the chosen image as the thin
//! Mach-O every other surface rebuilds for itself.

use std::fmt::Write as _;
use std::path::Path;

use kuna_analysis::loader::kuna_dyldcache::{read_selected_image, DyldCache};

use crate::jsonfmt::{dumps_indent2, Json};

fn num(v: u64) -> Json {
    Json::Number(v.to_string())
}

pub(super) fn run(binary: &str, output: Option<&str>, list: bool, json: bool) -> i32 {
    let path = Path::new(binary);
    if list {
        return match DyldCache::open(path) {
            Ok(cache) => crate::output::emit_with_status(&render_list(binary, &cache, json), 0),
            Err(e) => {
                eprintln!("error: {binary}: {e}");
                1
            }
        };
    }
    let image = match read_selected_image(path) {
        Some(Ok(image)) => image,
        Some(Err(e)) => {
            eprintln!("error: {binary}: {e}");
            return if e.contains("choose one with --image") || e.contains("image name") { 2 } else { 1 };
        }
        None => {
            eprintln!("error: {binary}: not a dyld shared cache");
            return 1;
        }
    };
    let mut name = std::env::var(kuna_decomp::kuna_dyldcache::DYLDIMAGE_ENV).unwrap_or_default().trim().to_string();
    if name.is_empty() {
        if let Ok(cache) = DyldCache::open(path) {
            if let [only] = cache.images() {
                name = only.path.clone();
            }
        }
    }
    let out_path = output.map(str::to_string).unwrap_or_else(|| format!("{binary}.{}.unpacked", output_stem(&name)));
    if let Err(e) = std::fs::write(&out_path, &image) {
        eprintln!("error: could not write {out_path}: {e}");
        return 1;
    }
    let text = if json {
        let v = Json::Object(vec![
            ("binary".into(), Json::Str(binary.into())),
            ("output".into(), Json::Str(out_path.clone())),
            ("packer".into(), Json::Str("dyld_shared_cache".into())),
            ("image".into(), Json::Str(name.clone())),
            ("unpacked_size".into(), num(image.len() as u64)),
        ]);
        format!("{}\n", dumps_indent2(&v))
    } else {
        format!("unpacked {binary} image {name} -> {out_path} ({} bytes)\n", image.len())
    };
    crate::output::emit_with_status(&text, 0)
}

/// The image's file name for the default output path, `image` when it has none.
fn output_stem(name: &str) -> &str {
    name.rsplit('/').next().filter(|s| !s.is_empty()).unwrap_or("image")
}

fn render_list(binary: &str, cache: &DyldCache, json: bool) -> String {
    if json {
        let images = cache
            .images()
            .iter()
            .enumerate()
            .map(|(i, image)| {
                Json::Object(vec![
                    ("index".into(), num(i as u64)),
                    ("address".into(), num(image.address)),
                    ("address_hex".into(), Json::Str(format!("{:#x}", image.address))),
                    ("path".into(), Json::Str(image.path.clone())),
                ])
            })
            .collect();
        let v = Json::Object(vec![
            ("binary".into(), Json::Str(binary.into())),
            ("arch".into(), Json::Str(cache.arch())),
            ("mappings".into(), num(cache.mappings().len() as u64)),
            ("images".into(), Json::Array(images)),
        ]);
        return format!("{}\n", dumps_indent2(&v));
    }
    let mut s = String::new();
    let _ = writeln!(
        s,
        "{binary}: dyld shared cache ({}), {} images, {} mappings",
        cache.arch(),
        cache.images().len(),
        cache.mappings().len()
    );
    for (i, image) in cache.images().iter().enumerate() {
        let _ = writeln!(s, "{i:5}  {:#x}  {}", image.address, image.path);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::output_stem;

    #[test]
    fn an_unnamed_image_still_gets_a_stem() {
        assert_eq!(output_stem("/usr/lib/libobjc.A.dylib"), "libobjc.A.dylib");
        assert_eq!(output_stem("libfoo"), "libfoo");
        assert_eq!(output_stem(""), "image");
        assert_eq!(output_stem("/usr/lib/"), "image");
    }
}
