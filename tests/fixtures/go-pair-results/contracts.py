#!/usr/bin/env python3
"""Build paired-result Go DWARF contracts and malformed cross-unit controls."""
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent
pair_body = 'mov %rax,%rcx; add %rbx,%rax; sub %rbx,%rcx; mov %rcx,%rbx; ret'
inputs = [('a','i64',0),('b','i64',3)]
functions = [
    ('pair', pair_body, inputs, ['i64','i64']),
    ('explicitPair', pair_body, inputs, ['i64','i64']),
    ('identityPair', 'ret', inputs, ['i64','i64']),
    ('signedUnsigned', pair_body, inputs, ['i64','u64']),
    ('badInput', pair_body, [('a','i64',0),('b','i64',2)], ['i64','i64']),
    ('swappedResults', pair_body, inputs, ['i64','i64']),
    ('wrongSecond', pair_body, inputs, ['i64','i64']),
    ('missingRole', pair_body, inputs, ['i64','i64']),
    ('interleaved', pair_body, inputs, ['i64','i64']),
    ('narrowPair', pair_body, inputs, ['i32','i64']),
    ('floatPair', pair_body, inputs, ['f64','f64']),
    ('mixedPair', pair_body, inputs, ['i64','f64']),
    ('threeResults', pair_body, inputs, ['i64','i64','i64']),
    ('aggregatePair', pair_body, inputs, ['point','i64']),
    ('duplicate', pair_body, inputs, ['i64','i64']),
    ('sameDuplicate', pair_body, inputs, ['i64','i64']),
    ('wrongProducer', pair_body, inputs, ['i64','i64']),
    ('pair.abi0', pair_body, inputs, ['i64','i64']),
    ('apply', 'mov %rax,%rdx; mov %rbx,%rax; mov %rcx,%rbx; call *%rdx; ret', [('fn','badptr',0),('a','i64',3),('b','i64',2)], ['i64','i64']),
    ('indirectFirst', 'mov %rax,%rdx; mov %rbx,%rax; mov %rcx,%rbx; call *%rdx; ret', [('fn','badptr',0),('a','i64',3),('b','i64',2)], ['i64']),
    ('indirectSecond', 'mov %rax,%rdx; mov %rbx,%rax; mov %rcx,%rbx; call *%rdx; mov %rbx,%rax; ret', [('fn','badptr',0),('a','i64',3),('b','i64',2)], ['i64']),
    ('indirectNarrow', 'mov %rax,%rdx; mov %rbx,%rax; mov %rcx,%rbx; call *%rdx; mov %ebx,%eax; ret', [('fn','badptr',0),('a','i64',3),('b','i64',2)], ['i32']),
]

lines = ['.text', '.global _start', '_start: ret']
def emit(*items): lines.extend(items)
def string(value): return '.asciz "'+value+'"'
def uleb(value): return '.uleb128 '+str(value)
def ref(name): return '.long type_'+name+'-debug_info'
for name,body,_,_ in functions:
    emit('.p2align 4', '.global main.'+name, '.type main.'+name+',@function', 'main.'+name+':', body,
         '.size main.'+name+',.-main.'+name)
emit('.data', '.global main.state', 'main.state: .long 0',
     '.section .go.buildinfo,"a"', '.balign 16', '.ascii "\\377 Go buildinf:"', '.byte 8,2', '.zero 16',
     '.byte 8', '.ascii "go1.22.5"', '.byte 0', '.section .debug_abbrev,"",@progbits')
def abbrev(number,tag,children,attrs):
    emit(uleb(number), uleb(tag), '.byte '+str(children))
    for name,form in attrs: emit(uleb(name),uleb(form))
    emit('.byte 0,0')
