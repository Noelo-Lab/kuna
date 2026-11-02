from pathlib import Path
import json,copy,hashlib,subprocess
w=Path('/tmp/kuna-rust-fmt-write');address=0x11230
paths={'single_before':w/'reference-write-before.json','single_after':w/'reference-write-after.json','full_before':Path('/tmp/kuna-rust-fmt-str/breadth/rust-rust.json'),'full_after':w/'breadth/rust-rust.json'}
functions={key:next(f for f in json.loads(path.read_text())['functions']if f['address']==address)for key,path in paths.items()}
for side in ['before','after']:
    prior=functions['single_'+side];fresh=functions['full_'+side];expected=copy.deepcopy(prior)
    old='unsafe fn write_str(mut a0: u64, mut a1: u64)';new='unsafe fn write_str(mut a0: *mut (), mut a1: u64)'
    assert expected['code'].count(old)==1;expected['code']=expected['code'].replace(old,new)
    if side=='before':
        old='core::fmt::write(a0,0x4e388,0x5f14,&mut v2)';new='core::fmt::write(a0,0x4e388 as *mut (),0x5f14 as *mut u16,&mut v2)'
    else:
        old='&mut *(a0 as *mut std::string::String)';new='&mut *((a0 as *mut _) as *mut std::string::String)'
    assert expected['code'].count(old)==1;expected['code']=expected['code'].replace(old,new)
    args=[v for v in expected['variables']if v['kind']=='arg'and v['arg_index']==0];assert len(args)==1 and args[0]['type']=='u64';args[0]['type']='*mut ()'
    assert expected==fresh,('Unexpected full-context difference',side)

def expression(function):
    line=next(line.strip()for line in function['code'].splitlines()if'use std::fmt::Write as _;'in line)
    suffix=" == b'\\0' {";assert line.startswith('if ')and line.endswith(suffix)
    return line[3:-len(suffix)]
old_expression=expression(functions['single_after']);fresh_expression=expression(functions['full_after'])
values=list(range(-128,128))+[-2**63,2**63-1,-257,256,-65537,65536,2**32,-2**32]
main='''fn emit(code:u8,out:String){assert_eq!(code,0);println!("{} {:?}",code,out);}
fn main(){for capacity in [0,8,1024]{for prefix in ["","prefix:é🙂\\0","long-long-long-existing:"]{for value in [VALUES]{
let mut out=String::with_capacity(capacity);out.push_str(prefix);
let code=write_reference(&mut out,value);emit(code,out);
}}}}
'''.replace('VALUES',','.join(str(v)+'i64'for v in values))
native='fn write_reference(out:&mut String,value:i64)->u8{std::fmt::write(out,format_args!("[{}]",value)).is_err() as u8}\n'+main
recovered='fn write_reference(out:&mut String,value:i64)->u8{unsafe{let a0=out as *mut String as *mut ();let v2=&value as *const i64;'+fresh_expression+'}}\n'+main
(w/'reference-context-native.rs').write_text(native);(w/'reference-context-execution.rs').write_text(recovered)
def execute(source,version,label):
    binary=w/label
    result=subprocess.run(['rustc','+'+version,'--edition=2021','--crate-name','reference_context',str(w/source),'-o',str(binary)],capture_output=True)
    (w/(label+'.compile.log')).write_bytes(result.stderr);assert result.returncode==0,result.stderr.decode()
    result=subprocess.run([str(binary)],capture_output=True,check=True);assert not result.stderr
    (w/(label+'.stdout')).write_bytes(result.stdout)
    return result.stdout
expected=execute('reference-context-native.rs','1.93.1','reference-context-native-1.93.1');count=len(expected.splitlines());assert count==len(values)*3*3
executions=[]
for version in ['1.90.0','1.93.1','1.99.1']:
    output=execute('reference-context-execution.rs',version,'reference-context-recovered-'+version);assert output==expected
    executions.append({'compiler':version,'calls':count,'stdout_sha256':hashlib.sha256(output).hexdigest(),'original_string_bytes_and_result_bytes_match':True})
record={'reference_address':hex(address),'context_difference_preexists_feature':True,'exact_allowed_differences':['writer parameter u64 to *mut (), including its variable type','parent runtime call arguments gain their inferred pointer casts','recovered writer gains the corresponding pointer cast through *mut _'],'all_other_complete_function_fields_identical':True,'source_instruction_mappings_identical_across_contexts':True,'fresh_emitted_expression_unchanged':fresh_expression,'single_emitted_expression':old_expression,'functions':functions,'input_file_sha256':{key:hashlib.sha256(path.read_bytes()).hexdigest()for key,path in paths.items()},'native_source':'std::fmt::write(out, format_args!("[{}]", value))','native_producer':'1.93.1','signed_values':len(values),'prefixes':3,'initial_capacities':3,'total_comparisons':count*len(executions),'executions':executions,'complete_function_recompilation_claimed':False}
(w/'reference-context-review.json').write_text(json.dumps(record,indent=2)+'\n');print('REFERENCE FULL-CONTEXT TYPE DIFFERENCE AND UNCHANGED EXPRESSION VERIFIED',count*len(executions),flush=True)
