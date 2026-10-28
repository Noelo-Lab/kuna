use super::*;

fn position(code: &str, at: usize) -> Position {
    let line = code[..at].bytes().filter(|&b| b == b'\n').count() + 1;
    let column = at - code[..at].rfind('\n').map_or(0, |i| i + 1);
    Position { line, column }
}

fn parameter(spelling: &str) -> Parameter {
    Parameter {
        storage: (("register".into(), 0x38), 8),
        spelling: spelling.into(),
        void_pointer: false,
        dependencies: vec![],
        forwards: vec![],
    }
}

fn records() -> (Record, Record) {
    let callee = Record {
        entry: ("ram".into(), 0x1000),
        parameters: vec![None, Some(parameter("unsigned char *"))],
        calls: vec![],
        language: OutLang::C,
        struct_defs: false,
        texts: vec![],
        preamble_end: Position { line: 1, column: 0 },
    };
    let caller = Record {
        entry: ("ram".into(), 0x2000),
        parameters: vec![],
        calls: vec![Call {
            callee: callee.entry.clone(),
            index: 1,
            storage: callee.parameters[1].as_ref().unwrap().storage.clone(),
            actual: "unsigned long *".into(),
            position: Position { line: 1, column: 7 },
            end: Position { line: 1, column: 10 },
            void_pointer: false,
            text: 0,
            text_start: 0,
            text_end: 3,
        }],
        language: OutLang::C,
        struct_defs: false,
        texts: vec![Arc::new("&v1".into())],
        preamble_end: Position { line: 1, column: 0 },
    };
    (caller, callee)
}

fn insertion(code: &str, pool: Arc<String>, start: usize, end: usize, language: OutLang) -> Insertion {
    Insertion {
        position: position(code, start),
        end: position(code, end),
        text_start: 0,
        text_end: pool.len(),
        expression: pool,
        spelling: if language == OutLang::C { "unsigned char *" } else { "*mut u8" }.into(),
        language,
        dependencies: vec![],
        forwards: vec![],
    }
}

#[test]
fn final_declarations_decide_independently_of_order() {
    let (caller, callee) = records();
    let a = insertions(&caller, &definitions([&caller, &callee]));
    let b = insertions(&caller, &definitions([&callee, &caller]));
    assert_eq!(a, b);
    assert!(Arc::ptr_eq(&a[0].expression, &caller.texts[0]));
    let applied = apply("sink(0,&v1);", &a);
    assert_eq!(applied.code, "sink(0,((unsigned char *)(&v1)));");
    assert_eq!(applied.inserted, a);
}

#[test]
fn accepted_pointer_records_and_applied_metadata_cross_worker_threads() {
    let (caller, callee) = records();
    let pool = Arc::clone(&caller.texts[0]);
    let applied = std::thread::spawn(move || {
        apply("sink(0,&v1);", &insertions(&caller, &definitions([&callee])))
    }).join().unwrap();
    assert_eq!(applied.code, "sink(0,((unsigned char *)(&v1)));");
    assert!(applied.skipped.is_empty());
    assert_eq!(applied.inserted.len(), 1);
    assert!(Arc::ptr_eq(&applied.inserted[0].expression, &pool));
}

#[test]
fn missing_ambiguous_or_differently_stored_parameters_abstain() {
    let (mut caller, callee) = records();
    assert!(insertions(&caller, &definitions([&caller])).is_empty());
    let mut other = callee.clone();
    other.parameters[1].as_mut().unwrap().spelling = "char *".into();
    assert!(insertions(&caller, &definitions([&callee, &other, &callee])).is_empty());
    let defs = definitions([&callee]);
    caller.calls[0].index = 0;
    assert!(insertions(&caller, &defs).is_empty());
    caller.calls[0].index = 2;
    assert!(insertions(&caller, &defs).is_empty());
    caller.calls[0].index = 1;
    caller.calls[0].storage.1 = 4;
    assert!(insertions(&caller, &defs).is_empty());
    caller.calls[0].storage.1 = 8;
    caller.calls[0].storage.0 .1 += 1;
    assert!(insertions(&caller, &defs).is_empty());
}

