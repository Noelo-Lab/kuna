use std::fs;
use std::io::Read;

use kuna_analysis::analyzers::bsim::corpus::CorpusExecutable;
use kuna_analysis::analyzers::bsim::lshweights::LshProfile;
use kuna_analysis::analyzers::bsim::signature::BsimSignature;
use kuna_bsim_server::{ImportOutcome, QueryFilters, QueryRequest, Store, StoreError};
use md5::Md5;
use sha2::{Digest, Sha256};

const PROFILE: LshProfile = LshProfile::Size64;
const SETTINGS: u32 = 0x49;

fn executable(bytes: &[u8], complete: bool, addresses: &[u64]) -> CorpusExecutable {
    CorpusExecutable {
        name: "shared-name".into(),
        architecture: "x86:LE:64:default".into(),
        compiler: "gcc".into(),
        md5: kuna_base::hex_lower(&Md5::digest(bytes)),
        complete,
        functions: addresses.iter().map(|address| BsimSignature {
            name: format!("function_{address:x}"),
            address: *address,
            settings: SETTINGS,
            features: vec![3, 4, 4, 9],
            has_unimplemented: false,
            has_bad_data: false,
        }).collect(),
    }
}

fn query() -> QueryRequest {
    QueryRequest {
        profile: PROFILE.name().into(),
        settings: SETTINGS,
        features: vec![3, 4, 4, 9],
        min_similarity: 0.9,
        max_matches: 10,
        filters: QueryFilters::default(),
    }
}

#[test]
fn same_filename_has_distinct_immutable_objects_and_deterministic_rank() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let a = store.import(PROFILE, executable(b"first build", true, &[23]), b"first build").unwrap();
    let b = store.import(PROFILE, executable(b"second build", false, &[42]), b"second build").unwrap();
    assert_ne!(a.sha256, b.sha256);
    assert_eq!(a.outcome, ImportOutcome::Inserted);
    assert_eq!(b.outcome, ImportOutcome::Inserted);

    let mut req = query();
    req.max_matches = 1;
    let result = store.query(&req).unwrap();
    assert_eq!(result.considered, 2);
    assert_eq!(result.incomplete_executables, 1);
    assert!(result.truncated);
    assert_eq!(result.matches[0].function.sha256, a.sha256);
    assert_eq!(result.matches[0].function.address, 23);
    assert!(result.matches[0].similarity > 0.99);

    req.filters.sha256 = Some(b.sha256.clone());
    req.filters.executable = Some("shared-name".into());
    req.filters.architecture = Some("x86:LE:64:default".into());
    req.filters.compiler = Some("gcc".into());
    let result = store.query(&req).unwrap();
    assert!(!result.truncated);
    assert_eq!(result.matches[0].function.sha256, b.sha256);
    assert_eq!(result.incomplete_executables, 1);
    assert!(!result.matches[0].function.complete);
    req.filters.compiler = Some("different".into());
    let miss = store.query(&req).unwrap();
    assert_eq!(miss.considered, 0);
    assert_eq!(miss.incomplete_executables, 0);
    assert!(miss.matches.is_empty());

    let entry = store.function(PROFILE.name(), &b.sha256, 42).unwrap();
    assert_eq!(entry.name, "function_2a");
    assert!(matches!(store.function(PROFILE.name(), &b.sha256, 23), Err(StoreError::NotFound(_))));
    let (mut bin, length) = store.open_binary(&b.sha256).unwrap();
    let mut bytes = Vec::new();
    bin.read_to_end(&mut bytes).unwrap();
    assert_eq!(bytes, b"second build");
    assert_eq!(length, bytes.len() as u64);
    assert_eq!(b.sha256, kuna_base::hex_lower(&Sha256::digest(&bytes)));
}

