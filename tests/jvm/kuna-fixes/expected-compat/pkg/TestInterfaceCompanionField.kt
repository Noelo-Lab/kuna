package pkg

public interface TestInterfaceCompanionField {
   @JvmStatic
   TestInterfaceCompanionField.Key Key = TestInterfaceCompanionField.Key.$$INSTANCE;

   public abstract fun id(): Int {
   }

   public companion object Key {
      public fun twice(n: Int): Int {
         return n * 2
      }
   }
}