#[test]
fn final_language_must_match_and_duplicate_language_conflicts_abstain() {
    let (mut caller, callee) = records();
    caller.language = OutLang::Rust;
    assert!(insertions(&caller, &definitions([&callee])).is_empty());
    let mut other = callee.clone();
    other.language = OutLang::Rust;
    assert!(insertions(&caller, &definitions([&callee, &other])).is_empty());
    assert!(insertions(&caller, &definitions([&other, &callee, &other])).is_empty());
    assert_eq!(insertions(&caller, &definitions([&other])).len(), 1);
}

#[test]
fn c_void_object_compatibility_does_not_hide_rust_casts() {
    for target_void in [false, true] {
        let (mut caller, mut callee) = records();
        caller.calls[0].void_pointer = !target_void;
        callee.parameters[1].as_mut().unwrap().void_pointer = target_void;
        assert!(insertions(&caller, &definitions([&callee])).is_empty());
        caller.language = OutLang::Rust;
        callee.language = OutLang::Rust;
        callee.parameters[1].as_mut().unwrap().spelling =
            if target_void { "*mut core::ffi::c_void" } else { "*mut u8" }.into();
        assert_eq!(insertions(&caller, &definitions([&callee])).len(), 1);
    }
}

#[test]
fn matching_actual_spelling_is_idempotent() {
    let (mut caller, callee) = records();
    caller.calls[0].actual = callee.parameters[1].as_ref().unwrap().spelling.clone();
    assert!(insertions(&caller, &definitions([&callee])).is_empty());
    let (caller, callee) = records();
    let edits = insertions(&caller, &definitions([&callee]));
    let first = apply("sink(0,&v1);", &edits);
    let second = apply(&first.code, &edits);
    assert_eq!(second.code, first.code);
    assert!(second.inserted.is_empty());
}

#[test]
fn each_printed_occurrence_is_edited_and_neighbouring_text_is_preserved() {
    let (mut caller, callee) = records();
    let mut again = caller.calls[0].clone();
    again.position.line = 2;
    again.end.line = 2;
    caller.calls.push(again);
    let edits = insertions(&caller, &definitions([&callee]));
    assert_eq!(
        apply("sink(0,&v1);\nsink(0,&v1); // &v1\n", &edits).code,
        "sink(0,((unsigned char *)(&v1)));\nsink(0,((unsigned char *)(&v1))); // &v1\n"
    );
}

#[test]
fn stale_and_ambiguous_sites_stay_uncast_and_leave_the_document_intact() {
    let (caller, callee) = records();
    let edits = insertions(&caller, &definitions([&callee]));
    for code in ["sink(0,&v2);", "sink(0,&v10);", "a\nxxxxx&v1", "sink(0,"] {
        let applied = apply(code, &edits);
        assert_eq!(applied.code, code);
        assert_eq!(applied.skipped.len(), 1, "{code}");
        assert!(applied.inserted.is_empty());
    }
    let twice = apply("sink(0,&v1);", &[edits[0].clone(), edits[0].clone()]);
    assert_eq!(twice.code, "sink(0,&v1);");
    assert_eq!(twice.skipped.len(), 2);
    assert!(twice.inserted.is_empty());
    let mut broken = edits[0].clone();
    broken.spelling = "unsigned\nchar *".into();
    assert_eq!(apply("sink(0,&v1);", &[broken]).code, "sink(0,&v1);");
}

#[test]
fn whole_span_positions_ranges_and_utf8_boundaries_are_validated() {
    let code = "sink(é);";
    let edit = insertion(code, Arc::new("é".into()), 5, 7, OutLang::C);
    assert_eq!(apply(code, &[edit.clone()]).inserted.len(), 1);
    for bad in 0..7 {
        let mut broken = edit.clone();
        match bad {
            0 => broken.position.line = 0,
            1 => broken.end.line = 2,
            2 => broken.position.column = 6,
            3 => broken.end.column = 6,
            4 => broken.text_start = 1,
            5 => broken.text_end = 3,
            _ => broken.end.column = 5,
        }
        let applied = apply(code, &[broken]);
        assert_eq!(applied.code, code);
        assert!(applied.inserted.is_empty());
        assert_eq!(applied.skipped.len(), 1);
    }
}

