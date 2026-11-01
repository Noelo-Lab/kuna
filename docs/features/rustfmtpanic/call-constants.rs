use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute,register_decomp_commands,IfaceDecompData,DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::{context::VarnodeId,funcdata::Funcdata};
use kuna_num::opcodes::OpCode;

fn constant(fd:&Funcdata,mut id:VarnodeId)->Option<u64>{
    for _ in 0..8 {
        let value=fd.vbank().get(id)?;
        if value.is_constant(){return Some(value.get_offset());}
        let op=fd.obank().get(value.get_def()?)?;
        match op.code(){
            OpCode::CPUI_COPY|OpCode::CPUI_CAST=>id=op.get_in(0)?,
            OpCode::CPUI_PTRSUB=>{
                let base=fd.vbank().get(op.get_in(0)?)?;
                if !base.is_constant()||base.get_offset()!=0{return None;}id=op.get_in(1)?;
            }
            _=>return None,
        }
    }
    None
}
fn main(){
    let args:Vec<_>=std::env::args().collect();
    let mut p=bootstrap_from_object(&args[1],"",&["/home/mahaloz/kwt/angr-rust-panic-main/specs".into()]).unwrap();p.commit_pending_analysis().unwrap();
    let mut s=ConsoleCommands::into_status(vec!["option setlanguage rust-language".into(),format!("load addr {}",args[2]),"decompile".into()]);register_decomp_commands(&mut s);
    s.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut().downcast_mut::<IfaceDecompData>().unwrap().conf=Some(p);
    for _ in 0..3 {execute(&mut s);assert!(!s.inerror,"{}",s.optr);}
    let d=s.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();let fd=d.fd.as_ref().unwrap();
    for i in 0..fd.num_calls(){let spec=fd.get_call_specs(i);if !matches!(spec.get_name(),"panic_fmt"|"core::panicking::panic_fmt"){continue;}
        let op=fd.obank().get(spec.get_op()).unwrap();if op.num_input()!=4 {continue;}
        let values:Vec<_>=(1..4).map(|i|constant(fd,op.get_in(i).unwrap()).map_or("null".into(),|v|v.to_string())).collect();println!("[{},{}]",op.get_addr().get_offset(),values.join(","));
    }
}
