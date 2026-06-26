/*
 * AIF gap-walk fixture (stripped x86-64): a hidden function reachable ONLY through
 * a data/indirect (function-pointer-table) path. Entry-discovery + funcsyms miss it
 * (stripped + no static CALL reaches it + no `.eh_frame` FDE), but the
 * AggressiveInstructionFinder gap-walk finds it: its function-start fingerprint
 * matches the visible handlers' (>= threshold) and it disassembles into a valid
 * subroutine ending in a clean RET.
 *
 * Build: gcc -O0 -no-pie -fno-pic -fcf-protection=none -fno-stack-protector \
 *            -fno-asynchronous-unwind-tables -fno-unwind-tables \
 *            -o aif_gap_x86_64 aif_gap_x86_64.c && strip aif_gap_x86_64
 *
 *  - h00..h23 (24 handlers): ordinary functions, all CALLED DIRECTLY from main, so
 *    the recursive-descent Listing reaches them (so function_count >= 20, clearing
 *    Ghidra's MINIMUM_FUNCTION_COUNT=20). They share an IDENTICAL multi-insn
 *    prologue (push rbp; mov rsp,rbp; mov [rbp-..],edi; ...) differing only in an
 *    operand immediate, so the function-start fingerprint histogram has one bucket
 *    with >= the acceptance threshold (4) of identical entries.
 *  - hidden_handler: NEVER called directly. Its address is taken ONLY into a const
 *    function-pointer `table`; main indexes the table with a volatile (unfoldable)
 *    value and calls indirectly -> the static walk sees only a CALLIND with no
 *    resolvable target, so hidden_handler's bytes stay an UNDEFINED gap. It shares
 *    the same prologue (fingerprint match) and is a valid >2-insn subroutine with a
 *    clean RET.
 */

volatile int sel;   /* defeats constant folding of the table index */

#define H(n, k) \
    __attribute__((noinline)) int h##n(int x) { int y = x + (k); y = y * 2; return y + (n); }

H(0,10) H(1,11) H(2,12) H(3,13) H(4,14) H(5,15) H(6,16) H(7,17)
H(8,18) H(9,19) H(10,20) H(11,21) H(12,22) H(13,23) H(14,24) H(15,25)
H(16,26) H(17,27) H(18,28) H(19,29) H(20,30) H(21,31) H(22,32) H(23,33)

/* The hidden function: same prologue/body shape, valid multi-insn subroutine,
 * only reachable via the indirect/data path below. */
__attribute__((noinline)) int hidden_handler(int x) {
    int y = x + 64;
    y = y * 2;
    return y + 9;
}

typedef int (*handler_fn)(int);

/* const function-pointer table; hidden_handler appears ONLY here (last slot). */
static handler_fn const table[2] = { h0, hidden_handler };

int main(void) {
    int s = sel;                  /* volatile read: opaque index */
    handler_fn f = table[s & 1];
    int r = f(s);                 /* indirect call -> CALLIND, no static target */
    /* call every handler directly so all 24 are reached + fingerprinted */
    int (*const direct[24])(int) = {
        h0,h1,h2,h3,h4,h5,h6,h7,h8,h9,h10,h11,
        h12,h13,h14,h15,h16,h17,h18,h19,h20,h21,h22,h23
    };
    /* defeat the compiler turning this into a loop-over-table by calling each
     * with a distinct direct call */
    r += h0(s); r += h1(s); r += h2(s); r += h3(s); r += h4(s); r += h5(s);
    r += h6(s); r += h7(s); r += h8(s); r += h9(s); r += h10(s); r += h11(s);
    r += h12(s); r += h13(s); r += h14(s); r += h15(s); r += h16(s); r += h17(s);
    r += h18(s); r += h19(s); r += h20(s); r += h21(s); r += h22(s); r += h23(s);
    (void)direct;
    return r;
}
