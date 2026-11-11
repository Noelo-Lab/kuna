from common import *
import shutil
old=w/'ast-audit';new=w/'ast-audit-v2';(new/'src').mkdir(parents=True,exist_ok=True)
(new/'Cargo.toml').write_text((old/'Cargo.toml').read_text().replace('kuna-rust-pointer-audit','kuna-rust-pointer-audit-v2'));shutil.copy2(old/'Cargo.lock',new/'Cargo.lock')
s=(old/'src/main.rs').read_text()
needle='            TokenTree::Ident(id) if id == "self"'
s=s.replace(needle,'            TokenTree::Ident(id) if id == "unsafe" && !matches!(tokens.get(index+1), Some(TokenTree::Ident(next)) if next == "fn" || next == "extern" || next == "trait" || next == "impl") && !matches!(tokens.get(index+1), Some(TokenTree::Group(g)) if g.delimiter() == proc_macro2::Delimiter::Brace) => TokenTree::Ident(proc_macro2::Ident::new("__audit_unsafe", id.span())),\n'+needle)
s=s.replace('#[derive(Default)]struct PointerIndices{count:usize,scales:Vec<String>}','''fn pointer_form(e:&Expr,allow_cast:bool)->Option<(Expr,Expr,String,bool)>{
 if let Some((b,i,s))=pointer(e){return Some((b,i,s,false));}
 if allow_cast{if let Expr::Cast(c)=e{if matches!(&*c.ty,Type::Ptr(t) if t.mutability.is_some()){return pointer(&c.expr).map(|(b,i,s)|(b,i,s,true));}}}
 None
}
#[derive(Default)]struct PointerIndices{count:usize,scales:Vec<String>,allow_cast:bool,pointer_casts:usize}''')
s=s.replace('pointer(&u.expr)}else{None}', 'pointer_form(&u.expr,self.allow_cast)}else{None}').replace('if let Some((base,index,scale))=direct{Some((base,index,scale,true))}else{pointer(e).map(|(b,i,s)|(b,i,s,false))}', 'if let Some((base,index,scale,cast))=direct{Some((base,index,scale,true,cast))}else{pointer_form(e,self.allow_cast).map(|(b,i,s,c)|(b,i,s,false,c))}').replace('if let Some((base,index,scale,value))=replacement{','if let Some((base,index,scale,value,cast))=replacement{\n  self.pointer_casts+=usize::from(cast);')
s=s.replace('fn compare(before:&syn::File,after:&syn::File)->Option<PointerIndices>{','fn compare_inner(before:&syn::File,after:&syn::File,allow_cast:bool)->Option<PointerIndices>{').replace('let mut transforms=PointerIndices::default();','let mut transforms=PointerIndices{allow_cast,..Default::default()};')
s=s.replace('fn parse(source:&str)', 'fn compare(before:&syn::File,after:&syn::File)->Option<PointerIndices>{compare_inner(before,after,false).or_else(||compare_inner(before,after,true))}\nfn parse(source:&str)')
s=s.replace(' let cases=[',''' let keyword_old=parse("unsafe fn shadow(mut unsafe:*mut i32,i:i64)->i32{return unsafe[i];}").unwrap();
 let keyword_new=parse("unsafe fn shadow(mut unsafe:*mut i32,i:i64)->i32{return *unsafe.wrapping_byte_offset((i as isize).wrapping_mul(4));}").unwrap();assert!(compare(&keyword_old,&keyword_new).is_some());
 let cases=[
 ("&mut p[i]","p.wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut u64",true),
 ("p[i]","*(p.wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut u64)",true),
 ("&mut p[i] as *mut u8","(p.wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut u64) as *mut u8",true),
 ("&mut p[i] as *mut i32","p.wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut u64",false),
 ("p[i] as u8","*p.wrapping_byte_offset((i as isize).wrapping_mul(8))",false),''')
s=s.replace('"instruction_scale_expressions":transforms.map(|t|t.scales)','"pointer_result_casts":transforms.as_ref().map(|t|t.pointer_casts),"instruction_scale_expressions":transforms.map(|t|t.scales)').replace('"self_checks":cases.len()', '"self_checks":cases.len()+1').replace('cases.len());return;', 'cases.len()+1);return;')
s=s.replace('base, index, dereference and surrounding AST retained.', 'base, index, dereference and surrounding AST retained; newly explicit pointer-result casts are recorded separately.')
(new/'src/main.rs').write_text(s)
run(r,w,['cargo','build','--release','--manifest-path',str(new/'Cargo.toml')],'audit-v2-build.log')
run(r,w,[str(new/'target/release/kuna-rust-pointer-audit-v2')],'audit-v2-selfchecks.log')
for name in ['grep','native-c']:
 p=w/'breadth'/(name+'-record.json');temp=w/(name+'-v2-input.json');temp.write_text(json.dumps({'records':[json.loads(p.read_text())]})+'\n');run(r,w,[str(new/'target/release/kuna-rust-pointer-audit-v2'),str(temp),str(w/(name+'-v2-review.json'))],name+'-v2-review.log');print(name,json.loads((w/(name+'-v2-review.json')).read_text())['pending'],'pending')
