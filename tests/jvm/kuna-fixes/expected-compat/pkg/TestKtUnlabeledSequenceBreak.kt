package pkg

public class TestKtUnlabeledSequenceBreak {
   private final val lock: Any = Any()

   public final var folded: Any?
      internal set

   public final var closed: Boolean
      internal set

   public final var space: Long
      internal set

   public final var written: Long
      internal set

   public fun write(n: Long): Any? {
      var count: Long = 0L
      count = n
      var found: Any = null
      synchronized (this.lock) {
         if (this.closed) {
            throw IllegalStateException("closed")
         }

         while (count > 0L) {
            if (this.folded != null) {
               found = this.folded
               return@run
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
