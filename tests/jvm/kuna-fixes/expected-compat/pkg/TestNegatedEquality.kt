package pkg

public class TestNegatedEquality(name: String, tag: Any?) {
   private final val name: String
   private final val tag: Any?

   init {
      this.name = name
      this.tag = tag
   }

   public fun differs(other: TestNegatedEquality): Boolean {
      return this.name == other.name xor true
   }

   public fun stored(other: Any?): Boolean {
      return this.tag == other xor true
   }

   public fun guard(a: String?, b: String?): Int {
      return if (a == b xor true) 1 else 2
   }

   public fun either(a: String?, b: String?, c: Boolean): Boolean {
      return c or a == b xor true
   }

   public fun compareWith(a: String?, b: String?, c: Boolean): Boolean {
      return (a == b xor true) == c
   }

   public fun numbers(i: Int, j: Int): Boolean {
      return i == j xor true
   }

   public fun same(a: Any, b: Any): Boolean {
      return a === b xor true
   }
}
