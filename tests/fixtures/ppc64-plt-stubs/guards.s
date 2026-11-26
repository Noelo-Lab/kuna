# `release` is a whole function shaped like a Power10 stub: it loads free's
# PLT slot and branches to it. `hidden` has the same shape but loads strlen's
# GOT entry. `outer` calls free through a real linker stub, then `release`
# directly and `hidden`.
	.abiversion 2
	.text
	.globl release
	.type release,@function
release:
.Lrelease:
	pld 12, free@plt@pcrel
	mtctr 12
	bctr
	.size release, .-release

	.p2align 4
	.globl outer
	.type outer,@function
outer:
	mflr 0
	std 31,-8(1)
	std 0,16(1)
	stdu 1,-48(1)
	mr 31,3
	bl free@notoc
	mr 3,31
	bl .Lrelease
	mr 3,31
	bl hidden@notoc
	addi 1,1,48
	ld 0,16(1)
	ld 31,-8(1)
	mtlr 0
	blr
	.size outer, .-outer

	.p2align 4
	.type hidden,@function
hidden:
	pld 12, strlen@got@pcrel
	mtctr 12
	bctr
	.size hidden, .-hidden
