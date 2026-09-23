//! Unit tests for the ledger as data: the replay takes the ledger's decisions,
//! a table rebuilds the same structures in another factory, the wire keeps
//! every field, and a forced hook notices a function that asks differently.

use std::rc::Rc;

use super::*;
use crate::dtype::{type_metatype, TypeFactoryImpl};

/// A factory with the core types a real architecture mints.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    for (name, size, meta, chartp) in [
        ("undefined", 1, type_metatype::TYPE_UNKNOWN, false),
        ("undefined4", 4, type_metatype::TYPE_UNKNOWN, false),
        ("undefined8", 8, type_metatype::TYPE_UNKNOWN, false),
        ("char", 1, type_metatype::TYPE_INT, true),
        ("uint4", 4, type_metatype::TYPE_UINT, false),
        ("int8", 8, type_metatype::TYPE_INT, false),
    ] {
        f.set_core_type(name, size, meta, chartp).unwrap();
    }
    f.cache_core_types().unwrap();
    f
}

#[derive(Clone, Copy)]
enum Ty {
    CharPtr,
    Long,
    Uint,
    Bytes(int4),
}

fn ty(f: &TypeFactoryImpl, t: Ty) -> Rc<Datatype> {
    match t {
        Ty::CharPtr => {
            f.get_type_pointer(8, f.get_base(1, type_metatype::TYPE_INT).unwrap(), 1).unwrap()
        }
        Ty::Long => f.get_base(8, type_metatype::TYPE_INT).unwrap(),
        Ty::Uint => f.get_base(4, type_metatype::TYPE_UINT).unwrap(),
        Ty::Bytes(n) => {
            f.get_type_array(n, f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap()).unwrap()
        }
    }
}

/// The member list `fields_for` would build: every member named for its offset.
fn fields(f: &TypeFactoryImpl, spec: &[(int4, Ty)]) -> Vec<TypeField> {
    spec.iter()
        .enumerate()
        .map(|(i, &(off, t))| TypeField::new(i as int4, off, format!("field_0x{off:x}"), ty(f, t)))
        .collect()
}

/// A request as a worker records it, every core type held at load.
fn req(fs: &[TypeField], size: int4, unclaimed: &[(int4, int4)]) -> SynthRequest {
    SynthRequest::of(fs, size, unclaimed, &[], &AtLoad::of(&factory()))
}

fn at_load() -> Rc<AtLoad> {
    Rc::new(AtLoad::of(&factory()))
}

type Step = (Vec<(int4, Ty)>, int4, Vec<(int4, int4)>);

/// Six lookups that take every branch of the decision: a mint, an exact-shape
/// reuse, a larger layout that supersedes the first, a reader the survivor
/// answers, an unrelated shape, and a reader whose unclaimed bytes veto the
/// survivor and fall back to the superseded shape.
fn script() -> Vec<Step> {
    let pair = vec![(0, Ty::CharPtr), (8, Ty::Long)];
    let four = vec![(0, Ty::CharPtr), (8, Ty::Long), (0x10, Ty::Uint), (0x14, Ty::Uint)];
    let words = vec![(0, Ty::Long), (8, Ty::Long), (0x10, Ty::Long)];
    vec![
        (pair.clone(), 16, vec![]),
        (pair.clone(), 16, vec![]),
        (four, 24, vec![]),
        (pair.clone(), 16, vec![]),
        (words, 24, vec![]),
        (pair, 16, vec![(0x10, 4)]),
    ]
}

#[test]
fn a_replay_names_every_structure_as_the_ledger_does() {
    let live = factory();
    let mut replay = Replay::probe(&factory());
    let mut named = Vec::new();
    for (spec, size, unclaimed) in script() {
        let fs = fields(&live, &spec);
        let request = req(&fs, size, &unclaimed);
        let real =
            ledger::lookup_or_mint(&live, fs, size, &unclaimed, &[], crate::kuna_structmerge::StructMergeMode::Off).map(|t| t.get_name().to_string());
        assert_eq!(replay.lookup_or_mint(&request), real);
        named.push(real.unwrap());
    }
    assert_eq!(named, ["struct_0", "struct_0", "struct_1", "struct_1", "struct_2", "struct_0"]);
    assert_eq!(replay.superseded_names(), ledger::superseded_names(&live));
    assert_eq!(replay.superseded_names(), ["struct_0"]);
    let minted: Vec<&str> = replay.table().iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(minted, ["struct_0", "struct_1", "struct_2"]);
}

