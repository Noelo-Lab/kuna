/* kuna `thunknames`: one-instruction `b <target>` thunks with no symbol of
 * their own. `.Lthunk` jumps to release_box, `.Lthunk2` jumps to `.Lthunk`,
 * and drop_box / drop_two call them with bl. release_box's prototype comes
 * from its DWARF. Rebuild: see README.md. */
long release_box(long *box, long by)
{
	*box -= by;
	return *box;
}

__asm__(
	"	.text\n"
	"	.p2align 2\n"
	".Lthunk:  b release_box\n"
	".Lthunk2: b .Lthunk\n"
	"	.globl drop_box\n"
	"	.type drop_box,%function\n"
	"drop_box:\n"
	"	stp x29, x30, [sp, #-16]!\n"
	"	mov x1, #1\n"
	"	bl .Lthunk\n"
	"	add x0, x0, #3\n"
	"	ldp x29, x30, [sp], #16\n"
	"	ret\n"
	"	.size drop_box, .-drop_box\n"
	"	.globl drop_two\n"
	"	.type drop_two,%function\n"
	"drop_two:\n"
	"	stp x29, x30, [sp, #-16]!\n"
	"	mov x1, #2\n"
	"	bl .Lthunk2\n"
	"	ldp x29, x30, [sp], #16\n"
	"	ret\n"
	"	.size drop_two, .-drop_two\n");
