/* PowerPC64 ELFv1 functions behind descriptors whose dynamic symbols state the
   size of the code. Every leaf opens with the same three instructions, which also
   occur inside `sample`, `outer` and `entered`, behind `sample` in a routine that
   has no symbol, and behind the 8 bytes of `dotted`, whose symbol states the 24
   bytes of its descriptor as the dot-symbol ABI does. */
.abiversion 1
.macro descriptor name
.section ".opd","aw"
.balign 8
.globl \name
.type \name,@function
\name:
.quad .Lentry\@,.TOC.@tocbase,0
.text
.balign 4
.Lentry\@:
.L\name:
.endm

.macro prefix n
std 4,-8(1)
addis 5,3,1
addi 3,5,\n
.endm

.macro leaf n
descriptor leaf\n
prefix \n
blr
.size leaf\n,16
.endm

descriptor driver
mflr 0
stdu 1,-128(1)
std 0,16(1)
.irp n,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24
bl .Lleaf\n
.endr
bl .Lcalled
ld 0,16(1)
addi 1,1,128
mtlr 0
blr
.size driver,.-.Ldriver

.irp n,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24
leaf \n
.endr

descriptor sample
cmpdi 3,0
beq 1f
nop
prefix 7
1:
blr
.size sample,.-.Lsample

/* no symbol and no reference */
prefix 11
blr

/* a second symbol names the code inside */
descriptor outer
cmpdi 3,0
beq 1f
nop
.section ".opd","aw"
.balign 8
.globl inner
.type inner,@function
inner:
.quad .Linner,.TOC.@tocbase,0
.text
.Linner:
prefix 12
1:
blr
.size inner,.-.Linner
.size outer,.-.Louter

/* driver calls the code inside */
descriptor entered
cmpdi 3,0
beq 1f
nop
.Lcalled:
prefix 13
1:
blr
.size entered,.-.Lentered

descriptor dotted
li 3,1
blr
.size dotted,24
prefix 14
blr

leaf 25