/// The sweep's lookups read the final ledger, and a layout nothing answers is
/// reported rather than minted.
#[test]
fn a_lookup_after_the_batch_answers_without_minting() {
    let f = factory();
    let mut replay = Replay::probe(&factory());
    let pair = req(&fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long)]), 16, &[]);
    let four = req(
        &fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long), (0x10, Ty::Uint), (0x14, Ty::Uint)]),
        24,
        &[],
    );
    assert_eq!(replay.lookup_or_mint(&pair).as_deref(), Some("struct_0"));
    assert_eq!(replay.lookup_or_mint(&four).as_deref(), Some("struct_1"));
    assert_eq!(replay.lookup(&pair), Ok(Some("struct_1".to_string())));
    let other =
        req(&fields(&f, &[(0, Ty::Long), (8, Ty::Uint), (0xc, Ty::Uint)]), 16, &[]);
    assert_eq!(replay.lookup(&other), Err(WouldMint));
    assert_eq!(replay.table().len(), 2);
}

/// A name another type already holds is skipped, exactly as the ledger's scan
/// skips it, and a held synthesized shape answers like one the run minted.
#[test]
fn a_held_name_is_never_minted_over() {
    let live = factory();
    let foreign = live.get_type_struct("struct_0").unwrap();
    let named = vec![TypeField::new(0, 0, "flags", ty(&live, Ty::Long))];
    live.set_fields_struct_raw(&foreign, named, Vec::new(), 8, 1, 0).unwrap();
    let mut replay = Replay::probe(&live);
    let fs = fields(&live, &[(0, Ty::CharPtr), (8, Ty::Long)]);
    let request = req(&fs, 16, &[]);
    let real = ledger::lookup_or_mint(&live, fs, 16, &[], &[], crate::kuna_structmerge::StructMergeMode::Off).map(|t| t.get_name().to_string());
    assert_eq!(real.as_deref(), Some("struct_1"));
    assert_eq!(replay.lookup_or_mint(&request), real);
    let again = Replay::probe(&live);
    assert_eq!(again.lookup(&request), Ok(Some("struct_1".to_string())));
}

