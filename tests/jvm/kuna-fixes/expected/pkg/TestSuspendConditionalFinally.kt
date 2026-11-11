package pkg

public class TestSuspendConditionalFinally {
   public suspend fun drain(sink: pkg.TestSuspendConditionalFinally.Sink, src: pkg.TestSuspendConditionalFinally.Source, consume: Boolean) {
      var cause: Throwable? = null

      try {
         val e: TestSuspendConditionalFinally.Cursor = src.open()

         while (e.hasMore()) {
            sink.put(e.take())
         }
      } catch (var12: Throwable) {
         cause = var12
         throw var12
      } finally {
         if (consume) {
            src.cancel(cause)
         }
      }

      return Unit.INSTANCE
   }

   public suspend fun sum(src: pkg.TestSuspendConditionalFinally.Source, cursor: pkg.TestSuspendConditionalFinally.Cursor, flag: Boolean): Int {
      try {
         var total: Int = 0

         while (cursor.hasMore()) {
            total += cursor.take()
         }

         return total
      } finally {
         src.closeIf(flag)
      }
   }

   public interface Cursor {
      public abstract suspend fun hasMore(): Boolean

      public abstract fun take(): Int
   }

   public interface Sink {
      public abstract suspend fun put(v: Int)
   }

   public interface Source {
      public abstract fun open(): pkg.TestSuspendConditionalFinally.Cursor

      public abstract fun cancel(cause: Throwable?)

      public abstract fun closeIf(flag: Boolean)
   }
}
