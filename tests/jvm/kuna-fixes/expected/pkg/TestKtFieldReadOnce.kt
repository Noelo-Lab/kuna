package pkg

public class TestKtFieldReadOnce {
   @Volatile
   private final var worker: Thread? = null

   public final var head: pkg.TestKtFieldReadOnce.Segment? = null

   public fun worker(): Thread {
      var var10000: Thread? = this.worker
      if (var10000 == null) {
         var10000 = Thread()
      }

      return var10000
   }

   public fun first(): Int {
      val var10000: TestKtFieldReadOnce.Segment? = this.head
      if (var10000 == null) {
         throw IllegalStateException("empty")
      } else {
         return var10000.pos
      }
   }

   public fun total(): Int {
      var n: Int = 0
      val var10000: TestKtFieldReadOnce.Segment? = this.head
      if (var10000 != null) {
         val h: TestKtFieldReadOnce.Segment = var10000
         n += h.pos
         var s: TestKtFieldReadOnce.Segment = h.next!!

         while (s !== h) {
            n += s.pos
            s = s.next!!
         }
      }

      return n
   }

   public class Segment(next: pkg.TestKtFieldReadOnce.Segment?, pos: Int) {
      public final var next: pkg.TestKtFieldReadOnce.Segment?
      public final var pos: Int

      init {
         this.next = next
         this.pos = pos
      }
   }
}