#[test]
fn cross_process_store_handles_merge_retries_and_publish_only_after_blob() {
    let dir = tempfile::tempdir().unwrap();
    let reader = Store::open(dir.path()).unwrap();
    let writer = Store::open(dir.path()).unwrap();
    let original = b"import source";
    let hash = kuna_base::hex_lower(&Sha256::digest(original));
    let path = dir.path().join("binaries").join(&hash);
    fs::write(&path, original).unwrap();
    assert!(matches!(reader.open_binary(&hash), Err(StoreError::NotFound(_))));

    let mut bad = executable(original, false, &[]);
    bad.md5.clear();
    assert!(matches!(writer.import(PROFILE, bad, original), Err(StoreError::Invalid(_))));
    assert!(matches!(reader.open_binary(&hash), Err(StoreError::NotFound(_))));

    let initial = writer.import(PROFILE, executable(original, false, &[]), original).unwrap();
    let incomplete = reader.query(&query()).unwrap();
    assert_eq!(incomplete.considered, 0);
    assert_eq!(incomplete.incomplete_executables, 1);
    let added = writer.import(PROFILE, executable(original, false, &[7]), original).unwrap();
    assert_eq!(added.outcome, ImportOutcome::Updated);
    let complete = writer.import(PROFILE, executable(original, true, &[9, 7]), original).unwrap();
    assert_eq!(complete.outcome, ImportOutcome::Updated);
    assert_eq!(complete.sha256, initial.sha256);
    let mut alternate = executable(original, true, &[9, 7]);
    alternate.name = "different display name".into();
    let duplicate = writer.import(PROFILE, alternate, original).unwrap();
    assert_eq!(duplicate.outcome, ImportOutcome::AlreadyPresent);
    let profile = reader.profiles().unwrap().profiles.into_iter().find(|x| x.profile == PROFILE.name()).unwrap();
    assert_eq!((profile.executables, profile.functions, profile.incomplete_executables), (1, 2, 0));
    assert_eq!(reader.function(PROFILE.name(), &hash, 7).unwrap().executable, "shared-name");
    assert_eq!(fs::read_dir(dir.path().join("binaries")).unwrap().count(), 1);

    fs::write(&path, b"wrong bytes!!").unwrap();
    assert!(matches!(writer.import(PROFILE, executable(original, true, &[7, 9]), original), Err(StoreError::Corrupt(_))));
    fs::remove_file(path).unwrap();
    assert!(matches!(reader.profiles(), Err(StoreError::Corrupt(_))));
    assert!(matches!(reader.query(&query()), Err(StoreError::Corrupt(_))));
    assert!(matches!(reader.open_binary(&hash), Err(StoreError::Corrupt(_))));
}

#[test]
fn publish_updates_one_signature_without_changing_a_completed_record() {
    let dir = tempfile::tempdir().unwrap();
    let writer = Store::open(dir.path()).unwrap();
    let reader = Store::open(dir.path()).unwrap();
    let original = b"completed executable";
    let imported = writer.import(PROFILE, executable(original, true, &[7, 9]), original).unwrap();
    let lookup = |address| reader.function(PROFILE.name(), &imported.sha256, address).unwrap();
    let sibling = lookup(9);

    let mut selective = executable(original, false, &[7]);
    selective.name = "uploaded-name".into();
    selective.functions[0].name = "parse_header".into();
    let renamed = writer.publish(PROFILE, selective.clone(), original).unwrap();
    assert_eq!(renamed.outcome, ImportOutcome::Updated);
    assert_eq!(renamed.sha256, imported.sha256);
    assert_eq!(lookup(7).name, "parse_header");
    assert_eq!(lookup(7).executable, "shared-name");
    assert!(lookup(7).complete);
    assert_eq!(lookup(9), sibling);

    selective.functions[0].features = vec![33, 35, 35, 37];
    let changed = writer.publish(PROFILE, selective.clone(), original).unwrap();
    assert_eq!(changed.outcome, ImportOutcome::Updated);
    let old_matches = reader.query(&query()).unwrap().matches;
    assert_eq!(old_matches.len(), 1);
    assert_eq!(old_matches[0].function, sibling);
    let mut request = query();
    request.features.clone_from(&selective.functions[0].features);
    let new_matches = reader.query(&request).unwrap().matches;
    assert_eq!(new_matches.len(), 1);
    assert_eq!(new_matches[0].function, lookup(7));

    let repeated = writer.publish(PROFILE, selective, original).unwrap();
    assert_eq!(repeated.outcome, ImportOutcome::AlreadyPresent);
    assert_eq!(serde_json::to_string(&repeated.outcome).unwrap(), "\"already_present\"");
    let added = writer.publish(PROFILE, executable(original, false, &[8]), original).unwrap();
    assert_eq!(added.outcome, ImportOutcome::Updated);
    assert!(lookup(8).complete);
    assert_eq!(lookup(9), sibling);
    assert_eq!(fs::read_dir(dir.path().join("binaries")).unwrap().count(), 1);
}