#[test]
fn one_drifted_site_does_not_cost_the_other_casts() {
    let (mut caller, callee) = records();
    let mut drifted = caller.calls[0].clone();
    drifted.position = Position { line: 2, column: 3 };
    drifted.end = Position { line: 2, column: 6 };
    caller.calls.push(drifted);
    let mut later = caller.calls[0].clone();
    later.position.line = 3;
    later.end.line = 3;
    caller.calls.push(later);
    let applied = apply(
        "sink(0,&v1);\nsink(0,&v1);\nsink(0,&v1);",
        &insertions(&caller, &definitions([&callee])),
    );
    assert_eq!(applied.code,
        "sink(0,((unsigned char *)(&v1)));\nsink(0,&v1);\nsink(0,((unsigned char *)(&v1)));");
    assert_eq!(applied.skipped.len(), 1);
    assert_eq!(applied.skipped[0].0.position.line, 2);
    assert_eq!(applied.inserted.len(), 2);
}

#[test]
fn nested_multiline_arguments_share_the_pool_and_stream_in_both_languages() {
    let code = "sink(wrap(\n  provider(value)\n)); // provider(value)\n";
    let outer_start = code.find("wrap(").unwrap();
    let outer_end = code.find(");").unwrap();
    let pool = Arc::new(code[outer_start..outer_end].to_string());
    let inner_start = code.find("value").unwrap();
    for language in [OutLang::C, OutLang::Rust] {
        let outer = insertion(code, Arc::clone(&pool), outer_start, outer_end, language);
        let mut inner = insertion(code, Arc::clone(&pool), inner_start, inner_start + 5, language);
        inner.text_start = inner_start - outer_start;
        inner.text_end = inner.text_start + 5;
        let applied = apply(code, &[inner, outer]);
        let expected = if language == OutLang::C {
            "sink(((unsigned char *)(wrap(\n  provider(((unsigned char *)(value)))\n)))); // provider(value)\n"
        } else {
            "sink(((wrap(\n  provider(((value) as *mut u8))\n)) as *mut u8)); // provider(value)\n"
        };
        assert_eq!(applied.code, expected);
        assert!(applied.skipped.is_empty());
        assert_eq!(applied.inserted.len(), 2);
        assert!(applied.inserted.iter().all(|e| Arc::ptr_eq(&e.expression, &pool)));
    }
}

#[test]
fn crossing_spans_decline_both_but_preserve_a_disjoint_neighbour() {
    let code = "a+b+c; d";
    let a = insertion(code, Arc::new("a+b".into()), 0, 3, OutLang::C);
    let b = insertion(code, Arc::new("b+c".into()), 2, 5, OutLang::C);
    let c = insertion(code, Arc::new("d".into()), 7, 8, OutLang::C);
    let applied = apply(code, &[a, b, c]);
    assert_eq!(applied.code, "a+b+c; ((unsigned char *)(d))");
    assert_eq!(applied.skipped.len(), 2);
    assert_eq!(applied.inserted.len(), 1);
}

#[test]
fn later_sites_crossing_an_already_rejected_span_also_decline() {
    let code = "a+b+c+d+e";
    let edits: Vec<_> = [(0, 5), (2, 9), (4, 7)].into_iter().map(|(start, end)| {
        insertion(code, Arc::new(code[start..end].into()), start, end, OutLang::C)
    }).collect();
    let applied = apply(code, &edits);
    assert_eq!(applied.code, code);
    assert_eq!(applied.skipped.len(), 3);
    assert!(applied.inserted.is_empty());
}

