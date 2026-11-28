package pkg

public class TestCompoundBitwiseInfixOperand(bits: Int) {
   public final val bits: Int

   init {
      this.bits = bits
   }

   public infix fun mask(b: Int): Int {
      return this.bits and b
   }

   public fun mix(arr: IntArray, i: Int, b: Int) {
      arr[i] = arr[i] xor (this mask b)
   }

   public fun check(flags: BooleanArray, x: Any) {
      flags[0] = flags[0] and (x !is String)
   }
}
