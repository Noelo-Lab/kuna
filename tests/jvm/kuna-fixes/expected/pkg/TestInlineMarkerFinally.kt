package pkg

import kotlin.jvm.internal.InlineMarker
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestInlineMarkerFinally.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestInlineMarkerFinally.kt\npkg/TestInlineMarkerFinally\n*L\n1#1,32:1\n18#1,5:33\n*S KotlinDebug\n*F\n+ 1 TestInlineMarkerFinally.kt\npkg/TestInlineMarkerFinally\n*L\n26#1:33,5\n*E\n"])
public class TestInlineMarkerFinally {
   public final var hits: Int = 0

   public inline fun <T> takeIf(get: () -> T?, pred: (T) -> Boolean): T? {
      synchronized (this) {
         val var10000: Any = get()
         if (var10000 == null) {
            return null
         } else {
            this.hits = this.hits + 1
            return (if (pred(var10000)) var10000 else null) as T
         }
      }
   }

   public inline fun <R> pkg.TestInlineMarkerFinally.Lock.locked(block: () -> R): R {
      this.lock()

      try {
         return block() as R
      } finally {
         InlineMarker.finallyStart(1)
         this.unlock()
         InlineMarker.finallyEnd(1)
      }
   }

   public inline fun <T> lockedOrNull(lock: pkg.TestInlineMarkerFinally.Lock, get: () -> T?): T? {
      lock.lock()

      try {
         val var10000: Any = get()
         if (var10000 == null) {
            return null
         } else {
            this.hits = this.hits + 1
            return var10000 as T
         }
      } finally {
         InlineMarker.finallyStart(1)
         lock.unlock()
         InlineMarker.finallyEnd(1)
      }
   }

   public interface Lock {
      public abstract fun lock()

      public abstract fun unlock()
   }
}
