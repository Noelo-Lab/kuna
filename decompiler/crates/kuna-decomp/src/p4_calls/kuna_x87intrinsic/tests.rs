use super::*;

#[test]
fn the_prototype_parses_with_its_name() {
    let mut name = None;
    decode_prototype(|root| {
        name = Some(String::from_utf8_lossy(root.get_attribute_value("name")?).into_owned());
        Ok(())
    })
    .expect("prototype_xml must parse");
    assert_eq!(name.as_deref(), Some(MODEL_NAME));
}

#[test]
fn st0_is_the_input_and_the_output() {
    let xml = prototype_xml();
    assert_eq!(xml.matches(r#"<register name="ST0"/>"#).count(), 3, "input, output and killedbycall");
    assert!(xml.contains(r#"extrapop="4""#));
}

#[test]
fn intrinsic_names_match_with_any_underscores() {
    assert!(is_intrinsic("__CIsin"));
    assert!(is_intrinsic("_CIsqrt"));
    assert!(is_intrinsic("CIlog10"));
    assert!(!is_intrinsic("__CIpow"));
    assert!(!is_intrinsic("sin"));
}
