fn plain_value(e:Expr)->String {
    let e=unparen(e);
    assert!(matches!(e,Expr::Path(_)|Expr::Lit(_)),"new string input expression needs review: {}",text(&e));
    text(&e)
}
fn parameter(source:&str)->serde_json::Value {
    let expr=syn::parse_str::<Expr>(source).unwrap();
    if let Expr::Call(outer)=&expr {
        assert!(outer.attrs.is_empty());assert_eq!(text(&outer.func),"std :: str :: from_utf8_unchecked");assert_eq!(outer.args.len(),1);
        let Expr::Call(inner)=&outer.args[0] else {panic!("missing slice constructor")};
        assert!(inner.attrs.is_empty());assert_eq!(text(&inner.func),"std :: slice :: from_raw_parts");assert_eq!(inner.args.len(),2);
        let Expr::Cast(data)=unparen(inner.args[0].clone()) else {panic!("no byte pointer cast")};
        assert!(data.attrs.is_empty());assert_eq!(text(&data.ty),"* const u8");
        let data=match unparen(*data.expr) {
            Expr::Cast(bridge)=>{assert!(bridge.attrs.is_empty());assert_eq!(text(&bridge.ty),"* const _");plain_value(*bridge.expr)}
            e=>plain_value(e),
        };
        let Expr::Cast(length)=unparen(inner.args[1].clone()) else {panic!("no usize cast")};
        assert!(length.attrs.is_empty());assert_eq!(text(&length.ty),"usize");
        return serde_json::json!({"kind":"string","data":data,"length":plain_value(*length.expr)});
    }
    let Expr::Reference(reference)=expr else {panic!("not borrowed primitive")};
    assert!(reference.mutability.is_none() && reference.attrs.is_empty());
    let Expr::Unary(read)=*reference.expr else {panic!("not dereference")};
    assert!(matches!(read.op,syn::UnOp::Deref(_)) && read.attrs.is_empty());
    let Expr::Cast(typed)=unparen(*read.expr) else {panic!("no typed read")};
    let syn::Type::Ptr(ptr)=typed.ty.as_ref() else {panic!("not pointer")};
    let primitive=text(&ptr.elem);assert!(ptr.mutability.is_none() && matches!(primitive.as_str(),"u8"|"i8"|"u16"|"i16"|"u32"|"i32"|"u64"|"i64"));
    let Expr::Cast(bridge)=unparen(*typed.expr) else {panic!("no pointer bridge")};
    assert_eq!(text(&bridge.ty),"* const _");
    serde_json::json!({"kind":"primitive","primitive":primitive,"pointer":plain_value(*bridge.expr)})
}
