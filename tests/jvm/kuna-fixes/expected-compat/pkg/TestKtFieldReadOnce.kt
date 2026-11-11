package pkg

public class TestKtFieldReadOnce {
   private final var worker: Thread?

   public final var head: pkg.TestKtFieldReadOnce.Segment?
      internal set

   public fun worker(): Thread {
      var var10000: Thread = this.worker
      if (this.worker == null) {
         var10000 = Thread()
      }

      return var10000
   }

   public fun first(): Int {
      if (this.head == null) {
         throw IllegalStateException("empty")
      } else {
         return this.head.pos
      }
   }

   public fun total(): Int {
      var n: Int = 0
      if (this.head != null) {
         val h: TestKtFieldReadOnce.Segment = this.head
         n += this.head.pos
         var var10000: TestKtFieldReadOnce.Segment = h.next
         var s: TestKtFieldReadOnce.Segment = var10000

         while (s != h) {
            n += s.pos
            var10000 = s.next
            s = var10000
         }
      }

      return n
   }

   public class Segment(next: pkg.TestKtFieldReadOnce.Segment?, pos: Int) {
      public final var next: pkg.TestKtFieldReadOnce.Segment?
         internal set

      public final var pos: Int
         internal set

      init {
         this.next = next
         this.pos = pos
      }
   }
}