#[test]
fn a_table_rebuilds_the_same_structures_in_another_factory() {
    let live = factory();
    let mut replay = Replay::probe(&factory());
    for (spec, size, unclaimed) in script() {
        let fs = fields(&live, &spec);
        replay.lookup_or_mint(&req(&fs, size, &unclaimed));
        ledger::lookup_or_mint(&live, fs, size, &unclaimed, &[], crate::kuna_structmerge::StructMergeMode::Off);
    }
    let other = factory();
    let table = decode_table(&encode_table(replay.table())).unwrap();
    install_table(&other, &table).unwrap();
    for (name, _) in replay.table() {
        let a = live.find_by_name(name).unwrap().unwrap();
        let b = other.find_by_name(name).unwrap().unwrap();
        assert_eq!(ledger::layout_of(&a), ledger::layout_of(&b), "{name}");
        assert_eq!(ledger::members_of(&a), ledger::members_of(&b), "{name}");
        assert_eq!(a.get_size(), b.get_size());
    }
    // A second install finds its names taken.
    assert!(install_table(&other, &table).is_err());

    // A worker that minted its own structures forgets them, and only them,
    // before it takes the table.
    let worker = factory();
    let foreign = worker.get_type_struct("struct_0").unwrap();
    let named = vec![TypeField::new(0, 0, "flags", ty(&worker, Ty::Long))];
    worker.set_fields_struct_raw(&foreign, named, Vec::new(), 8, 1, 0).unwrap();
    let held = held_names(&worker);
    assert_eq!(held, ["struct_0"]);
    let words = fields(&worker, &[(0, Ty::Long), (8, Ty::Long), (0x10, Ty::Long)]);
    ledger::lookup_or_mint(&worker, words, 24, &[], &[], crate::kuna_structmerge::StructMergeMode::Off).unwrap();
    assert!(install_table(&worker, &table).is_err());
    forget_minted(&worker, &held).unwrap();
    assert_eq!(held_names(&worker), ["struct_0"]);
    let shifted: Vec<(String, SynthRequest)> =
        table.iter().map(|(n, q)| (n.replace("struct_", "struct_1"), q.clone())).collect();
    install_table(&worker, &shifted).unwrap();

    // The replayed names are the ones a worker mints first.
    let fresh = factory();
    let held = held_names(&fresh);
    for (spec, size, unclaimed) in script().into_iter().rev() {
        let st = ledger::lookup_or_mint(&fresh, fields(&fresh, &spec), size, &unclaimed, &[], crate::kuna_structmerge::StructMergeMode::Off).unwrap();
        // The parameter's pointer keeps the structure reachable.
        fresh.get_type_pointer(8, st, 1).unwrap();
    }
    assert!(install_table(&fresh, &table).is_err());
    forget_minted(&fresh, &held).unwrap();
    assert!(held_names(&fresh).is_empty());
    assert!(fresh.dependent_order().iter().all(|t| ledger::minted_number(t).is_none()));
    install_table(&fresh, &table).unwrap();
    let minted: Vec<String> = fresh
        .dependent_order()
        .iter()
        .filter(|t| ledger::minted_number(t).is_some())
        .map(|t| t.get_name().to_string())
        .collect();
    assert_eq!(minted.len(), 3, "each replayed name exactly once: {minted:?}");
}

#[test]
fn a_field_type_is_rebuilt_only_when_it_is_the_same_type() {
    let f = factory();
    let g = factory();
    for t in [Ty::CharPtr, Ty::Long, Ty::Uint, Ty::Bytes(8)] {
        let ct = ty(&f, t);
        let recipe = TypeRecipe::of(&ct).expect("core types, pointers and byte arrays travel");
        let rebuilt = recipe.build(&g).expect("rebuilds");
        assert_eq!(TypeRecipe::of(&rebuilt), Some(recipe));
    }
    // A structure the ledger minted is named by its process, so it never travels.
    let minted =
        ledger::lookup_or_mint(&f, fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long)]), 16, &[], &[], crate::kuna_structmerge::StructMergeMode::Off).unwrap();
    assert_eq!(TypeRecipe::of(&minted), None);
    let to_minted = f.get_type_pointer(8, minted, 1).unwrap();
    assert_eq!(TypeRecipe::of(&to_minted), None);
    // An opaque library shell has no members, and still travels by its name.
    let file = f.get_type_struct("FILE").unwrap();
    let file_ptr = f.get_type_pointer(8, file, 1).unwrap();
    let shell = TypeRecipe::of(&file_ptr).expect("a pointer to a named shell travels");
    g.get_type_struct("FILE").unwrap();
    assert!(shell.build(&g).is_some());
    // A name the other factory does not hold is not rebuilt as something else.
    let missing = TypeRecipe::Named {
        name: "DIR".into(),
        id: 7,
        meta: type_metatype::TYPE_STRUCT.as_i32(),
        size: 216,
    };
    assert!(missing.build(&g).is_none());
}

#[test]
fn a_record_survives_the_wire() {
    let f = factory();
    let record = FunctionRecord {
        requests: script()
            .into_iter()
            .map(|(spec, size, unclaimed)| req(&fields(&f, &spec), size, &unclaimed))
            .collect(),
        answers: vec![
            None,
            Some(("struct_3".into(), None)),
            Some(("struct_4".into(), Some(req(&fields(&f, &[(0, Ty::Long)]), 8, &[])))),
        ],
        off_script: true,
    };
    let mut bytes = vec![];
    record.encode(&mut bytes);
    bytes.extend_from_slice(b"trailing frame bytes");
    let (back, used) = FunctionRecord::decode(&bytes).unwrap();
    assert_eq!(back, record);
    assert_eq!(&bytes[used..], b"trailing frame bytes");
    assert!(FunctionRecord::decode(&bytes[..used - 1]).is_none());
}

