package pkg

public class TestNegatedEquality(name: String, tag: Any?) {
   private final val name: String
   private final val tag: Any?

   init {
      this.name = name
      this.tag = tag
   }

   public fun differs(other: TestNegatedEquality): Boolean {
      return this.name != other.name
   }

   public fun stored(other: Any?): Boolean {
      return this.tag != other
   }

   public fun guard(a: String?, b: String?): Int {
      return if (a != b) 1 else 2
   }

   public fun either(a: String?, b: String?, c: Boolean): Boolean {
      return c or (a != b)
   }

   public fun compareWith(a: String?, b: String?, c: Boolean): Boolean {
      return (a != b) == c
   }

   public fun numbers(i: Int, j: Int): Boolean {
      return i != j
   }

   public fun same(a: Any, b: Any): Boolean {
      return a !== b
   }
}
