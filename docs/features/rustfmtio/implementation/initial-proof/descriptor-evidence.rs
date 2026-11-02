use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute,register_decomp_commands,IfaceDecompData,DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::{context::{VarnodeId,OpId},funcdata::Funcdata};
use kuna_num::opcodes::OpCode;
use serde_json::{json,Value};
use std::collections::BTreeMap;

fn constant(fd:&Funcdata,mut id:VarnodeId)->Option<u64>{
 for _ in 0..8 {
  let value=fd.vbank().get(id)?;if value.get_size()!=8{return None;}
  if value.is_constant(){return Some(value.get_offset());}
  let op=fd.obank().get(value.get_def()?)?;
  match op.code(){
   OpCode::CPUI_COPY|OpCode::CPUI_CAST=>id=op.get_in(0)?,
   OpCode::CPUI_PTRSUB=>{let base=fd.vbank().get(op.get_in(0)?)?;if !base.is_constant()||base.get_offset()!=0{return None;}id=op.get_in(1)?;}
   _=>return None,
  }
 }
 None
}
fn stack(fd:&Funcdata,mut id:VarnodeId)->Option<i64>{
 let base=fd.get_arch().manage().get_stack_space()?.get_spacebase(0).ok()?;let mut offset=0i64;
 for _ in 0..8 {
  let value=fd.vbank().get(id)?;if value.get_size()!=8{return None;}
  let Some(def)=value.get_def() else {return (value.is_input()&&value.get_space().get_index()==base.space.as_ref()?.get_index()&&value.get_offset()==base.offset).then_some(offset);};
  let op=fd.obank().get(def)?;
  match op.code(){
   OpCode::CPUI_COPY|OpCode::CPUI_CAST=>id=op.get_in(0)?,
   OpCode::CPUI_PTRSUB|OpCode::CPUI_INT_ADD|OpCode::CPUI_INT_SUB=>{
    let literal=fd.vbank().get(op.get_in(1)?)?;if !literal.is_constant(){return None;}
    offset=if op.code()==OpCode::CPUI_INT_SUB{offset.checked_sub(literal.get_offset() as i64)?}else{offset.checked_add(literal.get_offset() as i64)?};id=op.get_in(0)?;
   }
   _=>return None,
  }
 }
 None
}
fn word(fd:&Funcdata,id:VarnodeId)->Option<Value>{
 let value=fd.vbank().get(id)?;
 Some(json!({"space":value.get_space().get_name(),"offset":value.get_offset(),"size":value.get_size(),"constant":constant(fd,id),"definition":value.get_def().and_then(|id|fd.obank().get(id)).map(|o|format!("{:?}",o.code()))}))
}
fn proof(fd:&Funcdata,id:OpId,count:usize)->Option<Value>{
 let op=fd.obank().get(id)?;let first=2;
 let template=constant(fd,op.get_in(first)?)?;
 if let Some(tag)=constant(fd,op.get_in(first+1)?){
  if tag&1==1&&count==0{return Some(json!({"template":template,"literal_tag":tag,"argument_count":0,"arguments":[]}));}
 }
 let array=stack(fd,op.get_in(first+1)?)?;let space=fd.get_arch().manage().get_stack_space()?;let mut words=BTreeMap::new();
 for indirect in fd.obank().iter_alive(){
  let effect=fd.obank().get(indirect)?;if effect.code()!=OpCode::CPUI_INDIRECT||effect.is_indirect_creation(){continue;}
  let referent=fd.vbank().get(effect.get_in(1)?)?;
  if OpId::from(slotmap::KeyData::from_ffi(referent.get_offset()))!=id{continue;}
  let out=fd.vbank().get(effect.get_out()?)?;
  if out.get_space().get_index()==space.get_index()&&out.get_size()==8{
   if words.insert(out.get_offset() as i64,effect.get_in(0)?).is_some(){return None;}
  }
 }
 if count>16{return None;}
 let mut arguments=Vec::new();
 for i in 0..count as i64{
  let pointer=stack(fd,*words.get(&(array+i*16))?)?;
  arguments.push(json!({"pointer_stack_offset":pointer,"formatter":constant(fd,*words.get(&(array+i*16+8))?)?,"data_word":words.get(&pointer).and_then(|&v|word(fd,v)),"length_word":words.get(&(pointer+8)).and_then(|&v|word(fd,v))}));
 }
 Some(json!({"template":template,"array_stack_offset":array,"argument_count":count,"arguments":arguments}))
}
fn main(){
 let args:Vec<_>=std::env::args().collect();
 let mut p=bootstrap_from_object(&args[1],"",&["/home/mahaloz/kwt/angr-rust-io-proposal/specs".into()]).unwrap();p.arch_mut().analysis_listing = true;p.commit_pending_analysis().unwrap();
 let mut s=ConsoleCommands::into_status(vec!["option setlanguage rust-language".into(),format!("load addr {}",args[2]),"decompile".into()]);register_decomp_commands(&mut s);
 s.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut().downcast_mut::<IfaceDecompData>().unwrap().conf=Some(p);
 for _ in 0..3{execute(&mut s);assert!(!s.inerror,"{}",s.optr);}
 let d=s.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();let fd=d.fd.as_ref().unwrap();
 for i in 0..fd.num_calls(){let call=fd.get_call_specs(i);
  let op=fd.obank().get(call.get_op()).unwrap();if op.num_input()!=4{continue;}let output=op.get_out().and_then(|id|fd.vbank().get(id));
  let result_type=output.map(|v|match v.get_type_def_facing().get_metatype(){
   kuna_decomp::dtype::type_metatype::TYPE_BOOL=>"bool",
   kuna_decomp::dtype::type_metatype::TYPE_INT=>"i64",
   kuna_decomp::dtype::type_metatype::TYPE_UINT|kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN=>"u64",
   _=>"unsupported",
  });
  println!("{}",json!({"call_instruction":op.get_addr().get_offset(),"name":call.get_name(),"callee":call.get_entry_address().get_offset(),"runtime_argument_count":op.num_input()-1,"writer":op.get_in(1).and_then(|id|word(fd,id)),"result_size":output.map(|v|v.get_size()),"result_type":result_type,"descriptor":proof(fd,call.get_op(),args[3].parse().unwrap())}));
 }
}
