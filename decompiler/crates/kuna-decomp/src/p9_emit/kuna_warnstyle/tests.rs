use super::*;
use crate::printc::warning_slug;

#[test]
fn apply_parses_inline_banner_and_reports() {
    let (val, msg) = OptionWarnStyle.apply("inline").unwrap();
    assert!(val);
    assert_eq!(msg, "Warning style set to inline");
    let (val, msg) = OptionWarnStyle.apply("banner").unwrap();
    assert!(!val);
    assert_eq!(msg, "Warning style set to banner");
    assert!(OptionWarnStyle.apply("loud").is_err());
}

#[test]
fn elem_id_is_in_the_kuna_range() {
    assert_eq!(ELEM_WARNSTYLE.get_id(), 4117);
    assert_eq!(OptionWarnStyle::NAME, "warnstyle");
}

#[test]
fn slug_table_covers_the_known_warnings() {
    assert_eq!(warning_slug("WARNING: Subroutine does not return"), "no-return");
    assert_eq!(warning_slug("WARNING: Does not return"), "no-return");
    assert_eq!(warning_slug("WARNING: Treating indirect jump as call"), "jump-as-call");
    assert_eq!(warning_slug("WARNING: Treating indirect jump as return"), "jump-as-return");
    assert_eq!(warning_slug("WARNING: Could not inline here"), "inline-failed");
    assert_eq!(
        warning_slug("WARNING: Read-only address (ram,0x100) is written"),
        "writes-rodata"
    );
    assert_eq!(
        warning_slug(
            "WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped)"
        ),
        "branch-flip"
    );
    assert_eq!(
        warning_slug("WARNING: tailcalljump: recovered tail call -> introduced call to 0x1640"),
        "tail-call"
    );
    assert_eq!(
        warning_slug("WARNING: taildup: duplicated return-call tail to remove goto"),
        "return-dupe"
    );
    assert_eq!(
        warning_slug("WARNING: crossjumprevert: duplicated cross-jump tail to remove goto"),
        "crossjump-dupe"
    );
    // Header warnings keep their count as an xN suffix (elided when 1).
    assert_eq!(
        warning_slug("WARNING: earlyreturn: hoisted 3 const-guard early-return(s) in f"),
        "early-return x3"
    );
    assert_eq!(
        warning_slug("WARNING: earlyreturn: hoisted 1 const-guard early-return(s) in f"),
        "early-return"
    );
    assert_eq!(
        warning_slug("WARNING: returndup: duplicated 2 shared bare-epilogue return(s) for early-return recovery in f"),
        "return-dupe x2"
    );
    assert_eq!(warning_slug("WARNING: Inlined function: add50"), "inlined: add50");
    // The jumptable prefix survives as a marker; unknown text keeps full info.
    assert_eq!(
        warning_slug("WARNING (jumptable): Treating indirect jump as call"),
        "jt: jump-as-call"
    );
    assert_eq!(
        warning_slug("WARNING: something novel happened"),
        "warn: something novel happened"
    );
}
