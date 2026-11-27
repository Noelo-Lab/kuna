package pkg

public class TestKtUnlabeledSequenceBreak {
   private final val lock: Any = Any()
   public final var folded: Any? = null
   public final var closed: Boolean = false
   public final var space: Long = 0L
   public final var written: Long = 0L

   public fun write(n: Long): Any? {
      var count: Long = 0L
      count = n
      var found: Any? = null
      synchronized (this.lock) {
         if (this.closed) {
            throw IllegalStateException("closed")
         }

         while (count > 0L) {
            val var10000: Any? = this.folded
            if (var10000 != null) {
               found = var10000
               break
            } else if (this.space == 0L) {
               this.space = 1L
            } else {
               val step: Long = Math.min(this.space, count)
               this.written += step
               count -= step
            }
         }
      }

      if (found != null) {
         this.written = count
      }

      return found
   }
}