#[test]
fn full_import_fills_in_a_selectively_published_executable_without_overwriting_it() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let original = b"native executable";
    let mut selective = executable(original, false, &[7]);
    selective.functions[0].name = "curated_entry".into();
    let published = store.publish(PROFILE, selective, original).unwrap();
    assert_eq!(published.outcome, ImportOutcome::Inserted);
    let lookup = |address| store.function(PROFILE.name(), &published.sha256, address);
    assert!(!lookup(7).unwrap().complete);
    assert!(matches!(lookup(9), Err(StoreError::NotFound(_))));

    let mut full = executable(original, true, &[9, 7]);
    full.functions[1].features = vec![33, 35, 35, 37];
    let imported = store.import(PROFILE, full, original).unwrap();
    assert_eq!(imported.outcome, ImportOutcome::Updated);
    assert_eq!(imported.sha256, published.sha256);
    assert_eq!(lookup(7).unwrap().name, "curated_entry");
    assert_eq!(lookup(9).unwrap().name, "function_9");
    let result = store.query(&query()).unwrap();
    assert_eq!(result.considered, 2);
    assert_eq!(result.incomplete_executables, 0);
    assert_eq!(result.matches.len(), 2);
    assert!(result.matches.iter().all(|matched| matched.function.complete));
}

#[test]
fn native_and_project_imports_merge_in_either_order() {
    for project_first in [false, true] {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        let original = b"merged executable";
        let native = executable(original, true, &[7, 9]);
        // Even a fully signed project snapshot is not native discovery coverage.
        let mut project = executable(original, true, &[8, 7]);
        project.name = "saved-program".into();
        project.functions[1].name = "saved_entry".into();
        project.functions[1].features = vec![33, 35, 35, 37];

        let digest = if project_first {
            let (saved, merge) = store.import_project(PROFILE, project, original).unwrap();
            assert_eq!(saved.outcome, ImportOutcome::Inserted);
            assert_eq!((merge.inserted, merge.updated), (2, 0));
            assert!(!merge.executable_complete);
            assert!(!store.function(PROFILE.name(), &saved.sha256, 7).unwrap().complete);
            let discovered = store.import(PROFILE, native, original).unwrap();
            assert_eq!(discovered.outcome, ImportOutcome::Updated);
            assert_eq!(discovered.sha256, saved.sha256);
            discovered.sha256
        } else {
            let discovered = store.import(PROFILE, native, original).unwrap();
            let (saved, merge) = store.import_project(PROFILE, project, original).unwrap();
            assert_eq!(saved.outcome, ImportOutcome::Updated);
            assert_eq!((merge.inserted, merge.updated), (1, 1));
            assert!(merge.executable_complete);
            assert_eq!(saved.sha256, discovered.sha256);
            saved.sha256
        };
        let lookup = |address| store.function(PROFILE.name(), &digest, address).unwrap();
        assert_eq!(lookup(8).name, "function_8");
        assert_eq!(lookup(9).name, "function_9");
        let saved = lookup(7);
        assert!(saved.complete);
        assert_eq!(saved.name, "saved_entry");
        assert_eq!(saved.executable, if project_first { "saved-program" } else { "shared-name" });
        let mut request = query();
        request.features = vec![33, 35, 35, 37];
        let result = store.query(&request).unwrap();
        assert_eq!(result.considered, 3);
        assert_eq!(result.incomplete_executables, 0);
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].function, saved);
    }
}

