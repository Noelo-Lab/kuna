package pkg

public object TestObjectInstanceLocal {
   private final val table: IntArray = IntArray(8)
   public final val first: Int = TestObjectInstanceLocal.compute(3)

   private fun compute(x: Int): Int {
      return x * 2
   }

   private fun register(i: Int) {
      table[i] = i * i
   }

   public fun get(i: Int): Int {
      return table[i]
   }

   init {
      repeat(8) { i ->
         TestObjectInstanceLocal.register(i)
      }
   }
}
