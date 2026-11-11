#!/usr/bin/env python3
"""Build small amd64 Go-ABI functions with synthetic cross-unit Go DWARF."""
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent
functions = [
    ('fromWord', 'ret', [('p','u64',0)], ['p32']),
    ('narrow', 'ret', [('a','u64',0)], ['u32']),
    ('extend', 'movsbq %al,%rax; ret', [('a','i8',0)], ['i64']),
    ('boolNot', 'xor $1,%al; ret', [('x','bool',0)], ['bool']),
    ('mix', 'addsd %xmm1,%xmm0; cvtsi2sdq %rax,%xmm2; addsd %xmm2,%xmm0; ret', [('x','f64',17),('n','i64',0),('y','f64',18)], ['f64']),
    ('store', 'mov %ebx,(%rax); ret', [('p','p32',0),('v','i32',3)], []),
    ('add', 'lea (%rax,%rbx),%rax; ret', [('a','i64',0),('b','i64',3)], ['i64']),
    ('scale', 'mov (%rax),%rcx; imul %rbx,%rcx; mov %rcx,(%rax); imulq 8(%rax),%rbx; mov %rbx,8(%rax); ret', [('p','ppoint',0),('k','i64',3)], []),
    ('callIdentity', 'call main.fromWord; ret', [('p','u64',0)], ['p32']),
    ('noResult', 'mov %eax,main.state(%rip); ret', [('a','i32',0)], []),
    ('resultOnly', 'mov $42,%eax; ret', [], ['i64']),
    ('badReg', 'ret', [('p','u64',3)], ['u32']),
    ('missingFlag', 'ret', [('p','u64',0)], ['u32']),
    ('multiResult', 'lea 1(%rax),%rbx; ret', [('p','u64',0)], ['u64','u64']),
    ('trampoline', 'ret', [('p','u64',0)], ['u32']),
    ('worker.func1', 'ret', [('p','u64',0)], ['u32']),
    ('badType', 'ret', [('p','badptr',0)], ['u32']),
    ('tooWide', 'ret', [('p','i128',0)], ['i128']),
    ('wrongProducer', 'ret', [('p','u64',0)], ['u32']),
    ('abstractIdentity', 'ret', [('p','u64',0)], ['p32']),
    ('badResultReg', 'ret', [('p','u64',0)], ['u32']),
    ('stackInput', 'ret', [('p','u64',0)], ['u32']),
    ('tooManyIntegers', 'ret', [('a'+str(i),'i64',r) for i,r in enumerate([0,3,2,5,4,8,9,10,11,0])], ['i64']),
    ('tooManyFloats', 'ret', [('a'+str(i),'f64',17+i) for i in range(9)], ['f64']),
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
    emit('.byte '+('9' if name=='trampoline' else '2'),string('main.'+name),'.quad main.'+name)
    if name=='trampoline':emit('.byte 1')
    for param,ty,reg in inputs:
        emit('.byte '+('8' if name=='missingFlag' else '3'),string(param))
        if name!='missingFlag':emit('.byte 0')
        emit(ref(ty),'.byte 2,0x91,0' if name=='stackInput' else '.byte 1,'+str(0x50+reg))
    for i,ty in enumerate(results):emit('.byte 3',string('~r'+str(i)),'.byte 1',ref(ty),'.byte 1,0x53' if name=='badResultReg' else '.byte 0')
    emit('.byte 0')
cu_start('cu_functions')
for name,_,inputs,results in functions:
    if name not in ['wrongProducer','abstractIdentity']:function(name,inputs,results)
emit('abstract_function:', '.byte 10',string('main.abstractIdentity'),'.byte 1',
     'abstract_parameter:', '.byte 11',string('p'),'.byte 0',ref('u64'),'.byte 0',
     '.byte 12','.long abstract_function-cu_functions','.quad main.abstractIdentity',
     '.byte 13','.long abstract_parameter-cu_functions','.byte 1,0x50',
     '.byte 3',string('~r0'),'.byte 1',ref('p32'),'.byte 0','.byte 0')
cu_end('cu_functions')
cu_start('cu_types')
for name,size,encoding,kind in [('i64',8,5,6),('u64',8,7,11),('i32',4,5,5),('u32',4,7,10),('i8',1,5,3),('bool',1,2,1),('f64',8,4,14),('i128',16,5,6)]:
    emit('type_'+name+':','.byte 4',string(name),f'.byte {encoding},{size},{kind}')
for name,target in [('p32','i32'),('ppoint','point')]:emit('type_'+name+':','.byte 5',string('*'+target),ref(target),'.byte 22')
emit('type_badptr:', '.byte 5',string('*broken'),'.long 0xfffffff0','.byte 22',
     'type_point:','.byte 6',string('main.point'),'.byte 16,25',
     '.byte 7',string('x'),ref('i64'),'.byte 0', '.byte 7',string('y'),ref('i64'),'.byte 8','.byte 0')
cu_end('cu_types')
cu_start('cu_old','Go cmd/compile go1.16.15')
function('wrongProducer',[('p','u64',0)],['u32'])
cu_end('cu_old')
emit('.section .note.GNU-stack,"",@progbits')
with tempfile.TemporaryDirectory(prefix='kuna-go-dwarf-') as directory:
    directory=Path(directory);source=directory/'fixture.S';obj=directory/'fixture.o'
    source.write_text('\n'.join(lines)+'\n')
    subprocess.run(['as','--64',str(source),'-o',str(obj)],check=True)
    subprocess.run(['ld','-m','elf_x86_64','-e','_start','-Ttext=0x401000','--build-id=none','-o',str(ROOT/'uncompressed'),str(obj)],check=True)
    subprocess.run(['objcopy','--compress-debug-sections=zlib',str(ROOT/'uncompressed'),str(ROOT/'compressed')],check=True)
print('Built',len(functions),'functions with cross-unit Go DWARF')