#[test]
fn repeated_project_snapshots_keep_disjoint_entries_and_update_overlaps() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let original = b"project snapshots";
    let first = executable(original, false, &[4, 11]);
    let second = executable(original, false, &[3, 7, 13]);
    let (initial, _) = store.import_project(PROFILE, first.clone(), original).unwrap();
    let (disjoint, merge) = store.import_project(PROFILE, second.clone(), original).unwrap();
    assert_eq!(disjoint.outcome, ImportOutcome::Updated);
    assert_eq!((merge.inserted, merge.updated), (3, 0));
    assert!(!merge.executable_complete);

    for (snapshot, count) in [(first, 2), (second, 3)] {
        let (same, merge) = store.import_project(PROFILE, snapshot, original).unwrap();
        assert_eq!(same.outcome, ImportOutcome::AlreadyPresent);
        assert_eq!((merge.inserted, merge.updated, merge.unchanged), (0, 0, count));
    }

    let mut latest = executable(original, false, &[4, 6]);
    latest.functions[0].name = "project_rename".into();
    latest.functions[0].features = vec![33, 35, 35, 37];
    let (renamed, merge) = store.import_project(PROFILE, latest, original).unwrap();
    assert_eq!(renamed.outcome, ImportOutcome::Updated);
    assert_eq!(renamed.sha256, initial.sha256);
    assert_eq!((merge.inserted, merge.updated), (1, 1));
    assert!(!merge.executable_complete);
    assert_eq!(store.function(PROFILE.name(), &initial.sha256, 4).unwrap().name, "project_rename");
    let result = store.query(&query()).unwrap();
    assert_eq!(result.incomplete_executables, 1);
    for address in [3, 6, 7, 11, 13] {
        assert!(result.matches.iter().any(|m| m.function.address == address));
    }
    let mut request = query();
    request.features = vec![33, 35, 35, 37];
    let latest_matches = store.query(&request).unwrap().matches;
    assert_eq!(latest_matches.len(), 1);
    assert_eq!(latest_matches[0].function.address, 4);
}

#[test]
fn publishing_an_identical_project_signature_protects_it_against_later_snapshots() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let original = b"published project executable";
    let mut project = executable(original, false, &[7, 9]);
    project.functions[0].name = "saved_and_published".into();
    let (saved, _) = store.import_project(PROFILE, project.clone(), original).unwrap();
    let mut publication = project.clone();
    publication.functions.truncate(1);
    let published = store.publish(PROFILE, publication, original).unwrap();
    assert_eq!(published.outcome, ImportOutcome::Updated);
    assert_eq!(published.sha256, saved.sha256);

    project.functions[0].name = "conflicting_project_name".into();
    project.functions[0].features = vec![33, 35, 35, 37];
    let (retry, merge) = store.import_project(PROFILE, project, original).unwrap();
    assert_eq!(retry.outcome, ImportOutcome::AlreadyPresent);
    assert_eq!((merge.inserted, merge.updated, merge.unchanged, merge.kept_published), (0, 0, 1, 1));
    let matches = store.query(&query()).unwrap().matches;
    assert_eq!(matches.len(), 2);
    assert!(matches.iter().any(|m| m.function.address == 7 && m.function.name == "saved_and_published"));
}

