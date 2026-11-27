package pkg

public class TestAccessorFieldInObject {
   public final var count: Int = 0
      public set(v) {
         object : Runnable {
            public open fun run() {
               field = v
            }
         }.run()
      }


   public final var label: String = "a"
      public get() {
         object : Runnable {
            public open fun run() {
               System.out.println(field)
            }
         }.run()
         return field
      }


   private final var hits: Int = 0

   public fun hit(): Runnable {
      return object : Runnable {
         public open fun run() {
            this@TestAccessorFieldInObject.hits = this@TestAccessorFieldInObject.hits + 1
         }
      }
   }
}
