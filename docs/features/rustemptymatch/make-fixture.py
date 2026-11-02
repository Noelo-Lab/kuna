from pathlib import Path
import json,subprocess,hashlib
r=Path('/home/mahaloz/kwt/angr-rust-empty-match');w=Path('/tmp/kuna-rust-empty-match');f=r/'decompiler/crates/kuna-analysis/tests/fixtures';parts=['.text\n'];names=['empty_case','empty_default','empty_shared','empty_loop','loop_break_control']
for name in names[:4]:
 loop=name=='empty_loop';s=f'.globl {name}\n.type {name},@function\n{name}:\n'
 if loop:s+='xor %edx,%edx\ntest %esi,%esi\njle .Lempty_loop_done\n'
 s+='shr $32,%rdi\n'
 if loop:s+='.Lempty_loop_loop:\n'
 s+=f'xor %eax,%eax\ncmp $40,%rdi\nja .L{name}_default\nlea .L{name}_table(%rip),%rcx\nmovslq (%rcx,%rdi,4),%r8\nadd %rcx,%r8\njmp *%r8\n'
 for i in range(41):
  if name!='empty_default'and(i==0 or(name=='empty_shared'and i==1)):continue
  s+=f'.L{name}_{i}:\nmov ${i+1 if name=="empty_default"else i},%eax\njmp .L{name}_join\n'
 s+=f'.L{name}_default:\n'
 if name!='empty_default':s+='mov $41,%eax\n'
 s+=f'.L{name}_join:\nadd $7,%eax\n'
 if loop:s+='add %eax,%edx\nsub $1,%esi\njnz .Lempty_loop_loop\n.Lempty_loop_done:\nmov %edx,%eax\n'
 s+=f'ret\n.size {name},.-{name}\n.pushsection .rodata\n.balign 4\n.L{name}_table:\n'
 for i in range(41):
  target=f'.L{name}_join'if name!='empty_default'and(i==0 or(name=='empty_shared'and i==1))else f'.L{name}_{i}';s+=f'.long {target}-.L{name}_table\n'
 s+='.popsection\n';parts.append(s)
parts.append('''.globl loop_break_control
.type loop_break_control,@function
loop_break_control:
    xor %eax,%eax
    test %esi,%esi
    jle .Lcontrol_done
.Lcontrol_loop:
    add $3,%eax
    cmp %edi,%eax
    jge .Lcontrol_done
    sub $1,%esi
    jnz .Lcontrol_loop
.Lcontrol_done:
    ret
.size loop_break_control,.-loop_break_control
.section .note.GNU-stack,"",@progbits
''');(f/'rust_empty_match.S').write_text(''.join(parts))
(f/'rust_empty_match.c').write_text('''#include <stdio.h>
#include <stdlib.h>
extern int empty_case(unsigned long);
extern int empty_default(unsigned long);
extern int empty_shared(unsigned long);
extern int empty_loop(unsigned long, int);
extern int loop_break_control(int, int);
int main(int argc,char **argv) {
    if (argc != 3) return 1;
    unsigned long x = strtoul(argv[1],0,0);
    int n = atoi(argv[2]);
    printf("%d %d %d %d %d\\n", empty_case(x),empty_default(x),empty_shared(x),empty_loop(x,n),loop_break_control(10,n));
    return 0;
}
''')
(f/'rust_empty_match.kuna').write_text(''.join(f'prototype {n} int {n}(unsigned long x)\n'for n in names[:3])+'prototype empty_loop int empty_loop(unsigned long x, int n)\nprototype loop_break_control int loop_break_control(int x, int n)\n')
cmd=['gcc','-O2','-fno-pie','-no-pie','-Wl,--build-id=none','rust_empty_match.S','rust_empty_match.c','-o','rust_empty_match_x86_64'];subprocess.run(cmd,cwd=f,check=True);(w/'fixture-names.json').write_text(json.dumps(names));record={'command':cmd,'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0],'files':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in f.glob('rust_empty_match*')}};(w/'fixture-build.json').write_text(json.dumps(record,indent=2)+'\n');print('Five authored fixtures built.')
