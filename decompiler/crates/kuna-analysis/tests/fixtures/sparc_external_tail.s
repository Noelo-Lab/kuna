.text
.global external_tail
.type external_tail,@function
external_tail:
 add %o0,1,%g2
 sll %g2,16,%o0
 sethi %hi(provider),%g1
 jmpl %g1+%lo(provider),%g0
 sra %o0,16,%o0
.size external_tail,.-external_tail
