.text
.global stack_value
.type stack_value,@function
stack_value:
 save %sp, -112, %sp
 st %i0, [%fp-8]
 st %i1, [%fp-4]
 add %fp, -8, %o0
 call value_sink
 nop
 ret
 restore
.size stack_value,.-stack_value
.global stack_pointer
.type stack_pointer,@function
stack_pointer:
 save %sp, -112, %sp
 st %i0, [%fp-8]
 st %i1, [%fp-4]
 add %fp, -8, %o0
 call pointer_sink
 nop
 ret
 restore
.size stack_pointer,.-stack_pointer
.global pointer_value
.type pointer_value,@function
pointer_value:
 save %sp, -96, %sp
 mov %i0, %o0
 call value_sink
 nop
 ret
 restore
.size pointer_value,.-pointer_value
.global indirect_value
.type indirect_value,@function
indirect_value:
 save %sp, -112, %sp
 st %i0, [%fp-8]
 st %i1, [%fp-4]
 add %fp, -8, %o0
 jmpl %i2, %o7
 nop
 ret
 restore
.size indirect_value,.-indirect_value
.global value_reader
.type value_reader,@function
value_reader:
 save %sp, -96, %sp
 ld [%i0], %i1
 ld [%i0+4], %i0
 add %i0, %i1, %i0
 ret
 restore
.size value_reader,.-value_reader

.text
.global advanced
.type advanced,@function
advanced:
 save %sp, -96, %sp
 add %i0, 4, %i0
 ld [%i0-4], %i1
 ld [%i0], %i0
 add %i0, %i1, %i0
 ret
 restore
.size advanced,.-advanced
.global spilled
.type spilled,@function
spilled:
 save %sp, -104, %sp
 st %i0, [%fp-4]
 ld [%fp-4], %i0
 ld [%i0], %i1
 ld [%i0+4], %i0
 add %i0, %i1, %i0
 ret
 restore
.size spilled,.-spilled
.global updated
.type updated,@function
updated:
 save %sp, -96, %sp
 ld [%i0], %i1
 add %i1, 1, %i1
 st %i1, [%i0]
 ld [%i0+4], %i0
 add %i0, %i1, %i0
 ret
 restore
.size updated,.-updated
.global cursor
.type cursor,@function
cursor:
 save %sp, -96, %sp
 mov 0, %i2
 mov 2, %i3
.Lscan:
 ld [%i0+4], %i1
 add %i2, %i1, %i2
 ld [%i0], %i0
 subcc %i3, 1, %i3
 bne .Lscan
 nop
 mov %i2, %i0
 ret
 restore
.size cursor,.-cursor

.global indirect_pointer
.type indirect_pointer,@function
indirect_pointer:
 save %sp, -112, %sp
 st %i0, [%fp-8]
 st %i1, [%fp-4]
 add %fp, -8, %o0
 jmpl %i2, %o7
 nop
 ret
 restore
.size indirect_pointer,.-indirect_pointer

.global stack_value_reverse
.type stack_value_reverse,@function
stack_value_reverse:
 save %sp, -112, %sp
 st %i0, [%fp-8]
 st %i1, [%fp-4]
 add %fp, -8, %o0
 call value_sink_reverse
 nop
 ret
 restore
.size stack_value_reverse,.-stack_value_reverse
.global stack_pointer_reverse
.type stack_pointer_reverse,@function
stack_pointer_reverse:
 save %sp, -112, %sp
 st %i0, [%fp-8]
 st %i1, [%fp-4]
 add %fp, -8, %o0
 call pointer_sink_reverse
 nop
 ret
 restore
.size stack_pointer_reverse,.-stack_pointer_reverse

.text
.global stack_cursor
.type stack_cursor,@function
stack_cursor:
 save %sp, -112, %sp
 ld [%fp+92], %g1
 mov 2, %g2
 clr %i0
.Lstackscan:
 ld [%g1+4], %g3
 add %i0, %g3, %i0
 ld [%g1], %g1
 st %g1, [%fp+92]
 subcc %g2, 1, %g2
 bne .Lstackscan
 nop
 ret
 restore
.size stack_cursor,.-stack_cursor

.global collision_cursor
.type collision_cursor,@function
collision_cursor:
 save %sp, -96, %sp
 mov 0, %i2
 mov 2, %i3
.Lcollision:
 ld [%i0+4], %i1
 add %i2, %i1, %i2
 ld [%i0], %i0
 subcc %i3, 1, %i3
 bne .Lcollision
 nop
 mov %i2, %i0
 ret
 restore
.size collision_cursor,.-collision_cursor
