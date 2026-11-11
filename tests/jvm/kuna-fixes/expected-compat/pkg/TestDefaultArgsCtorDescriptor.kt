package pkg

public class TestDefaultArgsCtorDescriptor {
   public fun sink(): pkg.TestDefaultArgsCtorDescriptor.Sink {
      return TestDefaultArgsCtorDescriptor.Sink(false, 0L, 3, null)
   }

   public class Nested(size: Int = 8) {
      public final val size: Int

      init {
         this.size = size
      }

      fun Nested() {
         this(0, 1, null)
      }
   }

   public inner class Sink(finished: Boolean = ..., limit: Long = ...) {
      public final val finished: Boolean
      public final val limit: Long

      init {
         this.finished = finished
         this.limit = limit
      }
   }
}
