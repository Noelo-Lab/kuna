.text
.globl empty_case
.type empty_case,@function
empty_case:
shr $32,%rdi
xor %eax,%eax
cmp $40,%rdi
ja .Lempty_case_default
lea .Lempty_case_table(%rip),%rcx
movslq (%rcx,%rdi,4),%r8
add %rcx,%r8
jmp *%r8
.Lempty_case_1:
mov $1,%eax
jmp .Lempty_case_join
.Lempty_case_2:
mov $2,%eax
jmp .Lempty_case_join
.Lempty_case_3:
mov $3,%eax
jmp .Lempty_case_join
.Lempty_case_4:
mov $4,%eax
jmp .Lempty_case_join
.Lempty_case_5:
mov $5,%eax
jmp .Lempty_case_join
.Lempty_case_6:
mov $6,%eax
jmp .Lempty_case_join
.Lempty_case_7:
mov $7,%eax
jmp .Lempty_case_join
.Lempty_case_8:
mov $8,%eax
jmp .Lempty_case_join
.Lempty_case_9:
mov $9,%eax
jmp .Lempty_case_join
.Lempty_case_10:
mov $10,%eax
jmp .Lempty_case_join
.Lempty_case_11:
mov $11,%eax
jmp .Lempty_case_join
.Lempty_case_12:
mov $12,%eax
jmp .Lempty_case_join
.Lempty_case_13:
mov $13,%eax
jmp .Lempty_case_join
.Lempty_case_14:
mov $14,%eax
jmp .Lempty_case_join
.Lempty_case_15:
mov $15,%eax
jmp .Lempty_case_join
.Lempty_case_16:
mov $16,%eax
jmp .Lempty_case_join
.Lempty_case_17:
mov $17,%eax
jmp .Lempty_case_join
.Lempty_case_18:
mov $18,%eax
jmp .Lempty_case_join
.Lempty_case_19:
mov $19,%eax
jmp .Lempty_case_join
.Lempty_case_20:
mov $20,%eax
jmp .Lempty_case_join
.Lempty_case_21:
mov $21,%eax
jmp .Lempty_case_join
.Lempty_case_22:
mov $22,%eax
jmp .Lempty_case_join
.Lempty_case_23:
mov $23,%eax
jmp .Lempty_case_join
.Lempty_case_24:
mov $24,%eax
jmp .Lempty_case_join
.Lempty_case_25:
mov $25,%eax
jmp .Lempty_case_join
.Lempty_case_26:
mov $26,%eax
jmp .Lempty_case_join
.Lempty_case_27:
mov $27,%eax
jmp .Lempty_case_join
.Lempty_case_28:
mov $28,%eax
jmp .Lempty_case_join
.Lempty_case_29:
mov $29,%eax
jmp .Lempty_case_join
.Lempty_case_30:
mov $30,%eax
jmp .Lempty_case_join
.Lempty_case_31:
mov $31,%eax
jmp .Lempty_case_join
.Lempty_case_32:
mov $32,%eax
jmp .Lempty_case_join
.Lempty_case_33:
mov $33,%eax
jmp .Lempty_case_join
.Lempty_case_34:
mov $34,%eax
jmp .Lempty_case_join
.Lempty_case_35:
mov $35,%eax
jmp .Lempty_case_join
.Lempty_case_36:
mov $36,%eax
jmp .Lempty_case_join
.Lempty_case_37:
mov $37,%eax
jmp .Lempty_case_join
.Lempty_case_38:
mov $38,%eax
jmp .Lempty_case_join
.Lempty_case_39:
mov $39,%eax
jmp .Lempty_case_join
.Lempty_case_40:
mov $40,%eax
jmp .Lempty_case_join
.Lempty_case_default:
mov $41,%eax
.Lempty_case_join:
add $7,%eax
ret
.size empty_case,.-empty_case
.pushsection .rodata
.balign 4
.Lempty_case_table:
.long .Lempty_case_join-.Lempty_case_table
.long .Lempty_case_1-.Lempty_case_table
.long .Lempty_case_2-.Lempty_case_table
.long .Lempty_case_3-.Lempty_case_table
.long .Lempty_case_4-.Lempty_case_table
.long .Lempty_case_5-.Lempty_case_table
.long .Lempty_case_6-.Lempty_case_table
.long .Lempty_case_7-.Lempty_case_table
.long .Lempty_case_8-.Lempty_case_table
.long .Lempty_case_9-.Lempty_case_table
.long .Lempty_case_10-.Lempty_case_table
.long .Lempty_case_11-.Lempty_case_table
.long .Lempty_case_12-.Lempty_case_table
.long .Lempty_case_13-.Lempty_case_table
.long .Lempty_case_14-.Lempty_case_table
.long .Lempty_case_15-.Lempty_case_table
.long .Lempty_case_16-.Lempty_case_table
.long .Lempty_case_17-.Lempty_case_table
.long .Lempty_case_18-.Lempty_case_table
.long .Lempty_case_19-.Lempty_case_table
.long .Lempty_case_20-.Lempty_case_table
.long .Lempty_case_21-.Lempty_case_table
.long .Lempty_case_22-.Lempty_case_table
.long .Lempty_case_23-.Lempty_case_table
.long .Lempty_case_24-.Lempty_case_table
.long .Lempty_case_25-.Lempty_case_table
.long .Lempty_case_26-.Lempty_case_table
.long .Lempty_case_27-.Lempty_case_table
.long .Lempty_case_28-.Lempty_case_table
.long .Lempty_case_29-.Lempty_case_table
.long .Lempty_case_30-.Lempty_case_table
.long .Lempty_case_31-.Lempty_case_table
.long .Lempty_case_32-.Lempty_case_table
.long .Lempty_case_33-.Lempty_case_table
.long .Lempty_case_34-.Lempty_case_table
.long .Lempty_case_35-.Lempty_case_table
.long .Lempty_case_36-.Lempty_case_table
.long .Lempty_case_37-.Lempty_case_table
.long .Lempty_case_38-.Lempty_case_table
.long .Lempty_case_39-.Lempty_case_table
.long .Lempty_case_40-.Lempty_case_table
.popsection
.globl empty_default
.type empty_default,@function
empty_default:
shr $32,%rdi
xor %eax,%eax
cmp $40,%rdi
ja .Lempty_default_default
lea .Lempty_default_table(%rip),%rcx
movslq (%rcx,%rdi,4),%r8
add %rcx,%r8
jmp *%r8
.Lempty_default_0:
mov $1,%eax
jmp .Lempty_default_join
.Lempty_default_1:
mov $2,%eax
jmp .Lempty_default_join
.Lempty_default_2:
mov $3,%eax
jmp .Lempty_default_join
.Lempty_default_3:
mov $4,%eax
jmp .Lempty_default_join
.Lempty_default_4:
mov $5,%eax
jmp .Lempty_default_join
.Lempty_default_5:
mov $6,%eax
jmp .Lempty_default_join
.Lempty_default_6:
mov $7,%eax
jmp .Lempty_default_join
.Lempty_default_7:
mov $8,%eax
jmp .Lempty_default_join
.Lempty_default_8:
mov $9,%eax
jmp .Lempty_default_join
.Lempty_default_9:
mov $10,%eax
jmp .Lempty_default_join
.Lempty_default_10:
mov $11,%eax
jmp .Lempty_default_join
.Lempty_default_11:
mov $12,%eax
jmp .Lempty_default_join
.Lempty_default_12:
mov $13,%eax
jmp .Lempty_default_join
.Lempty_default_13:
mov $14,%eax
jmp .Lempty_default_join
.Lempty_default_14:
mov $15,%eax
jmp .Lempty_default_join
.Lempty_default_15:
mov $16,%eax
jmp .Lempty_default_join
.Lempty_default_16:
mov $17,%eax
jmp .Lempty_default_join
.Lempty_default_17:
mov $18,%eax
jmp .Lempty_default_join
.Lempty_default_18:
mov $19,%eax
jmp .Lempty_default_join
.Lempty_default_19:
mov $20,%eax
jmp .Lempty_default_join
.Lempty_default_20:
mov $21,%eax
jmp .Lempty_default_join
.Lempty_default_21:
mov $22,%eax
jmp .Lempty_default_join
.Lempty_default_22:
mov $23,%eax
jmp .Lempty_default_join
.Lempty_default_23:
mov $24,%eax
jmp .Lempty_default_join
.Lempty_default_24:
mov $25,%eax
jmp .Lempty_default_join
.Lempty_default_25:
mov $26,%eax
jmp .Lempty_default_join
.Lempty_default_26:
mov $27,%eax
jmp .Lempty_default_join
.Lempty_default_27:
mov $28,%eax
jmp .Lempty_default_join
.Lempty_default_28:
mov $29,%eax
jmp .Lempty_default_join
.Lempty_default_29:
mov $30,%eax
jmp .Lempty_default_join
.Lempty_default_30:
mov $31,%eax
jmp .Lempty_default_join
.Lempty_default_31:
mov $32,%eax
jmp .Lempty_default_join
.Lempty_default_32:
mov $33,%eax
jmp .Lempty_default_join
.Lempty_default_33:
mov $34,%eax
jmp .Lempty_default_join
.Lempty_default_34:
mov $35,%eax
jmp .Lempty_default_join
.Lempty_default_35:
mov $36,%eax
jmp .Lempty_default_join
.Lempty_default_36:
mov $37,%eax
jmp .Lempty_default_join
.Lempty_default_37:
mov $38,%eax
jmp .Lempty_default_join
.Lempty_default_38:
mov $39,%eax
jmp .Lempty_default_join
.Lempty_default_39:
mov $40,%eax
jmp .Lempty_default_join
.Lempty_default_40:
mov $41,%eax
jmp .Lempty_default_join
.Lempty_default_default:
.Lempty_default_join:
add $7,%eax
ret
.size empty_default,.-empty_default
.pushsection .rodata
.balign 4
.Lempty_default_table:
.long .Lempty_default_0-.Lempty_default_table
.long .Lempty_default_1-.Lempty_default_table
.long .Lempty_default_2-.Lempty_default_table
.long .Lempty_default_3-.Lempty_default_table
.long .Lempty_default_4-.Lempty_default_table
.long .Lempty_default_5-.Lempty_default_table
.long .Lempty_default_6-.Lempty_default_table
.long .Lempty_default_7-.Lempty_default_table
.long .Lempty_default_8-.Lempty_default_table
.long .Lempty_default_9-.Lempty_default_table
.long .Lempty_default_10-.Lempty_default_table
.long .Lempty_default_11-.Lempty_default_table
.long .Lempty_default_12-.Lempty_default_table
.long .Lempty_default_13-.Lempty_default_table
.long .Lempty_default_14-.Lempty_default_table
.long .Lempty_default_15-.Lempty_default_table
.long .Lempty_default_16-.Lempty_default_table
.long .Lempty_default_17-.Lempty_default_table
.long .Lempty_default_18-.Lempty_default_table
.long .Lempty_default_19-.Lempty_default_table
.long .Lempty_default_20-.Lempty_default_table
.long .Lempty_default_21-.Lempty_default_table
.long .Lempty_default_22-.Lempty_default_table
.long .Lempty_default_23-.Lempty_default_table
.long .Lempty_default_24-.Lempty_default_table
.long .Lempty_default_25-.Lempty_default_table
.long .Lempty_default_26-.Lempty_default_table
.long .Lempty_default_27-.Lempty_default_table
.long .Lempty_default_28-.Lempty_default_table
.long .Lempty_default_29-.Lempty_default_table
.long .Lempty_default_30-.Lempty_default_table
.long .Lempty_default_31-.Lempty_default_table
.long .Lempty_default_32-.Lempty_default_table
.long .Lempty_default_33-.Lempty_default_table
.long .Lempty_default_34-.Lempty_default_table
.long .Lempty_default_35-.Lempty_default_table
.long .Lempty_default_36-.Lempty_default_table
.long .Lempty_default_37-.Lempty_default_table
.long .Lempty_default_38-.Lempty_default_table
.long .Lempty_default_39-.Lempty_default_table
.long .Lempty_default_40-.Lempty_default_table
.popsection
.globl empty_shared
.type empty_shared,@function
empty_shared:
shr $32,%rdi
xor %eax,%eax
cmp $40,%rdi
ja .Lempty_shared_default
lea .Lempty_shared_table(%rip),%rcx
movslq (%rcx,%rdi,4),%r8
add %rcx,%r8
jmp *%r8
.Lempty_shared_2:
mov $2,%eax
jmp .Lempty_shared_join
.Lempty_shared_3:
mov $3,%eax
jmp .Lempty_shared_join
.Lempty_shared_4:
mov $4,%eax
jmp .Lempty_shared_join
.Lempty_shared_5:
mov $5,%eax
jmp .Lempty_shared_join
.Lempty_shared_6:
mov $6,%eax
jmp .Lempty_shared_join
.Lempty_shared_7:
mov $7,%eax
jmp .Lempty_shared_join
.Lempty_shared_8:
mov $8,%eax
jmp .Lempty_shared_join
.Lempty_shared_9:
mov $9,%eax
jmp .Lempty_shared_join
.Lempty_shared_10:
mov $10,%eax
jmp .Lempty_shared_join
.Lempty_shared_11:
mov $11,%eax
jmp .Lempty_shared_join
.Lempty_shared_12:
mov $12,%eax
jmp .Lempty_shared_join
.Lempty_shared_13:
mov $13,%eax
jmp .Lempty_shared_join
.Lempty_shared_14:
mov $14,%eax
jmp .Lempty_shared_join
.Lempty_shared_15:
mov $15,%eax
jmp .Lempty_shared_join
.Lempty_shared_16:
mov $16,%eax
jmp .Lempty_shared_join
.Lempty_shared_17:
mov $17,%eax
jmp .Lempty_shared_join
.Lempty_shared_18:
mov $18,%eax
jmp .Lempty_shared_join
.Lempty_shared_19:
mov $19,%eax
jmp .Lempty_shared_join
.Lempty_shared_20:
mov $20,%eax
jmp .Lempty_shared_join
.Lempty_shared_21:
mov $21,%eax
jmp .Lempty_shared_join
.Lempty_shared_22:
mov $22,%eax
jmp .Lempty_shared_join
.Lempty_shared_23:
mov $23,%eax
jmp .Lempty_shared_join
.Lempty_shared_24:
mov $24,%eax
jmp .Lempty_shared_join
.Lempty_shared_25:
mov $25,%eax
jmp .Lempty_shared_join
.Lempty_shared_26:
mov $26,%eax
jmp .Lempty_shared_join
.Lempty_shared_27:
mov $27,%eax
jmp .Lempty_shared_join
.Lempty_shared_28:
mov $28,%eax
jmp .Lempty_shared_join
.Lempty_shared_29:
mov $29,%eax
jmp .Lempty_shared_join
.Lempty_shared_30:
mov $30,%eax
jmp .Lempty_shared_join
.Lempty_shared_31:
mov $31,%eax
jmp .Lempty_shared_join
.Lempty_shared_32:
mov $32,%eax
jmp .Lempty_shared_join
.Lempty_shared_33:
mov $33,%eax
jmp .Lempty_shared_join
.Lempty_shared_34:
mov $34,%eax
jmp .Lempty_shared_join
.Lempty_shared_35:
mov $35,%eax
jmp .Lempty_shared_join
.Lempty_shared_36:
mov $36,%eax
jmp .Lempty_shared_join
.Lempty_shared_37:
mov $37,%eax
jmp .Lempty_shared_join
.Lempty_shared_38:
mov $38,%eax
jmp .Lempty_shared_join
.Lempty_shared_39:
mov $39,%eax
jmp .Lempty_shared_join
.Lempty_shared_40:
mov $40,%eax
jmp .Lempty_shared_join
.Lempty_shared_default:
mov $41,%eax
.Lempty_shared_join:
add $7,%eax
ret
.size empty_shared,.-empty_shared
.pushsection .rodata
.balign 4
.Lempty_shared_table:
.long .Lempty_shared_join-.Lempty_shared_table
.long .Lempty_shared_join-.Lempty_shared_table
.long .Lempty_shared_2-.Lempty_shared_table
.long .Lempty_shared_3-.Lempty_shared_table
.long .Lempty_shared_4-.Lempty_shared_table
.long .Lempty_shared_5-.Lempty_shared_table
.long .Lempty_shared_6-.Lempty_shared_table
.long .Lempty_shared_7-.Lempty_shared_table
.long .Lempty_shared_8-.Lempty_shared_table
.long .Lempty_shared_9-.Lempty_shared_table
.long .Lempty_shared_10-.Lempty_shared_table
.long .Lempty_shared_11-.Lempty_shared_table
.long .Lempty_shared_12-.Lempty_shared_table
.long .Lempty_shared_13-.Lempty_shared_table
.long .Lempty_shared_14-.Lempty_shared_table
.long .Lempty_shared_15-.Lempty_shared_table
.long .Lempty_shared_16-.Lempty_shared_table
.long .Lempty_shared_17-.Lempty_shared_table
.long .Lempty_shared_18-.Lempty_shared_table
.long .Lempty_shared_19-.Lempty_shared_table
.long .Lempty_shared_20-.Lempty_shared_table
.long .Lempty_shared_21-.Lempty_shared_table
.long .Lempty_shared_22-.Lempty_shared_table
.long .Lempty_shared_23-.Lempty_shared_table
.long .Lempty_shared_24-.Lempty_shared_table
.long .Lempty_shared_25-.Lempty_shared_table
.long .Lempty_shared_26-.Lempty_shared_table
.long .Lempty_shared_27-.Lempty_shared_table
.long .Lempty_shared_28-.Lempty_shared_table
.long .Lempty_shared_29-.Lempty_shared_table
.long .Lempty_shared_30-.Lempty_shared_table
.long .Lempty_shared_31-.Lempty_shared_table
.long .Lempty_shared_32-.Lempty_shared_table
.long .Lempty_shared_33-.Lempty_shared_table
.long .Lempty_shared_34-.Lempty_shared_table
.long .Lempty_shared_35-.Lempty_shared_table
.long .Lempty_shared_36-.Lempty_shared_table
.long .Lempty_shared_37-.Lempty_shared_table
.long .Lempty_shared_38-.Lempty_shared_table
.long .Lempty_shared_39-.Lempty_shared_table
.long .Lempty_shared_40-.Lempty_shared_table
.popsection
.globl empty_loop
.type empty_loop,@function
empty_loop:
xor %edx,%edx
test %esi,%esi
jle .Lempty_loop_done
shr $32,%rdi
.Lempty_loop_loop:
xor %eax,%eax
cmp $40,%rdi
ja .Lempty_loop_default
lea .Lempty_loop_table(%rip),%rcx
movslq (%rcx,%rdi,4),%r8
add %rcx,%r8
jmp *%r8
.Lempty_loop_1:
mov $1,%eax
jmp .Lempty_loop_join
.Lempty_loop_2:
mov $2,%eax
jmp .Lempty_loop_join
.Lempty_loop_3:
mov $3,%eax
jmp .Lempty_loop_join
.Lempty_loop_4:
mov $4,%eax
jmp .Lempty_loop_join
.Lempty_loop_5:
mov $5,%eax
jmp .Lempty_loop_join
.Lempty_loop_6:
mov $6,%eax
jmp .Lempty_loop_join
.Lempty_loop_7:
mov $7,%eax
jmp .Lempty_loop_join
.Lempty_loop_8:
mov $8,%eax
jmp .Lempty_loop_join
.Lempty_loop_9:
mov $9,%eax
jmp .Lempty_loop_join
.Lempty_loop_10:
mov $10,%eax
jmp .Lempty_loop_join
.Lempty_loop_11:
mov $11,%eax
jmp .Lempty_loop_join
.Lempty_loop_12:
mov $12,%eax
jmp .Lempty_loop_join
.Lempty_loop_13:
mov $13,%eax
jmp .Lempty_loop_join
.Lempty_loop_14:
mov $14,%eax
jmp .Lempty_loop_join
.Lempty_loop_15:
mov $15,%eax
jmp .Lempty_loop_join
.Lempty_loop_16:
mov $16,%eax
jmp .Lempty_loop_join
.Lempty_loop_17:
mov $17,%eax
jmp .Lempty_loop_join
.Lempty_loop_18:
mov $18,%eax
jmp .Lempty_loop_join
.Lempty_loop_19:
mov $19,%eax
jmp .Lempty_loop_join
.Lempty_loop_20:
mov $20,%eax
jmp .Lempty_loop_join
.Lempty_loop_21:
mov $21,%eax
jmp .Lempty_loop_join
.Lempty_loop_22:
mov $22,%eax
jmp .Lempty_loop_join
.Lempty_loop_23:
mov $23,%eax
jmp .Lempty_loop_join
.Lempty_loop_24:
mov $24,%eax
jmp .Lempty_loop_join
.Lempty_loop_25:
mov $25,%eax
jmp .Lempty_loop_join
.Lempty_loop_26:
mov $26,%eax
jmp .Lempty_loop_join
.Lempty_loop_27:
mov $27,%eax
jmp .Lempty_loop_join
.Lempty_loop_28:
mov $28,%eax
jmp .Lempty_loop_join
.Lempty_loop_29:
mov $29,%eax
jmp .Lempty_loop_join
.Lempty_loop_30:
mov $30,%eax
jmp .Lempty_loop_join
.Lempty_loop_31:
mov $31,%eax
jmp .Lempty_loop_join
.Lempty_loop_32:
mov $32,%eax
jmp .Lempty_loop_join
.Lempty_loop_33:
mov $33,%eax
jmp .Lempty_loop_join
.Lempty_loop_34:
mov $34,%eax
jmp .Lempty_loop_join
.Lempty_loop_35:
mov $35,%eax
jmp .Lempty_loop_join
.Lempty_loop_36:
mov $36,%eax
jmp .Lempty_loop_join
.Lempty_loop_37:
mov $37,%eax
jmp .Lempty_loop_join
.Lempty_loop_38:
mov $38,%eax
jmp .Lempty_loop_join
.Lempty_loop_39:
mov $39,%eax
jmp .Lempty_loop_join
.Lempty_loop_40:
mov $40,%eax
jmp .Lempty_loop_join
.Lempty_loop_default:
mov $41,%eax
.Lempty_loop_join:
add $7,%eax
add %eax,%edx
sub $1,%esi
jnz .Lempty_loop_loop
.Lempty_loop_done:
mov %edx,%eax
ret
.size empty_loop,.-empty_loop
.pushsection .rodata
.balign 4
.Lempty_loop_table:
.long .Lempty_loop_join-.Lempty_loop_table
.long .Lempty_loop_1-.Lempty_loop_table
.long .Lempty_loop_2-.Lempty_loop_table
.long .Lempty_loop_3-.Lempty_loop_table
.long .Lempty_loop_4-.Lempty_loop_table
.long .Lempty_loop_5-.Lempty_loop_table
.long .Lempty_loop_6-.Lempty_loop_table
.long .Lempty_loop_7-.Lempty_loop_table
.long .Lempty_loop_8-.Lempty_loop_table
.long .Lempty_loop_9-.Lempty_loop_table
.long .Lempty_loop_10-.Lempty_loop_table
.long .Lempty_loop_11-.Lempty_loop_table
.long .Lempty_loop_12-.Lempty_loop_table
.long .Lempty_loop_13-.Lempty_loop_table
.long .Lempty_loop_14-.Lempty_loop_table
.long .Lempty_loop_15-.Lempty_loop_table
.long .Lempty_loop_16-.Lempty_loop_table
.long .Lempty_loop_17-.Lempty_loop_table
.long .Lempty_loop_18-.Lempty_loop_table
.long .Lempty_loop_19-.Lempty_loop_table
.long .Lempty_loop_20-.Lempty_loop_table
.long .Lempty_loop_21-.Lempty_loop_table
.long .Lempty_loop_22-.Lempty_loop_table
.long .Lempty_loop_23-.Lempty_loop_table
.long .Lempty_loop_24-.Lempty_loop_table
.long .Lempty_loop_25-.Lempty_loop_table
.long .Lempty_loop_26-.Lempty_loop_table
.long .Lempty_loop_27-.Lempty_loop_table
.long .Lempty_loop_28-.Lempty_loop_table
.long .Lempty_loop_29-.Lempty_loop_table
.long .Lempty_loop_30-.Lempty_loop_table
.long .Lempty_loop_31-.Lempty_loop_table
.long .Lempty_loop_32-.Lempty_loop_table
.long .Lempty_loop_33-.Lempty_loop_table
.long .Lempty_loop_34-.Lempty_loop_table
.long .Lempty_loop_35-.Lempty_loop_table
.long .Lempty_loop_36-.Lempty_loop_table
.long .Lempty_loop_37-.Lempty_loop_table
.long .Lempty_loop_38-.Lempty_loop_table
.long .Lempty_loop_39-.Lempty_loop_table
.long .Lempty_loop_40-.Lempty_loop_table
.popsection
.globl loop_break_control
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
