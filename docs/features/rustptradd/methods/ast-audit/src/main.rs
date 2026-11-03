use serde_json::{json,Value};
use syn::{Expr,Type};
use syn::visit_mut::{self,VisitMut};
fn normalize_tokens(stream: proc_macro2::TokenStream, erase_integer_casts: bool) -> proc_macro2::TokenStream {
    use proc_macro2::{Group, TokenTree};
    let tokens: Vec<_> = stream.into_iter().collect();
    let mut result = proc_macro2::TokenStream::new(); let mut index = 0;
    while index < tokens.len() {
        if erase_integer_casts && matches!(&tokens[index], TokenTree::Ident(id) if id == "as") && index + 1 < tokens.len()
            && matches!(&tokens[index+1], TokenTree::Ident(id) if matches!(id.to_string().as_str(), "i8"|"u8"|"i16"|"u16"|"i32"|"u32"|"i64"|"u64"))
        { index += 2; continue; }
        let token = match &tokens[index] {
            TokenTree::Group(group) => TokenTree::Group(Group::new(group.delimiter(), normalize_tokens(group.stream(), erase_integer_casts))),
            TokenTree::Ident(id) if id == "self" => TokenTree::Ident(proc_macro2::Ident::new("__audit_self", id.span())),
            TokenTree::Ident(id) if id == "fn" && (matches!(index.checked_sub(1).and_then(|n| tokens.get(n)), Some(TokenTree::Punct(p)) if p.as_char()=='.') || !matches!(tokens.get(index+1), Some(TokenTree::Ident(next)) if next != "as")) => TokenTree::Ident(proc_macro2::Ident::new("__audit_fn", id.span())),
            TokenTree::Ident(id) if id == "_" && matches!(tokens.get(index+1), Some(TokenTree::Punct(p)) if p.as_char()==':') => TokenTree::Ident(proc_macro2::Ident::new("__audit_underscore", id.span())),
            other => other.clone(),
        };
        result.extend([token]); index += 1;
    }
    result
}

