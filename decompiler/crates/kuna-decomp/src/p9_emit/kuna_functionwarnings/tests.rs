use super::{trim_lines, Warning, STEMS};
use crate::printc::warning_slug;

fn warning(text: &str) -> Warning {
    Warning::new(Some(0x1000), text)
}

/// The slug `warning_slug` gives `text`, less its `jt: ` prefix, an inlined
/// name and an ` xN` count.
fn slug_base(text: &str) -> String {
    let slug = warning_slug(text);
    let slug = slug.strip_prefix("jt: ").unwrap_or(&slug).to_string();
    if slug.starts_with("inlined: ") {
        return "inlined".into();
    }
    match slug.rsplit_once(" x") {
        Some((base, n)) if n.bytes().all(|b| b.is_ascii_digit()) => base.into(),
        _ => slug,
    }
}

#[test]
fn an_abbreviated_warning_s_kind_is_its_slug() {
    for (text, kind) in [
        ("Subroutine does not return", "no-return"),
        ("Does not return", "no-return"),
        ("Treating indirect jump as call", "jump-as-call"),
        ("Treating indirect jump as return", "jump-as-return"),
        ("Could not inline here", "inline-failed"),
        ("No fallthrough prevents inlining here", "inline-failed"),
        ("Return address prevents inlining here", "inline-failed"),
        ("Read-only address 0x2000 written", "writes-rodata"),
        ("branchflip: negated the guard", "branch-flip"),
        ("tailcalljump: jump to f is a tail call", "tail-call"),
        ("taildup: duplicated 2 tails", "return-dupe"),
        ("crossjumprevert: split 3 shared tails", "crossjump-dupe"),
        ("int3pad: 11 bytes of int3 padding", "int3-pad"),
        ("outline: 4 outlined bodies", "outlined"),
        ("returndup: duplicated 5 shared bare-epilogue return(s)", "return-dupe"),
        ("earlyreturn: hoisted 3 returns", "early-return"),
        ("switchreturn: 2 cases return", "switch-return"),
        ("dedupitetail: merged 2 tails", "ite-dedupe"),
        ("iteregion: 2 ternaries", "ternary"),
        ("ifelseflatten: 6 else blocks", "else-flattened"),
        ("Inlined function: g<int, int> x2", "inlined"),
        ("Function: f replaced with injection: g", "injected"),
        ("Unable to use symbol s: size mismatch", "symbol-size-mismatch"),
    ] {
        for text in [format!("WARNING: {text}"), format!("WARNING (jumptable): {text}")] {
            assert!(!warning_slug(&text).contains("warn: "), "{text} is abbreviated");
            assert_eq!(slug_base(&text), kind, "{text}");
            assert_eq!(warning(&text).kind(), kind, "{text}");
        }
    }
}

#[test]
fn a_stem_is_one_the_slugs_leave_whole() {
    for (stem, _) in STEMS {
        for text in [format!("WARNING: {stem}"), format!("WARNING (jumptable): {stem}")] {
            assert!(warning_slug(&text).contains("warn: "), "{stem} is abbreviated; its kind is its slug");
        }
    }
    for (text, kind) in [
        ("WARNING: Bad instruction - Truncating control flow here", "bad-instruction"),
        ("WARNING: Control flow ignored unimplemented instructions", "unimplemented-instruction"),
        ("WARNING: Exceeded the 1 instruction budget: some flow is truncated", "instruction-limit"),
        ("WARNING (jumptable): Could not recover jumptable at 0x10. Too many branches", "jumptable"),
        ("WARNING: structured blocks unavailable (structuring declined)", "structuring-declined"),
        ("WARNING: Nullable weak code relocation could not be modeled", "weak-relocation"),
        (
            "WARNING: Nullable weak relocation operand could not be modeled; address guards may be incomplete",
            "weak-relocation",
        ),
        ("WARNING: funcboundflow: fall-through reached the next function entry", "funcboundflow"),
        ("WARNING: calltrampoline: 0x0044a677 discards the return address", "calltrampoline"),
        ("WARNING: ptrarith problems", "other"),
        ("WARNING: Unknown: data", "other"),
    ] {
        assert_eq!(warning(text).kind(), kind, "{text}");
    }
}

#[test]
fn a_message_is_the_text_less_its_marker() {
    assert_eq!(warning("WARNING: Subroutine does not return").message(), "Subroutine does not return");
    assert_eq!(
        warning("WARNING (jumptable): Treating indirect jump as return").message(),
        "(jumptable): Treating indirect jump as return"
    );
    assert_eq!(warning("no marker").message(), "no marker");
}

#[test]
fn lines_are_rebased_past_the_leading_newlines() {
    let mut warnings = vec![warning("WARNING: a"), warning("WARNING: b")];
    warnings[0].line = 3;
    warnings[1].line = 7;
    trim_lines(&mut warnings, "\n\nvoid f(void)\n");
    assert_eq!(warnings.iter().map(|w| w.line).collect::<Vec<_>>(), [1, 5]);
    trim_lines(&mut warnings, "void f(void)\n");
    assert_eq!(warnings.iter().map(|w| w.line).collect::<Vec<_>>(), [1, 5]);
}
