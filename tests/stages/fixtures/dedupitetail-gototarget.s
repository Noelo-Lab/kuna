# A loop that exits into the start of an if's true arm (a goto into the arm),
# with both arms ending in the same shared(y) call.  Source of the bytes in
# kuna-dedupitetail-gototarget.xml (as --64; objcopy -O binary -j .text).
#
#   void g(int x, int y) {
#       do { if (pre() == 1) goto T; } while (pre() == 2);
#       if (x) { T: if (cond_a()) body_a(); shared(y); }
#       else   {    if (cond_b()) body_b(); shared(y); }
#       post(y);
#   }
    .intel_syntax noprefix
    .text
g:
    push rbx
    push r12
    push r13
    mov ebx, esi
    mov r12d, edi
loop:
    call pre
    cmp eax, 1
    je true_arm
    call pre
    cmp eax, 2
    je loop
    test r12d, r12d
    je else_arm
true_arm:
    call cond_a
    test eax, eax
    je true_shared
    call body_a
true_shared:
    mov edi, ebx
    call shared
    jmp join
else_arm:
    call cond_b
    test eax, eax
    je else_shared
    call body_b
else_shared:
    mov edi, ebx
    call shared
join:
    mov edi, ebx
    call post
    pop r13
    pop r12
    pop rbx
    ret
cond_a: xor eax, eax
    ret
cond_b: xor eax, eax
    ret
body_a: ret
body_b: ret
shared: ret
post: xor eax, eax
    ret
pre: xor eax, eax
    ret