#[test]
fn structure_renames_rebase_both_positions_without_moving_other_lines() {
    let (mut caller, _) = records();
    let code = "struct_1 *p; sink(0,&v1);\nsink(0,&v1);";
    let start = code.find("&v1").unwrap();
    caller.calls[0].position = position(code, start);
    caller.preamble_end = position(code, start);
    caller.calls[0].end = position(code, start + 3);
    let mut again = caller.calls[0].clone();
    again.position = Position { line: 2, column: 7 };
    again.end = Position { line: 2, column: 10 };
    caller.calls.push(again);
    let pool = Arc::clone(&caller.texts[0]);
    caller.renamed(code, &[(0, 8, "struct_100".into())]);
    assert_eq!(caller.calls[0].position, Position { line: 1, column: start + 2 });
    assert_eq!(caller.calls[0].end, Position { line: 1, column: start + 5 });
    assert_eq!(caller.preamble_end, Position { line: 1, column: start + 2 });
    assert_eq!(caller.calls[1].position, Position { line: 2, column: 7 });
    assert_eq!(caller.calls[1].end, Position { line: 2, column: 10 });
    assert!(Arc::ptr_eq(&pool, &caller.texts[0]));
}

#[test]
fn renaming_updates_actual_contract_metadata_and_shared_nested_ranges() {
    let code = "sink(convert(struct_1_ptr, (struct_1 *)value));";
    let outer_expression = "convert(struct_1_ptr, (struct_1 *)value)";
    let inner_expression = "(struct_1 *)value";
    let outer_start = code.find(outer_expression).unwrap();
    let outer_end = outer_start + outer_expression.len();
    let inner_start = code.find(inner_expression).unwrap();
    let inner_end = inner_start + inner_expression.len();
    let (mut caller, mut callee) = records();
    caller.texts = vec![Arc::new(code[outer_start..outer_end].into())];
    caller.calls[0].position = position(code, outer_start);
    caller.calls[0].end = position(code, outer_end);
    caller.calls[0].text_end = outer_end - outer_start;
    caller.calls[0].actual = "struct_1 *".into();
    let mut nested = caller.calls[0].clone();
    nested.position = position(code, inner_start);
    nested.end = position(code, inner_end);
    nested.text_start = inner_start - outer_start;
    nested.text_end = inner_end - outer_start;
    caller.calls.push(nested);
    assert_eq!(code.get(inner_start..inner_end), Some(inner_expression));
    assert_eq!(
        caller.texts[0].get(caller.calls[1].text_start..caller.calls[1].text_end),
        Some(inner_expression),
    );
    let target = callee.parameters[1].as_mut().unwrap();
    target.spelling = "struct_1 *".into();
    target.dependencies = vec!["struct_1".into()];
    target.forwards = vec!["typedef struct struct_1 struct_1;".into()];
    let edit_start = code.find("struct_1 *").unwrap();
    let edits = [(edit_start, edit_start + 8, "struct_100".into())];
    caller.renamed(code, &edits);
    // A record without calls must still rename its accepted target metadata.
    callee.renamed(code, &edits);
    let changed = "sink(convert(struct_1_ptr, (struct_100 *)value));";
    assert_eq!(caller.texts[0].as_str(), "convert(struct_1_ptr, (struct_100 *)value)");
    assert_eq!(caller.calls[0].actual, "struct_100 *");
    assert_eq!(caller.calls[1].actual, "struct_100 *");
    assert_eq!(caller.calls[1].text_start, inner_start - outer_start);
    assert_eq!(caller.calls[1].text_end, inner_end - outer_start + 2);
    assert_eq!(caller.calls[1].end, position(changed, inner_end + 2));
    let target = callee.parameters[1].as_ref().unwrap();
    assert_eq!(target.spelling, "struct_100 *");
    assert_eq!(target.dependencies, ["struct_100"]);
    assert_eq!(target.forwards, ["typedef struct struct_100 struct_100;"]);
    caller.calls.iter_mut().for_each(|c| c.actual = "unsigned long *".into());
    let applied = apply(changed, &insertions(&caller, &definitions([&callee])));
    assert_eq!(applied.inserted.len(), 2);
    assert!(applied.skipped.is_empty());
    assert!(applied.inserted.iter().all(|e| e.dependencies == ["struct_100"]));
}