#[test]
fn publication_does_not_wait_for_the_local_import_lock() {
    use std::sync::mpsc;
    use std::time::Duration;

    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let lock = fs::File::options().read(true).write(true).create_new(true)
        .open(dir.path().join("import.lock")).unwrap();
    lock.lock().unwrap();
    let original = b"published after lock is released";
    let snapshot = executable(original, false, &[7]);
    let attempt = std::thread::scope(|scope| {
        let (send, receive) = mpsc::channel();
        let store = &store;
        let snapshot = snapshot.clone();
        let publish = scope.spawn(move || {
            let _ = send.send(store.publish(PROFILE, snapshot, original));
        });
        let attempt = receive.recv_timeout(Duration::from_secs(5));
        drop(lock);
        publish.join().unwrap();
        attempt
    });
    assert!(matches!(attempt, Ok(Err(StoreError::Deadline))));
    assert_eq!(store.publish(PROFILE, snapshot, original).unwrap().outcome, ImportOutcome::Inserted);
}

#[cfg(unix)]
#[test]
fn unchanged_import_does_not_hide_an_unreadable_profile_directory() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let original = b"directory sync retry";
    let snapshot = executable(original, false, &[7]);
    store.import_project(PROFILE, snapshot.clone(), original).unwrap();

    // The record is already visible, as after a rename with a failed fsync.
    // Search permission allows reopening it, but not opening its directory.
    let profile = dir.path().join("records").join(PROFILE.name());
    let original_permissions = fs::metadata(&profile).unwrap().permissions();
    fs::set_permissions(&profile, fs::Permissions::from_mode(0o300)).unwrap();
    let cannot_open_directory = fs::File::open(&profile).is_err();
    let retry = store.import_project(PROFILE, snapshot.clone(), original);
    fs::set_permissions(&profile, original_permissions).unwrap();
    if cannot_open_directory {
        assert!(matches!(retry, Err(StoreError::Io(e)) if e.kind() == std::io::ErrorKind::PermissionDenied));
    }
    let (synced, _) = store.import_project(PROFILE, snapshot, original).unwrap();
    assert_eq!(synced.outcome, ImportOutcome::AlreadyPresent);
}

#[test]
fn publish_validates_selective_signature_and_artifact_before_visibility() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let original = b"native executable";
    let valid = executable(original, false, &[7]);

    let mut incompatible = valid.clone();
    incompatible.architecture = "x86:LE:32:default".into();
    let mut wrong_settings = valid.clone();
    wrong_settings.functions[0].settings = 0;
    let mut unordered = valid.clone();
    unordered.functions[0].features = vec![4, 3];
    for bad in [
        executable(original, false, &[]),
        executable(original, false, &[7, 9]),
        executable(original, true, &[7]),
        executable(b"different binary", false, &[7]),
        incompatible,
        wrong_settings,
        unordered,
    ] {
        assert!(matches!(store.publish(PROFILE, bad, original), Err(StoreError::Invalid(_))));
    }
    assert!(matches!(
        store.publish(PROFILE, executable(b"", false, &[7]), b""),
        Err(StoreError::Invalid(_))
    ));
    let sha256 = kuna_base::hex_lower(&Sha256::digest(original));
    assert!(matches!(store.open_binary(&sha256), Err(StoreError::NotFound(_))));
    assert_eq!(fs::read_dir(dir.path().join("binaries")).unwrap().count(), 0);
}

#[cfg(unix)]
#[test]
fn internal_symlinks_are_not_artifacts() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    symlink(outside.path(), dir.path().join("binaries")).unwrap();
    assert!(matches!(Store::open(dir.path()), Err(StoreError::Corrupt(_))));
    fs::remove_file(dir.path().join("binaries")).unwrap();
    let store = Store::open(dir.path()).unwrap();
    let original = b"target";
    let hash = kuna_base::hex_lower(&Sha256::digest(original));
    let missing = outside.path().join("missing");
    symlink(&missing, dir.path().join("binaries").join(hash)).unwrap();
    assert!(matches!(store.import(PROFILE, executable(original, true, &[17]), original), Err(StoreError::Corrupt(_))));
    assert!(!missing.exists());
}