#[derive(Default)]struct Parentheses;
impl VisitMut for Parentheses{fn visit_expr_mut(&mut self,e:&mut Expr){while let Expr::Paren(p)=e{*e=*p.expr.clone();}visit_mut::visit_expr_mut(self,e);}}
fn int_type(t:&Type)->bool{matches!(t,Type::Path(p) if p.qself.is_none()&&p.path.leading_colon.is_none()&&p.path.segments.len()==1&&matches!(p.path.segments[0].ident.to_string().as_str(),"i32"|"u32"|"i64"|"u64"))}
fn isize_type(t:&Type)->bool{matches!(t,Type::Path(p) if p.qself.is_none()&&p.path.leading_colon.is_none()&&p.path.is_ident("isize"))}
fn literal(e:&Expr)->bool{matches!(e,Expr::Lit(l) if matches!(l.lit,syn::Lit::Int(_)))||matches!(e,Expr::Unary(u) if matches!(u.op,syn::UnOp::Neg(_))&&literal(&u.expr))}
fn strip_context(e:&Expr)->Expr{
 if let Expr::Cast(c)=e{if int_type(&c.ty){
  if literal(&c.expr){return *c.expr.clone();}
  if let Expr::Call(call)=&*c.expr{if call.args.len()==1{if let Expr::Path(p)=&*call.func{
   let parts:Vec<_>=p.path.segments.iter().map(|s|s.ident.to_string()).collect();
   if p.qself.is_none()&&p.path.leading_colon.is_some()&&parts==["core","convert","identity"]{
    if let syn::PathArguments::AngleBracketed(args)=&p.path.segments[2].arguments{if args.args.len()==1{if let syn::GenericArgument::Type(t)=&args.args[0]{if format!("{t:?}")==format!("{:?}",c.ty){return call.args[0].clone();}}}}
   }
  }}}
 }}
 e.clone()
}
fn pointer(e:&Expr)->Option<(Expr,Expr,String)>{
 let Expr::MethodCall(call)=e else{return None;};
 if call.method!="wrapping_byte_offset"||call.turbofish.is_some()||call.args.len()!=1{return None;}
 let Expr::MethodCall(mul)=&call.args[0]else{return None;};
 if mul.method!="wrapping_mul"||mul.turbofish.is_some()||mul.args.len()!=1||!literal(&mul.args[0]){return None;}
 let Expr::Cast(index)=&*mul.receiver else{return None;};if !isize_type(&index.ty){return None;}
 Some((*call.receiver.clone(),strip_context(&index.expr),format!("{:?}",mul.args[0])))
}
#[derive(Default)]struct PointerIndices{count:usize,scales:Vec<String>}
impl VisitMut for PointerIndices{fn visit_expr_mut(&mut self,e:&mut Expr){
 let direct=if let Expr::Unary(u)=e{if matches!(u.op,syn::UnOp::Deref(_)){pointer(&u.expr)}else{None}}else{None};
 let replacement=if let Some((base,index,scale))=direct{Some((base,index,scale,true))}else{pointer(e).map(|(b,i,s)|(b,i,s,false))};
 if let Some((base,index,scale,value))=replacement{
  self.count+=1;self.scales.push(scale);
  let indexed=Expr::Index(syn::ExprIndex{attrs:Vec::new(),expr:Box::new(base),bracket_token:Default::default(),index:Box::new(index)});
  *e=if value{indexed}else{Expr::Reference(syn::ExprReference{attrs:Vec::new(),and_token:Default::default(),mutability:Some(Default::default()),expr:Box::new(indexed)})};
 }
 visit_mut::visit_expr_mut(self,e);
}}
fn compare(before:&syn::File,after:&syn::File)->Option<PointerIndices>{
 let mut before=before.clone();let mut after=after.clone();Parentheses.visit_file_mut(&mut before);Parentheses.visit_file_mut(&mut after);
 let mut transforms=PointerIndices::default();transforms.visit_file_mut(&mut after);
 (transforms.count>0&&format!("{before:?}")==format!("{after:?}")).then_some(transforms)
}
fn parse(source:&str)->Result<syn::File,String>{source.parse().map_err(|e:proc_macro2::LexError|e.to_string()).and_then(|t|syn::parse2(normalize_tokens(t,false)).map_err(|e|e.to_string()))}
fn main(){
 let cases=[
 ("p[i]","*p.wrapping_byte_offset((i as isize).wrapping_mul(4))",true),
 ("&mut p[i]","p.wrapping_byte_offset((i as isize).wrapping_mul(4))",true),
 ("p[i].field","(*p.wrapping_byte_offset((i as isize).wrapping_mul(16))).field",true),
 ("base()[index()]","*base().wrapping_byte_offset((index() as isize).wrapping_mul(8))",true),
 ("&mut p[0xffffffffffffffff >> n]","p.wrapping_byte_offset(((::core::convert::identity::<u64>(0xffffffffffffffff >> n) as u64) as isize).wrapping_mul(4))",true),
 ("&mut p[5]","p.wrapping_byte_offset(((5 as i64) as isize).wrapping_mul(4))",true),
 ("p[i]","*q.wrapping_byte_offset((i as isize).wrapping_mul(4))",false),
 ("p[i]","*p.wrapping_byte_offset((j as isize).wrapping_mul(4))",false),
 ("p[i]","p.wrapping_byte_offset((i as isize).wrapping_mul(4))",false),
 ("p[i]","*p.byte_offset((i as isize).wrapping_mul(4))",false),
 ("p[i]","*p.wrapping_byte_offset((i as isize)*4)",false),
 ("p[i]","*p.wrapping_byte_offset(i.wrapping_mul(4))",false),
 ("p[i]","*p.wrapping_byte_offset(((i as i32) as isize).wrapping_mul(4))",false),
 ("p[i]+callback()","*p.wrapping_byte_offset((i as isize).wrapping_mul(4))+callback()+callback()",false)
 ];
 for(old,new,expected)in &cases{let before=syn::parse_file(&format!("fn test(){{{old};}}")).unwrap();let after=syn::parse_file(&format!("fn test(){{{new};}}")).unwrap();assert_eq!(compare(&before,&after).is_some(),*expected,"{old} => {new}");}
 let args:Vec<String>=std::env::args().collect();if args.len()==1{println!("{} audit self-checks passed",cases.len());return;}
 let input:Value=serde_json::from_slice(&std::fs::read(&args[1]).unwrap()).unwrap();let mut output=Vec::new();
 for row in input["records"].as_array().unwrap(){for change in row["changes"].as_array().unwrap(){
  let before=change["parent"]["code"].as_str().unwrap();let after=change["candidate"]["code"].as_str().unwrap();
  let raw_before=syn::parse_file(before);let raw_after=syn::parse_file(after);let normalized=raw_before.is_err()||raw_after.is_err();
  let before=if normalized{parse(before)}else{raw_before.clone().map_err(|e|e.to_string())};let after=if normalized{parse(after)}else{raw_after.clone().map_err(|e|e.to_string())};
  let transforms=match(&before,&after){(Ok(a),Ok(b))=>compare(a,b),_=>None};
  output.push(json!({"fixture":row["fixture"],"language":change["language"],"gopairreturns":change["gopairreturns"],"address":change["address"],"name":change["name"],"raw_parent_parse_error":raw_before.err().map(|e|e.to_string()),"raw_candidate_parse_error":raw_after.err().map(|e|e.to_string()),"identifier_normalization_only":normalized,"parent_parse_error":before.err(),"candidate_parse_error":after.err(),"classified":transforms.is_some(),"index_transformations":transforms.as_ref().map(|t|t.count),"instruction_scale_expressions":transforms.map(|t|t.scales)}));
 }}
 let pending=output.iter().filter(|v|v["classified"]!=true).count();let count=output.len();std::fs::write(&args[2],serde_json::to_string_pretty(&json!({"scope":"Exact AST reconstruction of pointer index/address expressions; base, index, dereference and surrounding AST retained. Instruction scale is recorded but cannot be inferred from the old AST alone. This is not whole-program native equivalence.","self_checks":cases.len(),"changed_records":count,"pending":pending,"records":output})).unwrap()+"\n").unwrap();println!("{count} records, {pending} pending");
}