#[test]
fn a_forced_hook_answers_in_order_and_notices_a_changed_script() {
    let f = factory();
    let first = fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long)]);
    let minted = ledger::lookup_or_mint(&f, first.clone(), 16, &[], &[], crate::kuna_structmerge::StructMergeMode::Off).unwrap();
    let hook = ShardHook::forcing(at_load());

    let second = fields(&f, &[(0, Ty::Long), (8, Ty::Long), (0x10, Ty::Long)]);
    hook.borrow_mut().begin(&[Some(minted.get_name().to_string()), None]);
    let a = lookup(&hook, &f, first.clone(), 16, &[], &[]);
    assert!(a.is_some_and(|t| Rc::ptr_eq(&t, &minted)));
    assert!(lookup(&hook, &f, second, 24, &[], &[]).is_none());
    let record = hook.borrow_mut().take();
    assert_eq!(record.requests.len(), 2);
    assert!(!record.off_script);

    // A repeated lookup is answered as the first one was and takes no answer.
    hook.borrow_mut().begin(&[Some(minted.get_name().to_string())]);
    assert!(lookup(&hook, &f, first.clone(), 16, &[], &[]).is_some());
    assert!(lookup(&hook, &f, first.clone(), 16, &[], &[]).is_some_and(|t| Rc::ptr_eq(&t, &minted)));
    let record = hook.borrow_mut().take();
    assert!(!record.off_script);
    assert_eq!(record.requests.len(), 2);
    assert_eq!(distinct(&record.requests).len(), 1);

    // One answer too few, one too many, and a name nobody minted.
    hook.borrow_mut().begin(&[]);
    assert!(lookup(&hook, &f, first.clone(), 16, &[], &[]).is_none());
    assert!(hook.borrow_mut().take().off_script);
    hook.borrow_mut().begin(&[None, None]);
    lookup(&hook, &f, first.clone(), 16, &[], &[]);
    lookup(&hook, &f, first.clone(), 16, &[], &[]);
    assert!(hook.borrow_mut().take().off_script);
    hook.borrow_mut().begin(&[Some("struct_9".into())]);
    assert!(lookup(&hook, &f, first, 16, &[], &[]).is_none());
    assert!(hook.borrow_mut().take().off_script);
}

#[test]
fn a_recording_hook_lets_the_ledger_answer() {
    let f = factory();
    let hook = ShardHook::recording(at_load());
    hook.borrow_mut().begin(&[]);
    let fs = fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long)]);
    let a = lookup(&hook, &f, fs.clone(), 16, &[], &[]).unwrap();
    let b = lookup(&hook, &f, fs.clone(), 16, &[], &[]).unwrap();
    assert!(Rc::ptr_eq(&a, &b));
    assert_eq!(a.get_name(), "struct_0");
    let record = hook.borrow_mut().take();
    assert_eq!(record.requests, vec![req(&fs, 16, &[]); 2]);
    assert!(record.requests.iter().all(SynthRequest::portable));
}

/// The header lists minted structures in the same order every run: after every
/// other type, by number.
#[test]
fn minted_structures_are_rendered_by_number() {
    let f = factory();
    let minted: Vec<Rc<Datatype>> = [
        (vec![(0, Ty::CharPtr), (8, Ty::Long)], 16),
        (vec![(0, Ty::Long), (8, Ty::Long), (0x10, Ty::Long)], 24),
        (vec![(0, Ty::Uint), (4, Ty::Uint), (8, Ty::Long)], 16),
    ]
    .into_iter()
    .map(|(spec, size)| ledger::lookup_or_mint(&f, fields(&f, &spec), size, &[], &[], crate::kuna_structmerge::StructMergeMode::Off).unwrap())
    .collect();
    let long = ty(&f, Ty::Long);
    let order = vec![
        Rc::clone(&minted[2]),
        Rc::clone(&long),
        Rc::clone(&minted[0]),
        Rc::clone(&minted[1]),
    ];
    let names: Vec<String> =
        ledger::in_name_order(order).iter().map(|t| t.get_name().to_string()).collect();
    assert_eq!(names, ["int8", "struct_0", "struct_1", "struct_2"]);
}

