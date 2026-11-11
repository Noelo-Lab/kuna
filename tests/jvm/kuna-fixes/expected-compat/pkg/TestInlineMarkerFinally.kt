package pkg

import kotlin.jvm.internal.InlineMarker
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestInlineMarkerFinally.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestInlineMarkerFinally.kt\npkg/TestInlineMarkerFinally\n*L\n1#1,32:1\n18#1,5:33\n*S KotlinDebug\n*F\n+ 1 TestInlineMarkerFinally.kt\npkg/TestInlineMarkerFinally\n*L\n26#1:33,5\n*E\n"])
public class TestInlineMarkerFinally {
   public final var hits: Int
      internal set

   public inline fun <T> takeIf(get: () -> T?, pred: (T) -> Boolean): T? {
      synchronized (this){} // $VF: monitorenter 

      var var8: Any
      run label51@{
         var var12: Any
         try {
            val var13: Boolean = true
            val var10000: Any = get()
            if (var10000 == null) {
               var8 = null
               return@label51
            }

            this.hits = this.hits + 1
            var12 = if (pred(var10000)) var10000 else null
         } finally {
            if (var10) {
               InlineMarker.finallyStart(1)
               // $VF: monitorexit
               InlineMarker.finallyEnd(1)
            }
         }

         InlineMarker.finallyStart(1)
         // $VF: monitorexit
         InlineMarker.finallyEnd(1)
         return (T)var12
         val var10: Boolean
      }

      InlineMarker.finallyStart(2)
      // $VF: monitorexit
      InlineMarker.finallyEnd(2)
      return (T)var8
   }

   public inline fun <R> pkg.TestInlineMarkerFinally.Lock.locked(block: () -> R): R {
      `$this$locked`.lock()

      try {
         return (R)block()
      } finally {
         InlineMarker.finallyStart(1)
         `$this$locked`.unlock()
         InlineMarker.finallyEnd(1)
      }
   }

   public inline fun <T> lockedOrNull(lock: pkg.TestInlineMarkerFinally.Lock, get: () -> T?): T? {
      lock.lock()

      var var12: Any
      run label44@{
         var var10: Any
         try {
            val var16: Boolean = true
            val var10000: Any = get()
            if (var10000 == null) {
               var12 = null
               return@label44
            }

            this.hits = this.hits + 1
            var10 = var10000
         } finally {
            if (var14) {
               InlineMarker.finallyStart(1)
               lock.unlock()
               InlineMarker.finallyEnd(1)
            }
         }

         InlineMarker.finallyStart(1)
         lock.unlock()
         InlineMarker.finallyEnd(1)
         return (T)var10
         val var14: Boolean
      }

      InlineMarker.finallyStart(2)
      lock.unlock()
      InlineMarker.finallyEnd(2)
      return (T)var12
   }

   public interface Lock {
      public abstract fun lock() {
      }

      public abstract fun unlock() {
      }
   }
}
