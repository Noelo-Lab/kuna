package pkg

public object TestObjectInstanceLocal {
   @JvmStatic
   public TestObjectInstanceLocal INSTANCE;
   private final val table: IntArray = IntArray(8)
   public final val first: Int

   private fun compute(x: Int): Int {
      return x * 2
   }

   private fun register(i: Int) {
      table[i] = i * i
   }

   public fun get(i: Int): Int {
      return table[i]
   }

   @JvmStatic
   fun {
      val var0: TestObjectInstanceLocal = TestObjectInstanceLocal()
      INSTANCE = var0
      first = var0.compute(3)

      repeat(7) { i ->
         var0.register(i)
      }
   }
}
