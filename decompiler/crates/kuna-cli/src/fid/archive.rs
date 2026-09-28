//! Archive-member ingestion with owned staging files for the path-based loader.

use std::io::Write;

use super::{records_for_object_path, FidRecord};

pub(super) fn records_for_archive(
    archive_path: &str,
    bytes: &[u8],
    spec_roots: &[String],
) -> Result<Vec<FidRecord>, String> {
    use object::read::archive::ArchiveFile;

    let archive = ArchiveFile::parse(bytes).map_err(|e| format!("not a valid archive: {e}"))?;
    let mut out: Vec<FidRecord> = Vec::new();
    for member in archive.members() {
        let member = member.map_err(|e| format!("archive member error: {e}"))?;
        let data = member
            .data(bytes)
            .map_err(|e| format!("archive member data error: {e}"))?;
        if !is_object(data) {
            continue;
        }
        let name = String::from_utf8_lossy(member.name());
        let tmp = stage_object(data).map_err(|e| format!("cannot stage member {name}: {e}"))?;
        let tmp_str = tmp.to_string_lossy().into_owned();
        let res = records_for_object_path(&tmp_str, spec_roots);
        drop(tmp);
        match res {
            Ok(mut recs) => out.append(&mut recs),
            Err(e) => eprintln!("kuna fid build: {archive_path}({name}): skipped: {e}"),
        }
    }
    Ok(out)
}

fn stage_object(data: &[u8]) -> std::io::Result<tempfile::TempPath> {
    let mut file = tempfile::Builder::new()
        .prefix("kuna_fid_")
        .suffix(".o")
        .tempfile()?;
    file.write_all(data)?;
    Ok(file.into_temp_path())
}

fn is_object(bytes: &[u8]) -> bool {
    bytes.starts_with(b"\x7fELF")
        || bytes.starts_with(&[0xcf, 0xfa, 0xed, 0xfe])
        || bytes.starts_with(&[0xce, 0xfa, 0xed, 0xfe])
        || bytes.starts_with(b"MZ")
        || (bytes.len() >= 2 && bytes[0] == 0x4c && bytes[1] == 0x01)
        || (bytes.len() >= 2 && bytes[0] == 0x64 && bytes[1] == 0x86)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staged_objects_have_independent_lifetimes() {
        let first = stage_object(b"first member").unwrap();
        let second = stage_object(b"second member").unwrap();
        assert_ne!(first.as_os_str(), second.as_os_str());
        assert_eq!(first.extension().unwrap(), "o");
        assert_eq!(std::fs::read(&first).unwrap(), b"first member");
        assert_eq!(std::fs::read(&second).unwrap(), b"second member");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&first).unwrap().permissions().mode() & 0o077,
                0
            );
        }
        let first_path = first.to_path_buf();
        let second_path = second.to_path_buf();
        drop(first);
        assert!(!first_path.exists());
        assert_eq!(std::fs::read(&second).unwrap(), b"second member");
        drop(second);
        assert!(!second_path.exists());
    }

    #[test]
    fn staged_objects_are_removed_on_unwind() {
        let mut path = None;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let staged = stage_object(b"member").unwrap();
            path = Some(staged.to_path_buf());
            panic!("stop before loading");
        }));
        assert!(result.is_err());
        assert!(!path.unwrap().exists());
    }

    #[test]
    fn concurrent_staging_keeps_each_members_bytes() {
        let objects: Vec<_> = std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8u8)
                .map(|byte| scope.spawn(move || (byte, stage_object(&[byte; 32]).unwrap())))
                .collect();
            workers
                .into_iter()
                .map(|worker| worker.join().unwrap())
                .collect()
        });
        let paths: std::collections::BTreeSet<_> =
            objects.iter().map(|(_, path)| path.to_path_buf()).collect();
        assert_eq!(paths.len(), objects.len());
        for (byte, path) in &objects {
            assert_eq!(std::fs::read(path).unwrap(), [*byte; 32]);
        }
        drop(objects);
        assert!(paths.iter().all(|path| !path.exists()));
    }
}