abbrev(1,0x11,1,[(3,8),(0x13,5),(0x25,8)])
abbrev(2,0x2e,1,[(3,8),(0x11,1)])
abbrev(3,5,0,[(3,8),(0x4b,0xc),(0x49,0x10),(2,0x18)])
abbrev(4,0x24,0,[(3,8),(0x3e,0xb),(0xb,0xb),(0x2900,0xb)])
abbrev(5,0xf,0,[(3,8),(0x49,0x10),(0x2900,0xb)])
abbrev(6,0x13,1,[(3,8),(0xb,0xb),(0x2900,0xb)])
abbrev(7,0xd,0,[(3,8),(0x49,0x10),(0x38,0xb)])
abbrev(8,5,0,[(3,8),(0x49,0x10),(2,0x18)])
abbrev(9,0x2e,1,[(3,8),(0x11,1),(0x56,0xc)])
abbrev(10,0x2e,1,[(3,8),(0x20,0xb)])
abbrev(11,5,0,[(3,8),(0x4b,0xc),(0x49,0x10)])
abbrev(12,0x2e,1,[(0x31,0x13),(0x11,1)])
abbrev(13,5,0,[(0x31,0x13),(2,0x18)])
emit('.byte 0', '.section .debug_info,"",@progbits', 'debug_info:')
def cu_start(label,producer='Go cmd/compile go1.22.5; regabi'):
    emit(label+':', '.long '+label+'_end-'+label+'-4', '.short 4', '.long 0', '.byte 8', '.byte 1',
         string('main'), '.short 22', string(producer))
def cu_end(label): emit('.byte 0',label+'_end:')
def function(name,inputs,results):
    emit('.byte 2',string('main.'+name),'.quad main.'+name)
    def input_formal(param,ty,reg):
        emit('.byte 3',string(param),'.byte 0',ref(ty),'.byte 1,'+str(0x50+reg))
    for param,ty,reg in inputs:
        if name!='interleaved' or param!='b':input_formal(param,ty,reg)
    for i,ty in enumerate(results):
        missing=name=='missingRole' and i==1
        emit('.byte '+('8' if missing else '3'),string('~r'+str(i)))
        if not missing:emit('.byte 1')
        emit(ref(ty))
        registers={'explicitPair':[0,3],'swappedResults':[3,0],'wrongSecond':[0,2]}.get(name)
        emit('.byte 1,'+str(0x50+registers[i]) if registers else '.byte 0')
        if name=='interleaved' and i==0:input_formal('b','i64',3)
    emit('.byte 0')
cu_start('cu_functions')
for name,_,args,results in functions:
    if name!='wrongProducer':function(name,args,results)
cu_end('cu_functions')
cu_start('cu_duplicates')
function('duplicate',inputs,['u64','u64'])
function('sameDuplicate',inputs,['i64','i64'])
cu_end('cu_duplicates')
cu_start('cu_types')
for name,size,encoding,kind in [('i64',8,5,6),('u64',8,7,11),('i32',4,5,5),('u32',4,7,10),('i8',1,5,3),('bool',1,2,1),('f64',8,4,14),('i128',16,5,6)]:
    emit('type_'+name+':','.byte 4',string(name),f'.byte {encoding},{size},{kind}')
for name,target in [('p32','i32'),('ppoint','point')]:emit('type_'+name+':','.byte 5',string('*'+target),ref(target),'.byte 22')
emit('type_badptr:', '.byte 5',string('*broken'),'.long 0xfffffff0','.byte 22',
     'type_point:','.byte 6',string('main.point'),'.byte 16,25',
     '.byte 7',string('x'),ref('i64'),'.byte 0', '.byte 7',string('y'),ref('i64'),'.byte 8','.byte 0')
cu_end('cu_types')
cu_start('cu_old','Go cmd/compile go1.16.15')
function('wrongProducer',inputs,['i64','i64'])
cu_end('cu_old')
emit('.section .note.GNU-stack,"",@progbits')
with tempfile.TemporaryDirectory(prefix='kuna-go-dwarf-') as directory:
    directory=Path(directory);source=directory/'fixture.S';obj=directory/'fixture.o'
    source.write_text('\n'.join(lines)+'\n')
    subprocess.run(['as','--64',str(source),'-o',str(obj)],check=True)
    subprocess.run(['ld','-m','elf_x86_64','-e','_start','-Ttext=0x401000','--build-id=none','-o',str(ROOT/'contracts-plain'),str(obj)],check=True)
    subprocess.run(['objcopy','--compress-debug-sections=zlib',str(ROOT/'contracts-plain'),str(ROOT/'contracts')],check=True)
print('Built',len(functions),'functions with cross-unit Go DWARF')
