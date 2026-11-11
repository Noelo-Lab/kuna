package pkg

public class TestSynchronizedAccessor {
   public final var a: Int = 1
      internal set

   public final var b: Int = 2
      internal set

   public final var c: Int = 3
      public final get() {
         return this.c + 1
      }

      public final set(value) {
         this.c = value * 2
      }


   public final val d: Int
      public final get() {
         return this.a + this.b
      }

}
