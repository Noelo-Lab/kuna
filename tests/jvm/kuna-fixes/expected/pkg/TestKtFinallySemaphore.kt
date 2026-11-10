package pkg

import kotlin.jvm.internal.InlineMarker
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtFinallySemaphore.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtFinallySemaphore.kt\npkg/TestKtFinallySemaphore\n*L\n1#1,40:1\n14#1,10:41\n*S KotlinDebug\n*F\n+ 1 TestKtFinallySemaphore.kt\npkg/TestKtFinallySemaphore\n*L\n31#1:41,10\n*E\n"])
public class TestKtFinallySemaphore {
   public final var first: Any? = null
   public final var count: Int = 0

   public inline fun pkg.TestKtFinallySemaphore.Src.consume(block: (pkg.TestKtFinallySemaphore.Src) -> Unit) {
      var cause: Throwable = null

      try {
         block(this)
      } catch (var8: Throwable) {
         cause = var8
         throw var8
      } finally {
         InlineMarker.finallyStart(1)
         this.cancel(cause)
         InlineMarker.finallyEnd(1)
      }
   }

   public inline fun removeIf(p: (Any) -> Boolean): Any? {
      synchronized (this) {
         val var10000: Any = this.first
         if (var10000 == null) {
            return null
         } else {
            return if (p(var10000)) var10000 else null
         }
      }
   }

   public inline fun firstNull(src: pkg.TestKtFinallySemaphore.Src): Int {
      val `$this$consume$iv`: TestKtFinallySemaphore.Src = src
      var `cause$iv`: Throwable = null

      try {
         val `e$iv`: TestKtFinallySemaphore.Src = `$this$consume$iv`

         do {
            if (`e$iv`.next() == null) {
               return this.count
            }

            this.count = this.count + 1
         } while (`e$iv`.more())
      } catch (var13: Throwable) {
         `cause$iv` = var13
         throw var13
      } finally {
         InlineMarker.finallyStart(1)
         src.cancel(`cause$iv`)
         InlineMarker.finallyEnd(1)
      }

      return -1
   }

   public interface Src {
      public abstract fun more(): Boolean

      public abstract fun next(): Any?

      public abstract fun cancel(cause: Throwable?)
   }
}
