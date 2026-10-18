//! Constant DWARF subranges use both bounds and the compilation unit's origin.
use super::Reader;

fn integer(value: gimli::AttributeValue<Reader<'_>>) -> Option<i128> {
    match value {
        gimli::AttributeValue::Sdata(value) => Some(i128::from(value)),
        value => value.udata_value().map(i128::from),
    }
}

pub(super) fn array_count(
    count: Option<gimli::AttributeValue<Reader<'_>>>,
    lower: Option<gimli::AttributeValue<Reader<'_>>>,
    upper: Option<gimli::AttributeValue<Reader<'_>>>,
    language: Option<gimli::DwLang>,
) -> Option<u64> {
    if let Some(count) = count {
        return integer(count).and_then(|value| u64::try_from(value).ok());
    }
    let lower = match lower {
        Some(value) => integer(value)?,
        None => match language {
            Some(gimli::DW_LANG_Fortran18 | gimli::DW_LANG_Ada2005 | gimli::DW_LANG_Ada2012) => 1,
            Some(value) => value.default_lower_bound().unwrap_or(0) as i128,
            None => 0,
        },
    };
    let upper = integer(upper?)?;
    u64::try_from(upper - lower + 1).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gimli::AttributeValue::{Sdata, Udata};

    #[test]
    fn origins_signed_bounds_and_explicit_counts() {
        assert_eq!(array_count(None, None, Some(Udata(3)), Some(gimli::DW_LANG_Fortran90)), Some(3));
        for language in [gimli::DW_LANG_Fortran18, gimli::DW_LANG_Ada95,
                         gimli::DW_LANG_Ada2005, gimli::DW_LANG_Ada2012] {
            assert_eq!(array_count(None, None, Some(Udata(3)), Some(language)), Some(3));
        }
        for language in [gimli::DW_LANG_C, gimli::DW_LANG_C17,
                         gimli::DW_LANG_C_plus_plus_17, gimli::DW_LANG_C_plus_plus_20] {
            assert_eq!(array_count(None, None, Some(Udata(3)), Some(language)), Some(4));
        }
        let dynamic = gimli::AttributeValue::UnitRef(gimli::UnitOffset(1));
        assert_eq!(array_count(Some(dynamic.clone()), None, Some(Udata(3)), None), None);
        assert_eq!(array_count(None, Some(dynamic), Some(Udata(3)), None), None);
        assert_eq!(array_count(None, Some(Sdata(-2)), Some(Sdata(2)), None), Some(5));
        assert_eq!(array_count(None, Some(Sdata(-8)), Some(Sdata(-4)), None), Some(5));
        assert_eq!(array_count(Some(Udata(7)), Some(Sdata(-2)), Some(Sdata(2)), None), Some(7));
        assert_eq!(array_count(None, Some(Udata(4)), Some(Udata(3)), None), Some(0));
        assert_eq!(array_count(None, None, Some(Udata(u64::MAX)), None), None);
        assert_eq!(array_count(None, Some(Udata(8)), Some(Udata(3)), None), None);
    }
}
