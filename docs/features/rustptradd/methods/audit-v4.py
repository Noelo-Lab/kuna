from common import *
import shutil
old=w/'ast-audit-v3';new=w/'ast-audit-v4';(new/'src').mkdir(parents=True,exist_ok=True)
(new/'Cargo.toml').write_text((old/'Cargo.toml').read_text().replace('kuna-rust-pointer-audit-v3','kuna-rust-pointer-audit-v4'));shutil.copy2(old/'Cargo.lock',new/'Cargo.lock');s=(old/'src/main.rs').read_text()
s=s.replace('struct NewPointerCasts{existing:std::collections::BTreeSet<String>,count:usize}', '''#[derive(Default)]struct StatementCasts{scopes:Vec<std::collections::BTreeSet<String>>}
impl VisitMut for StatementCasts{fn visit_stmt_mut(&mut self,stmt:&mut syn::Stmt){
 let mut copy=stmt.clone();let mut collector=ExistingPointerCasts::default();collector.visit_stmt_mut(&mut copy);self.scopes.push(collector.casts);
 visit_mut::visit_stmt_mut(self,stmt);
}}
struct NewPointerCasts{existing:std::collections::BTreeSet<String>,count:usize,scopes:Vec<std::collections::BTreeSet<String>>,statement:usize}
''')
s=s.replace('impl VisitMut for NewPointerCasts{fn visit_expr_mut', '''impl VisitMut for NewPointerCasts{
 fn visit_stmt_mut(&mut self,stmt:&mut syn::Stmt){
  let next=self.scopes.get(self.statement).cloned().unwrap_or_default();self.statement+=1;
  let previous=std::mem::replace(&mut self.existing,next);visit_mut::visit_stmt_mut(self,stmt);self.existing=previous;
 }
 fn visit_expr_mut''')
s=s.replace('let mut casts=NewPointerCasts{existing:existing.casts,count:0};','let mut statements=StatementCasts::default();statements.visit_file_mut(&mut before);let expected_statements=statements.scopes.len();\n let mut casts=NewPointerCasts{existing:existing.casts,count:0,scopes:statements.scopes,statement:0};').replace('(transforms.count>0&&format!("{before:?}")==format!("{after:?}")).then_some(transforms)\n}\n\nfn parse', '(transforms.count>0&&casts.statement==expected_statements&&format!("{before:?}")==format!("{after:?}")).then_some(transforms)\n}\n\nfn parse')
s=s.replace(' let cases=[',''' let cases=[
 ("a=&mut p[i];b=&mut p[i] as *mut u8","a=p.wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut u8;b=p.wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut u8",true),
 ("a=&mut p[i];b=&mut p[i] as *mut i32","a=p.wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut u8;b=p.wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut u8",false),''')
(new/'src/main.rs').write_text(s)
run(r,w,['cargo','build','--release','--manifest-path',str(new/'Cargo.toml')],'audit-v4-build.log')
run(r,w,[str(new/'target/release/kuna-rust-pointer-audit-v4')],'audit-v4-selfchecks.log')
run(r,w,[str(new/'target/release/kuna-rust-pointer-audit-v4'),str(w/'breadth/record.json'),str(w/'breadth/pointer-review-v4.json')],'audit-full-v4.log')
