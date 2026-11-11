package pkg

class TestOperatorPrecedence {
    fun orShl(x: Int, k: Int): Int = x or (1 shl k)
    fun utf8(b1: Int, b2: Int): Int = b2 xor 0x0f80 xor (b1 shl 6)
    fun orAnd(a: Int, b: Int, c: Int): Int = a or (b and c)
    fun xorAnd(a: Int, b: Int, c: Int): Int = a xor (b and c)
    fun andUshr(a: Int, b: Int): Int = a and (b ushr 3)
    fun orShr(a: Long, b: Long): Long = a or (b shr 7)
    fun shlOr(a: Int, b: Int): Int = (a shl 1) or b
    fun orThenShl(a: Int, b: Int): Int = (a or b) shl 1
    fun sumShl(a: Int, b: Int): Int = (a + b) shl 2
    fun mixed(x: Int, a: Int, b: Int): Boolean = (x and (a shl b)) != 0 && (x or 4) > 3
    fun andEq(ok: Boolean, flags: Int): Boolean = ok and (flags != 0)
    fun eqAnd(a: Int, b: Int, c: Boolean): Boolean = (a == b) and c
    fun ltXor(a: Int, b: Int, c: Boolean): Boolean = (a < b) xor c
    fun isOr(x: Any, y: Boolean): Boolean = (x is TestOperatorPrecedence) or y
    fun elvisAnd(x: Boolean?, a: Boolean, b: Boolean): Boolean = x ?: (a && b)
    fun elvisEq(x: Boolean?, a: Int, b: Int): Boolean = x ?: (a == b)
    fun elvisIs(x: Boolean?, y: Any): Boolean = x ?: (y is TestOperatorPrecedence)
    infix fun Int.with(other: Int): Int = this * 31 + other
    fun nestedInfix(a: Int, b: Int, c: Int): Int = a with (b with c)
    fun infixOfOr(a: Int, b: Int, c: Int): Int = a with (b or c)
    fun orOfInfix(a: Int, b: Int, c: Int): Int = a or (b with c)
}
