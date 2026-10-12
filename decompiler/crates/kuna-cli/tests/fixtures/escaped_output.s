.text
.global escaped_output
.type escaped_output,@function
escaped_output:
 save %sp,-160,%sp
 add %fp,-48,%o0
 mov %i0,%o1
 call fill_words
 nop
 ld [%fp-40],%i0
 ret
 restore
.size escaped_output,.-escaped_output
.global fill_words
.type fill_words,@function
fill_words:
 mov 0,%g1
.loop:
 sll %g1,2,%g2
 add %o1,%g1,%g3
 st %g3,[%o0+%g2]
 inc %g1
 cmp %g1,10
 bl .loop
 nop
 retl
 nop
.size fill_words,.-fill_words
.global bounded_output
.type bounded_output,@function
bounded_output:
 save %sp,-160,%sp
 add %fp,-8,%o0
 call touch_word
 nop
 add %fp,-48,%o0
 mov %i0,%o1
 call fill_words
 nop
 ld [%fp-40],%i0
 ret
 restore
.size bounded_output,.-bounded_output
.global touch_word
.type touch_word,@function
touch_word:
 st %g0,[%o0]
 retl
 nop
.size touch_word,.-touch_word
.global separate_word
.type separate_word,@function
separate_word:
 save %sp,-160,%sp
 st %i0,[%fp-8]
 add %fp,-16,%o0
 call touch_word
 nop
 ld [%fp-8],%i0
 ret
 restore
.size separate_word,.-separate_word

.org 0x100, 0
.global overlapping_output
.type overlapping_output,@function
overlapping_output:
 save %sp, -208, %sp
 add %fp, -92, %l0
 mov 0, %l1
.fill:
 sll %l1, 2, %l2
 st %i0, [%l0+%l2]
 add %l1, 1, %l1
 cmp %l1, 11
 bl .fill
 nop
 add %l0, 4, %o0
 call observe_words
 nop
 mov %o0, %l3
 add %l0, 4, %o0
 add %fp, -48, %o1
 call copy_words
 nop
 ld [%fp-40], %i0
 add %i0, %l3, %i0
 ret
 restore
.size overlapping_output,.-overlapping_output
.global observe_words
.type observe_words,@function
observe_words:
 mov 0,%g1
 mov 0,%g2
.sum:
 sll %g2,2,%g3
 ld [%o0+%g3],%g3
 add %g1,%g3,%g1
 add %g2,1,%g2
 cmp %g2,10
 bl .sum
 nop
 retl
 mov %g1,%o0
.size observe_words,.-observe_words
.global copy_words
.type copy_words,@function
copy_words:
 mov 0,%g1
.copy:
 sll %g1,2,%g2
 ld [%o0+%g2],%g3
 add %g3,%g1,%g3
 st %g3,[%o1+%g2]
 add %g1,1,%g1
 cmp %g1,10
 bl .copy
 nop
 retl
 nop
.size copy_words,.-copy_words

.org 0x200, 0
.text
.global looping_output
.type looping_output,@function
looping_output:
 save %sp, -208, %sp
 add %fp, -92, %l0
 mov 0, %l1
.fill_loop:
 sll %l1, 2, %l2
 st %i0, [%l0+%l2]
 add %l1, 1, %l1
 cmp %l1, 13
 bl .fill_loop
 nop
 add %l0, 4, %o0
 call observe_words
 nop
 mov %o0, %l3
 mov 0, %l4
 mov 0, %l5
 add %fp, -48, %l7
.again:
 add %l0, 4, %o0
 mov %l7, %o1
 call copy_words
 nop
 ld [%fp-40], %l6
 add %l5, %l6, %l5
 add %l7, 4, %l7
 add %l4, 1, %l4
 cmp %l4, 2
 bl .again
 nop
 add %l5, %l3, %i0
 ret
 restore
.size looping_output,.-looping_output

.org 0x300, 0
.global lower_scalar_output
.type lower_scalar_output,@function
lower_scalar_output:
 save %sp, -160, %sp
 add %fp, -52, %o0
 call touch_word
 nop
 st %i0, [%fp-52]
 add %fp, -48, %o0
 mov %i0, %o1
 call fill_words
 nop
 ld [%fp-40], %l0
 ld [%fp-52], %i0
 add %i0, %l0, %i0
 ret
 restore
.size lower_scalar_output,.-lower_scalar_output

.org 0x380, 0
.global parent_tail_scalar
.type parent_tail_scalar,@function
parent_tail_scalar:
 save %sp, -208, %sp
 add %fp, -92, %l0
 mov 0, %l1
.parent_fill:
 sll %l1, 2, %l2
 st %i0, [%l0+%l2]
 add %l1, 1, %l1
 cmp %l1, 11
 bl .parent_fill
 nop
 st %i0, [%fp-44]
 add %l0, 4, %o0
 add %fp, -48, %o1
 call copy_words
 nop
 ld [%fp-40], %l2
 ld [%fp-44], %i0
 add %i0, %l2, %i0
 ret
 restore
.size parent_tail_scalar,.-parent_tail_scalar

.org 0x400, 0
.global duplicate_arg_output
.type duplicate_arg_output,@function
duplicate_arg_output:
 save %sp, -160, %sp
 add %fp, -48, %o0
 mov %o0, %o1
 call mixed_words
 nop
 ld [%fp-40], %i0
 ret
 restore
.size duplicate_arg_output,.-duplicate_arg_output

.org 0x440, 0
.global mixed_words
.type mixed_words,@function
mixed_words:
 retl
 nop
.size mixed_words,.-mixed_words