#[test]
fn invalid_or_ambiguous_renames_cannot_authorize_stale_pointer_contracts() {
    let code = "struct_1 *left; struct_1 *right; sink(0,&v1);";
    let second = code.rfind("struct_1").unwrap();
    let start = code.find("&v1").unwrap();
    let (mut caller, mut callee) = records();
    caller.calls[0].position = position(code, start);
    caller.calls[0].end = position(code, start + 3);
    callee.parameters[1].as_mut().unwrap().spelling = "struct_1 *".into();
    assert_eq!(insertions(&caller, &definitions([&callee])).len(), 1);
    let invalid_edits = [
        vec![(code.len() + 1, code.len() + 2, "Item".into())],
        vec![(8, 0, "Item".into())],
        vec![(0, 8, "Item".into()), (0, 8, "Item".into())],
        vec![(1, 8, "Item".into())],
        vec![(0, 8, "bad\nname".into())],
        vec![(0, 8, "First".into()), (second, second + 8, "Second".into())],
    ];
    for edits in invalid_edits {
        let mut changed_caller = caller.clone();
        changed_caller.renamed(code, &edits);
        let applied = apply(code, &insertions(&changed_caller, &definitions([&callee])));
        assert_eq!(applied.code, code);
        assert!(applied.inserted.is_empty());
        assert!(changed_caller.calls.is_empty());
        assert!(changed_caller.texts.is_empty());

        let mut changed_callee = callee.clone();
        changed_callee.renamed(code, &edits);
        let applied = apply(code, &insertions(&caller, &definitions([&changed_callee])));
        assert_eq!(applied.code, code);
        assert!(applied.inserted.is_empty());
        assert!(changed_callee.parameters.iter().all(Option::is_none));
    }
    // Repeated occurrences of one unambiguous mapping remain authoritative.
    let mut renamed_callee = callee;
    renamed_callee.renamed(code, &[
        (second, second + 8, "Item".into()),
        (0, 8, "Item".into()),
    ]);
    assert_eq!(renamed_callee.parameters[1].as_ref().unwrap().spelling, "Item *");
    assert_eq!(insertions(&caller, &definitions([&renamed_callee])).len(), 1);
}

#[test]
fn unordered_identifier_edits_rebase_only_their_own_lines() {
    let code = "struct_1 *p; sink(0,&v1); struct_2 *q;\nstruct_3 *r; sink(0,&v1);";
    let (mut caller, _) = records();
    let first = code.find("&v1").unwrap();
    let second = code.rfind("&v1").unwrap();
    caller.calls[0].position = position(code, first);
    caller.calls[0].end = position(code, first + 3);
    let mut again = caller.calls[0].clone();
    again.position = position(code, second);
    again.end = position(code, second + 3);
    let original_second = again.position;
    caller.calls.push(again);
    let middle = code.find("struct_2").unwrap();
    let last = code.find("struct_3").unwrap();
    caller.renamed(code, &[
        (last, last + 8, "s".into()),
        (middle, middle + 8, "struct_200".into()),
        (0, 8, "struct_100".into()),
    ]);
    assert_eq!(caller.calls[0].position, position(code, first + 2));
    assert_eq!(caller.calls[0].end.column, first + 5);
    assert_eq!(caller.calls[1].position.column, original_second.column - 7);
    assert_eq!(caller.calls[1].end.column, original_second.column - 4);
}

