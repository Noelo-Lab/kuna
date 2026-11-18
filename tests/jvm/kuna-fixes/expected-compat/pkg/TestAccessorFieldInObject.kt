package pkg

public class TestAccessorFieldInObject {
   public final var count: Int
      public final set(v) {
         object : Runnable {
            public open fun run() {
               TestAccessorFieldInObject.this.count = v
            }
         }.run()
      }


   public final var label: String = "a"
      public final get() {
         object : Runnable {
            public open fun run() {
               System.out.println(TestAccessorFieldInObject.this.label)
            }
         }.run()
         return this.label
      }

      internal set

   private final var hits: Int

   public fun hit(): Runnable {
      return object : Runnable {
         public open fun run() {
            TestAccessorFieldInObject.this.hits = TestAccessorFieldInObject.this.hits + 1
         }
      }
   }
}
