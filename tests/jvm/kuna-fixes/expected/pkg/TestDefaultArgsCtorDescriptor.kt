package pkg

public class TestDefaultArgsCtorDescriptor {
   public fun sink(): pkg.TestDefaultArgsCtorDescriptor.Sink {
      return Sink()
   }

   public class Nested(size: Int = 8) {
      public final val size: Int

      init {
         this.size = size
      }
   }

   public inner class Sink(finished: Boolean = false, limit: Long = 16L) {
      public final val finished: Boolean
      public final val limit: Long

      init {
         this.finished = finished
         this.limit = limit
      }
   }
}