/// A named type the worker's load did not create is interned by some worker
/// and not by another (`pebnames` makes `PEB` the first time a function reads
/// it), so a field typed by one has no recipe and its structure cannot travel.
#[test]
fn a_type_interned_after_the_load_does_not_travel() {
    let f = factory();
    let loaded = AtLoad::of(&f);
    let peb = f.get_type_struct("PEB").unwrap();
    let named = vec![TypeField::new(0, 2, "BeingDebugged", ty(&f, Ty::Uint))];
    let peb = f.set_fields_struct_raw(&peb, named, Vec::new(), 8, 1, 0).unwrap();
    let peb_ptr = f.get_type_pointer(8, peb, 1).unwrap();
    let fs = vec![
        TypeField::new(0, 0, "field_0x0", ty(&f, Ty::Uint)),
        TypeField::new(1, 8, "field_0x8", peb_ptr),
    ];
    assert!(!SynthRequest::of(&fs, 16, &[], &[], &loaded).portable());
    assert!(SynthRequest::of(&fs, 16, &[], &[], &AtLoad::of(&f)).portable());
    assert!(req(&fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long)]), 16, &[]).portable());
}

/// A recording worker reports its own ledger's answer to every lookup, with
/// the lookup that minted the structure, and a name held at load stands alone.
#[test]
fn a_recording_hook_reports_its_own_answers() {
    let f = factory();
    let flags = fields(&f, &[(0, Ty::Long)]);
    ledger::lookup_or_mint(&f, flags.clone(), 8, &[], &[], crate::kuna_structmerge::StructMergeMode::Off).unwrap();
    let hook = ShardHook::recording(Rc::new(AtLoad::of(&f)));
    let pair = fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long)]);
    let minting = req(&pair, 16, &[]);

    hook.borrow_mut().begin(&[]);
    lookup(&hook, &f, pair.clone(), 16, &[], &[]).unwrap();
    lookup(&hook, &f, pair.clone(), 16, &[], &[]).unwrap();
    let first = hook.borrow_mut().take();
    let own = Some(("struct_1".to_string(), Some(minting.clone())));
    assert_eq!(first.answers, vec![own.clone(), own.clone()]);
    assert_eq!(first.own_answers(), Some(vec![own.clone()]));

    // A later function that reuses the structure reports the lookup that
    // minted it, not its own.
    hook.borrow_mut().begin(&[]);
    lookup(&hook, &f, pair, 16, &[(0x10, 8)], &[]);
    lookup(&hook, &f, flags, 8, &[], &[]).unwrap();
    let second = hook.borrow_mut().take();
    assert_eq!(second.answers[0], own);
    assert_eq!(second.answers[1], Some(("struct_0".to_string(), None)));

    // A repeat answered differently leaves nothing to rename by.
    let mut split = first.clone();
    split.answers[1] = None;
    assert_eq!(split.own_answers(), None);
    split.answers.pop();
    assert_eq!(split.own_answers(), None);
}

#[test]
fn a_renaming_pairs_structures_with_the_same_members() {
    let f = factory();
    let pair = req(&fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long)]), 16, &[]);
    let wide = req(&fields(&f, &[(0, Ty::CharPtr), (8, Ty::Uint)]), 16, &[]);
    let a = |name: &str, def: &SynthRequest| Some((name.to_string(), Some(def.clone())));
    let base = Some(("struct_0".to_string(), None));
    assert_eq!(
        renaming(&[a("struct_1", &pair), base.clone(), None], &[a("struct_7", &pair), base.clone(), None]),
        Some(vec![("struct_1".into(), "struct_7".into()), ("struct_0".into(), "struct_0".into())])
    );
    // The same name for one structure twice is one pair.
    assert!(renaming(&[a("struct_1", &pair), a("struct_1", &pair)], &[a("struct_7", &pair), a("struct_7", &pair)])
        .is_some());
    // Other members, a mint against a held name, an answer against none, a
    // held name against another, and one name for two.
    assert_eq!(renaming(&[a("struct_1", &pair)], &[a("struct_7", &wide)]), None);
    assert_eq!(renaming(&[a("struct_1", &pair)], &[base.clone()]), None);
    assert_eq!(renaming(&[a("struct_1", &pair)], &[None]), None);
    assert_eq!(renaming(&[base], &[Some(("struct_2".to_string(), None))]), None);
    assert_eq!(renaming(&[a("struct_1", &pair), a("struct_2", &pair)], &[a("struct_7", &pair), a("struct_7", &pair)]), None);
    assert_eq!(renaming(&[a("struct_1", &pair)], &[]), None);
    // A request without a recipe could be any type, so it pairs with nothing.
    let mut opaque = pair.clone();
    opaque.fields[0].ty = None;
    assert_eq!(renaming(&[a("struct_1", &opaque)], &[a("struct_7", &opaque)]), None);
}

