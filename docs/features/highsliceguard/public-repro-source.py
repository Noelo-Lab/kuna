from pathlib import Path
w=Path(__file__).resolve().parent/'public-repro';w.mkdir(exist_ok=True)
source='.text\n.globl pick\n.type pick,@function\npick:\n shr $32,%rdi\n cmp $40,%edi\n ja fallback\n lea table(%rip),%rax\n movslq (%rax,%rdi,4),%rdi\n add %rax,%rdi\n jmp *%rdi\n'
source+=''.join(f'case{i}:\n mov ${i*7+3},%eax\n ret\n'for i in range(41))
source+='fallback:\n mov $1000,%eax\n ret\n.size pick,.-pick\n.section .rodata\n.balign 4\ntable:\n'
source+=''.join(f' .long case{i}-table\n'for i in range(41))
source+='.section .note.GNU-stack,"",@progbits\n';(w/'guard.S').write_text(source)
