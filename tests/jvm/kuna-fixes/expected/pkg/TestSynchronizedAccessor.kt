package pkg

public class TestSynchronizedAccessor {
   @get:Synchronized
   public final var a: Int = 1

   @get:Synchronized
   @set:Synchronized
   public final var b: Int = 2

   @get:Synchronized
   @set:Synchronized
   public final var c: Int = 3
      public get() {
         return field + 1
      }

      public set(value) {
         field = value * 2
      }


   @get:Synchronized
   public final val d: Int
      public get() {
         return this.a + this.b
      }

}