#[test]
fn a_rename_touches_whole_identifiers_outside_literals() {
    let map = vec![("struct_1".to_string(), "struct_10".to_string()), ("struct_10".to_string(), "struct_2".to_string())];
    let keep = vec!["struct_0".to_string()];
    let r = |t: &str| rename_identifiers(t, &map, &keep);
    assert_eq!(
        r("void f(struct_1 *a0,struct_10 *a1,struct_0 *a2) // can't\n{ a0->my_struct_1 = struct_1x; }").as_deref(),
        Some("void f(struct_10 *a0,struct_2 *a1,struct_0 *a2) // can't\n{ a0->my_struct_1 = struct_1x; }")
    );
    assert_eq!(r("typedef struct struct_1 struct_1;").as_deref(), Some("typedef struct struct_10 struct_10;"));
    assert_eq!(r("/* struct_1 */ struct_1").as_deref(), Some("/* struct_10 */ struct_10"));
    assert_eq!(r("struct_").as_deref(), Some("struct_"));
    assert_eq!(r("").as_deref(), Some(""));
    // A name the renaming does not cover, and one spelled inside a literal.
    assert_eq!(r("struct_3 *p;"), None);
    assert_eq!(r("puts(\"struct_1\");"), None);
    assert_eq!(r("c = '\\''; struct_1 *p;").as_deref(), Some("c = '\\''; struct_10 *p;"));
    assert_eq!(r("s = \"a\\\"b\"; struct_1 *p;").as_deref(), Some("s = \"a\\\"b\"; struct_10 *p;"));
}

#[test]
fn the_replay_knows_the_names_held_before_the_run() {
    let live = factory();
    let foreign = live.get_type_struct("struct_1").unwrap();
    let named = vec![TypeField::new(0, 0, "flags", ty(&live, Ty::Long))];
    live.set_fields_struct_raw(&foreign, named, Vec::new(), 8, 1, 0).unwrap();
    let mut replay = Replay::probe(&live);
    assert_eq!(replay.held(), ["struct_1"]);
    replay.lookup_or_mint(&req(&fields(&live, &[(0, Ty::CharPtr), (8, Ty::Long)]), 16, &[]));
    assert_eq!(replay.table()[0].0, "struct_0");
    assert_eq!(replay.held(), ["struct_1"]);
}

/// A field that points at its own structure names a type only the minting
/// process can number, so the request is not portable and its layout spells
/// the field as a self pointer.
#[test]
fn a_self_pointing_request_is_not_portable() {
    let f = factory();
    let fs = fields(&f, &[(0, Ty::CharPtr), (8, Ty::Long)]);
    let plain = SynthRequest::of(&fs, 16, &[], &[], &AtLoad::of(&f));
    assert!(plain.portable());
    let selfish = SynthRequest::of(&fs, 16, &[], &[0], &AtLoad::of(&f));
    assert!(!selfish.portable());
    assert_eq!(selfish.want.fields[0].ty, ledger::SELF_KEY);
    assert_eq!(selfish.own[0].claim.as_ref().map(|k| k.ty.as_str()), Some(ledger::SELF_KEY));
    assert!(!selfish.defines_same(&selfish.clone()));
}
