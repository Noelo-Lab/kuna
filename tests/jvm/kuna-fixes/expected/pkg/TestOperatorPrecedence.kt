package pkg

public class TestOperatorPrecedence {
   public fun orShl(x: Int, k: Int): Int {
      return x or (1 shl k)
   }

   public fun utf8(b1: Int, b2: Int): Int {
      return b2 xor 3968 xor (b1 shl 6)
   }

   public fun orAnd(a: Int, b: Int, c: Int): Int {
      return a or (b and c)
   }

   public fun xorAnd(a: Int, b: Int, c: Int): Int {
      return a xor (b and c)
   }

   public fun andUshr(a: Int, b: Int): Int {
      return a and (b ushr 3)
   }

   public fun orShr(a: Long, b: Long): Long {
      return a or (b shr 7)
   }

   public fun shlOr(a: Int, b: Int): Int {
      return a shl 1 or b
   }

   public fun orThenShl(a: Int, b: Int): Int {
      return (a or b) shl 1
   }

   public fun sumShl(a: Int, b: Int): Int {
      return a + b shl 2
   }

   public fun mixed(x: Int, a: Int, b: Int): Boolean {
      return (x and (a shl b)) != 0 && (x or 4) > 3
   }

   public fun andEq(ok: Boolean, flags: Int): Boolean {
      return ok and (flags != 0)
   }

   public fun eqAnd(a: Int, b: Int, c: Boolean): Boolean {
      return (a == b) and c
   }

   public fun ltXor(a: Int, b: Int, c: Boolean): Boolean {
      return (a < b) xor c
   }

   public fun isOr(x: Any, y: Boolean): Boolean {
      return (x is TestOperatorPrecedence) or y
   }

   public fun elvisAnd(x: Boolean?, a: Boolean, b: Boolean): Boolean {
      return x ?: (a && b)
   }

   public fun elvisEq(x: Boolean?, a: Int, b: Int): Boolean {
      return x ?: (a == b)
   }

   public fun elvisIs(x: Boolean?, y: Any): Boolean {
      return x ?: (y is TestOperatorPrecedence)
   }

   public infix fun Int.with(other: Int): Int {
      return this * 31 + other
   }

   public fun nestedInfix(a: Int, b: Int, c: Int): Int {
      return a with (b with c)
   }

   public fun infixOfOr(a: Int, b: Int, c: Int): Int {
      return a with (b or c)
   }

   public fun orOfInfix(a: Int, b: Int, c: Int): Int {
      return a or (b with c)
   }
}