#[test]
fn capture_pools_outermost_multiline_spans_once_before_trimming() {
    let code = "\n\nsink(wrap(\n  provider(value)\n)); other(&v1);";
    let (mut caller, callee) = records();
    let outer_start = code.find("wrap").unwrap();
    let outer_end = code.find(");").unwrap();
    let inner_start = code.find("value").unwrap();
    let neighbour_start = code.find("&v1").unwrap();
    let mut outer = caller.calls[0].clone();
    outer.position = position(code, outer_start);
    outer.end = position(code, outer_end);
    let mut inner = outer.clone();
    inner.position = position(code, inner_start);
    inner.end = position(code, inner_start + 5);
    let mut neighbour = outer.clone();
    neighbour.position = position(code, neighbour_start);
    neighbour.end = position(code, neighbour_start + 3);
    caller.calls = vec![inner, neighbour, outer];
    caller.capture_texts(code);
    assert_eq!(caller.texts.len(), 2);
    assert_eq!(caller.texts[0].as_str(), &code[outer_start..outer_end]);
    assert_eq!(caller.texts[1].as_str(), "&v1");
    assert_eq!(caller.calls[0].text, caller.calls[2].text);
    assert_eq!(caller.calls[0].text_start, inner_start - outer_start);
    assert_eq!(caller.calls[0].text_end, inner_start - outer_start + 5);
    assert_eq!(caller.calls[1].text, 1);
    assert_eq!(caller.calls[2].text_start, 0);
    assert_eq!(caller.calls[2].text_end, caller.texts[0].len());
    caller.trim_lines(code);
    let edits = insertions(&caller, &definitions([&callee]));
    assert!(Arc::ptr_eq(&edits[0].expression, &edits[1].expression));
    let applied = apply(code.trim_matches('\n'), &edits);
    assert!(applied.skipped.is_empty());
    assert_eq!(applied.inserted.len(), 3);
}

#[test]
fn capture_invalid_crossing_and_duplicate_spans_fails_closed() {
    let code = "a+b+c; &v1";
    let (mut caller, callee) = records();
    let template = caller.calls[0].clone();
    caller.calls.clear();
    for (start, end) in [(0, 3), (2, 5), (7, 10)] {
        let mut call = template.clone();
        call.position = position(code, start);
        call.end = position(code, end);
        caller.calls.push(call);
    }
    let mut missing = template.clone();
    missing.position = Position::default();
    caller.calls.push(missing);
    caller.capture_texts(code);
    assert_eq!(caller.texts.len(), 1);
    assert_eq!(caller.texts[0].as_str(), "&v1");
    assert_eq!(insertions(&caller, &definitions([&callee])).len(), 1);
    caller.calls = vec![caller.calls[2].clone(), caller.calls[2].clone()];
    caller.capture_texts(code);
    assert!(caller.texts.is_empty());
    assert!(insertions(&caller, &definitions([&callee])).is_empty());
}

#[test]
fn trimming_changes_both_lines_without_changing_columns() {
    let (mut caller, _) = records();
    caller.calls[0].position.line = 3;
    caller.calls[0].end.line = 4;
    caller.preamble_end = Position { line: 3, column: 0 };
    caller.trim_lines("\n\nsink(0,\n&v1);\n");
    assert_eq!(caller.calls[0].position, Position { line: 1, column: 7 });
    assert_eq!(caller.calls[0].end, Position { line: 2, column: 10 });
    assert_eq!(caller.preamble_end, Position { line: 1, column: 0 });
}

