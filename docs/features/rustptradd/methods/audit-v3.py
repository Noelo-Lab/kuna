from common import *
import shutil
old=w/'ast-audit-v2';new=w/'ast-audit-v3';(new/'src').mkdir(parents=True,exist_ok=True)
(new/'Cargo.toml').write_text((old/'Cargo.toml').read_text().replace('kuna-rust-pointer-audit-v2','kuna-rust-pointer-audit-v3'));shutil.copy2(old/'Cargo.lock',new/'Cargo.lock');s=(old/'src/main.rs').read_text()
start=s.index('fn compare(before:');end=s.index('\nfn parse(source:',start)
s=s[:start]+'''
#[derive(Default)]struct ExistingPointerCasts{casts:std::collections::BTreeSet<String>}
impl VisitMut for ExistingPointerCasts{fn visit_expr_mut(&mut self,e:&mut Expr){
 visit_mut::visit_expr_mut(self,e);
 if matches!(e,Expr::Cast(c) if matches!(&*c.ty,Type::Ptr(_))){self.casts.insert(format!("{e:?}"));}
}}
struct NewPointerCasts{existing:std::collections::BTreeSet<String>,count:usize}
impl VisitMut for NewPointerCasts{fn visit_expr_mut(&mut self,e:&mut Expr){
 let before=self.count;visit_mut::visit_expr_mut(self,e);
 let removable=matches!(e,Expr::Cast(c) if matches!(&*c.ty,Type::Ptr(t) if t.mutability.is_some())&&matches!(&*c.expr,Expr::Reference(r) if r.mutability.is_some()&&matches!(&*r.expr,Expr::Index(_))));
 if removable&&!self.existing.contains(&format!("{e:?}")){
  if let Expr::Cast(c)=e{*e=*c.expr.clone();self.count+=1;}
 }
 if self.count>before{
  if let Expr::Unary(u)=e{if matches!(u.op,syn::UnOp::Deref(_)){if let Expr::Reference(r)=&*u.expr{if r.mutability.is_some()&&matches!(&*r.expr,Expr::Index(_)){*e=*r.expr.clone();}}}}
 }
}}
fn compare(before:&syn::File,after:&syn::File)->Option<PointerIndices>{
 if let Some(result)=compare_inner(before,after,false){return Some(result);}
 let mut before=before.clone();let mut after=after.clone();Parentheses.visit_file_mut(&mut before);Parentheses.visit_file_mut(&mut after);
 let mut transforms=PointerIndices::default();transforms.visit_file_mut(&mut after);
 let mut existing=ExistingPointerCasts::default();existing.visit_file_mut(&mut before);
 let mut casts=NewPointerCasts{existing:existing.casts,count:0};casts.visit_file_mut(&mut after);transforms.pointer_casts=casts.count;
 (transforms.count>0&&format!("{before:?}")==format!("{after:?}")).then_some(transforms)
}
''' +s[end:]
s=s.replace(' let cases=[',''' let cases=[
 ("consume(&mut p[i] as *mut u8,&mut q[j])","consume(p.wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut u8,q.wrapping_byte_offset((j as isize).wrapping_mul(8)) as *mut u64)",true),
 ("consume(*(&mut p[i] as *mut u8),&mut q[j])","consume(*(p.wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut u8),q.wrapping_byte_offset((j as isize).wrapping_mul(8)) as *mut u64)",true),
 ("consume(&mut p[i] as *mut u8,&mut q[j])","consume(p.wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32,q.wrapping_byte_offset((j as isize).wrapping_mul(8)) as *mut u64)",false),
 ("consume(&mut p[i] as *mut u8,&mut p[i])","consume(p.wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut u8,p.wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut u8)",false),''')
(new/'src/main.rs').write_text(s)
run(r,w,['cargo','build','--release','--manifest-path',str(new/'Cargo.toml')],'audit-v3-build.log')
run(r,w,[str(new/'target/release/kuna-rust-pointer-audit-v3')],'audit-v3-selfchecks.log')
for name in ['rust','namespace-rust','rust2025','pieces-1.90.0']:
 p=w/'breadth'/(name+'-record.json');temp=w/(name+'-v3-input.json');temp.write_text(json.dumps({'records':[json.loads(p.read_text())]})+'\n');run(r,w,[str(new/'target/release/kuna-rust-pointer-audit-v3'),str(temp),str(w/(name+'-v3-review.json'))],name+'-v3-review.log');print(name,json.loads((w/(name+'-v3-review.json')).read_text())['pending'],'pending',flush=True)
