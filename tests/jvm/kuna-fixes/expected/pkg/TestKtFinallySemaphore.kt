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
      synchronized (this){} // $VF: monitorenter 
      var var8: Boolean = false /* VF: Semaphore variable */

      var var6: Any
      run label51@{
         var var10: Any
         try {
            var8 = true
            val var10000: Any = this.first
            if (var10000 == null) {
               var6 = null
               var8 = false
               return@label51
            }

            var10 = if (p(var10000)) var10000 else null
            var8 = false
         } finally {
            if (var8) {
               InlineMarker.finallyStart(1)
               // $VF: monitorexit
               InlineMarker.finallyEnd(1)
            }
         }

         InlineMarker.finallyStart(1)
         // $VF: monitorexit
         InlineMarker.finallyEnd(1)
         return var10
      }

      InlineMarker.finallyStart(2)
      // $VF: monitorexit
      InlineMarker.finallyEnd(2)
      return var6
   }

   public inline fun firstNull(src: pkg.TestKtFinallySemaphore.Src): Int {
      val `$this$consume$iv`: TestKtFinallySemaphore.Src = src
      var `cause$iv`: Throwable = null
      var var13: Boolean = false /* VF: Semaphore variable */

      var var10: Int
      run label66@{
         try {
            var13 = true
            val `e$iv`: TestKtFinallySemaphore.Src = `$this$consume$iv`

            do {
               if (`e$iv`.next() == null) {
                  var10 = this.count
                  var13 = false
                  return@label66
               }

               this.count = this.count + 1
            } while (`e$iv`.more())

            var13 = false
         } catch (var14: Throwable) {
            `cause$iv` = var14
            throw var14
         } finally {
            if (var13) {
               InlineMarker.finallyStart(1)
               src.cancel(`cause$iv`)
               InlineMarker.finallyEnd(1)
            }
         }

         InlineMarker.finallyStart(1)
         src.cancel(null)
         InlineMarker.finallyEnd(1)
         return -1
      }

      InlineMarker.finallyStart(2)
      src.cancel(null)
      InlineMarker.finallyEnd(2)
      return var10
   }

   public interface Src {
      public abstract fun more(): Boolean

      public abstract fun next(): Any?

      public abstract fun cancel(cause: Throwable?)
   }
}
