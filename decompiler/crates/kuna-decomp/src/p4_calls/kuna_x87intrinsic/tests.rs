use super::*;

#[test]
fn both_prototypes_parse_with_their_names() {
    let mut names = Vec::new();
    decode_prototype(|root| {
        names.push(String::from_utf8_lossy(root.get_attribute_value("name")?).into_owned());
        Ok(())
    })
    .expect("prototypes must parse");
    assert_eq!(names, vec![MODEL_NAME.to_string(), BINARY_MODEL_NAME.to_string()]);
}

#[test]
fn st0_is_the_unary_input_and_output() {
    let xml = prototype_xml();
    assert_eq!(xml.matches(r#"<register name="ST0"/>"#).count(), 3, "input, output and killedbycall");
    assert!(xml.contains(r#"extrapop="4""#));
}

#[test]
fn the_binary_model_takes_st1_before_st0() {
    let xml = binary_prototype_xml();
    let input = &xml[xml.find("<input>").unwrap()..xml.find("</input>").unwrap()];
    assert!(input.find("ST1").unwrap() < input.find("ST0").unwrap());
    assert!(xml[xml.find("<killedbycall>").unwrap()..].contains("ST1"), "the popped slot is killed");
}

#[test]
fn intrinsic_names_choose_their_model() {
    assert_eq!(model_for("__CIsin").map(|m| m.0), Some(MODEL_NAME));
    assert_eq!(model_for("_CIsqrt").map(|m| m.0), Some(MODEL_NAME));
    assert_eq!(model_for("CIlog10").map(|m| m.0), Some(MODEL_NAME));
    assert_eq!(model_for("__CIatan2"), Some((BINARY_MODEL_NAME, &["y", "x"][..])));
    assert_eq!(model_for("__CIpow").map(|m| m.0), Some(BINARY_MODEL_NAME));
    assert!(model_for("sin").is_none());
    assert!(model_for("atan2").is_none());
}