#[test]
fn trimming_an_empty_type_preamble_preserves_the_definition_insertion_boundary() {
    for leading in ["\n", "\n\n\n"] {
        let code = format!("{leading}void caller(void) {{ sink(0,&value); }}\n");
        let trimmed = code.trim_matches('\n');
        let start = code.find("&value").unwrap();
        let (mut caller, mut callee) = records();
        caller.struct_defs = true;
        callee.struct_defs = true;
        caller.calls[0].position = position(&code, start);
        caller.calls[0].end = position(&code, start + "&value".len());
        let target = callee.parameters[1].as_mut().unwrap();
        target.spelling = "B *".into();
        target.dependencies = vec!["B".into()];
        target.forwards = vec!["typedef struct B B;".into()];
        for line in 1..=leading.len() + 1 {
            let mut caller = caller.clone();
            caller.preamble_end = Position { line, column: 0 };
            caller.capture_texts(&code);
            caller.trim_lines(&code);
            assert_eq!(caller.preamble_end, Position { line: 1, column: 0 });
            assert_eq!(offset(trimmed, &line_starts(trimmed), caller.preamble_end), Some(0));
            let applied = apply(trimmed, &insertions(&caller, &definitions([&callee])));
            assert_eq!(applied.code, "void caller(void) { sink(0,((B *)(&value))); }");
            assert!(applied.skipped.is_empty());
            assert_eq!(applied.inserted.len(), 1);
            assert_eq!(applied.inserted[0].dependencies, ["B"]);
            assert_eq!(applied.inserted[0].forwards, ["typedef struct B B;"]);
        }
        caller.preamble_end = Position::default();
        caller.trim_lines(&code);
        assert_eq!(caller.preamble_end, Position::default());
    }
}

#[test]
fn actual_local_declarations_exclude_arrays_and_nontrivial_declarators() {
    let mut declarations = Declarations::default();
    declarations.record("value", "struct item", "", false);
    assert_eq!(declarations.address("value", OutLang::C).as_deref(), Some("struct item *"));
    declarations.record("pointer", "*mut u64", "", false);
    assert_eq!(declarations.address("pointer", OutLang::Rust).as_deref(), Some("*mut *mut u64"));
    declarations.record("item", "Item", "", false);
    assert_eq!(declarations.address("item", OutLang::Rust).as_deref(), Some("*mut Item"));
    declarations.record("value", "unsigned char", "", true);
    assert!(declarations.address("value", OutLang::C).is_none());
    assert!(declarations.address("value", OutLang::Rust).is_none());
    declarations.record("value", "void (*", ")(void)", false);
    assert!(declarations.address("value", OutLang::C).is_none());
    assert!(declarations.address("value", OutLang::Rust).is_none());
    declarations.record("value", "unsigned long", "", false);
    declarations.clear();
    assert!(declarations.address("value", OutLang::C).is_none());
    assert!(declarations.address("pointer", OutLang::Rust).is_none());
}

#[test]
fn emitter_positions_count_embedded_newlines_and_restart_after_taking_output() {
    use crate::prettyprint::{Emit, EmitNoMarkup, SyntaxHighlight};
    let mut emit = EmitNoMarkup::new();
    emit.print("comment\ntext\né", SyntaxHighlight::NoColor);
    assert_eq!(emit.position(), Position { line: 3, column: 2 });
    emit.take_output();
    emit.print("&v1", SyntaxHighlight::NoColor);
    assert_eq!(emit.position(), Position { line: 1, column: 3 });
}

#[test]
fn a_prototype_slot_without_a_parameter_keeps_the_record_and_its_place() {
    use crate::context::ArchContext;
    use crate::fspec::{ParameterPieces, ProtoStore, ProtoStoreInternal};
    use crate::funcdata::Funcdata;
    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    let mut manage = AddrSpaceManager::new();
    manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manage.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manage.insert_space(Rc::clone(&ram)).unwrap();
    let glb = Rc::new(ArchContext::new(manage));
    let mut fd = Funcdata::new("f", "f", glb, Address::new(Rc::clone(&ram), 0x1000), 0x1000, 0x20)
        .unwrap();
    let byte = Rc::new(Datatype::new(1, type_metatype::TYPE_UINT));
    let mut store = ProtoStoreInternal::new(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
    let pieces = ParameterPieces { addr: Address::new(ram, 0x38), type_: Some(byte), flags: 0 };
    store.set_input(1, "b", &pieces);
    fd.get_func_proto_mut().set_store(Box::new(store));
    assert!(fd.get_func_proto().get_param(0).is_none());
    let record = Record::new(&fd, OutLang::C, false)
        .expect("a sparse prototype still records its calls");
    assert_eq!(record.entry, ("ram".to_string(), 0x1000));
    assert_eq!(record.parameters, vec![None, None]);
}
