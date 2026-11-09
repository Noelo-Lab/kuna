package pkg

import kotlin.jvm.internal.InlineMarker
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtFinallySemaphore.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtFinallySemaphore.kt\npkg/TestKtFinallySemaphore\n*L\n1#1,40:1\n14#1,10:41\n*S KotlinDebug\n*F\n+ 1 TestKtFinallySemaphore.kt\npkg/TestKtFinallySemaphore\n*L\n31#1:41,10\n*E\n"])
public class TestKtFinallySemaphore {
   public final var first: Any?
      internal set

   public final var count: Int
      internal set

   public inline fun pkg.TestKtFinallySemaphore.Src.consume(block: (pkg.TestKtFinallySemaphore.Src) -> Unit) {
      var cause: java.lang.Throwable = null

      try {
         block(`$this$consume`)
      } catch (var8: java.lang.Throwable) {
         cause = var8
         throw var8
      } finally {
         InlineMarker.finallyStart(1)
         `$this$consume`.cancel(cause)
         InlineMarker.finallyEnd(1)
      }
   }

   public inline fun removeIf(p: (Any) -> Boolean): Any? {
      synchronized (this){} // $VF: monitorenter 

      var var6: Any
      run label51@{
         var var10: Any
         try {
            val var11: Boolean = true
            val var10000: Any = this.first
            if (var10000 == null) {
               var6 = null
               return@label51
            }

            var10 = if (p(var10000)) var10000 else null
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
         val var8: Boolean
      }

      InlineMarker.finallyStart(2)
      // $VF: monitorexit
      InlineMarker.finallyEnd(2)
      return var6
   }

   public inline fun firstNull(src: pkg.TestKtFinallySemaphore.Src): Int {
      val `$this$consume$iv`: TestKtFinallySemaphore.Src = src
      var `cause$iv`: java.lang.Throwable = null

      var var10: Int
      run label66@{
         try {
            val var16: Boolean = true
            val `e$iv`: TestKtFinallySemaphore.Src = `$this$consume$iv`

            do {
               if (`e$iv`.next() == null) {
                  var10 = this.count
                  return@label66
               }

               this.count = this.count + 1
            } while (`e$iv`.more())
         } catch (var14: java.lang.Throwable) {
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
         val var13: <unknown>
      }

      InlineMarker.finallyStart(2)
      src.cancel(null)
      InlineMarker.finallyEnd(2)
      return var10
   }

   public interface Src {
      public abstract fun more(): Boolean {
      }

      public abstract fun next(): Any? {
      }

      public abstract fun cancel(cause: Throwable?) {
      }
   }
}
