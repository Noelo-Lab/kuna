package pkg

public interface TestInterfaceCompanionField {
   public abstract fun id(): Int

   public companion object Key {
      public fun twice(n: Int): Int {
         return n * 2
      }
   }
}
