package pkg

import kotlin.jvm.internal.SourceDebugExtension
import org.jetbrains.annotations.NotNull
import org.jetbrains.annotations.Nullable

@SourceDebugExtension(["SMAP\nTestSynchronizedBreakExit.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestSynchronizedBreakExit.kt\npkg/TestSynchronizedBreakExit\n*L\n1#1,67:1\n43#1:68\n46#1:69\n*S KotlinDebug\n*F\n+ 1 TestSynchronizedBreakExit.kt\npkg/TestSynchronizedBreakExit\n*L\n49#1:68\n55#1:69\n*E\n"])
public class TestSynchronizedBreakExit {
   public final var error: Int
      internal set

   public final var a: Boolean
      internal set

   public final var b: Boolean
      internal set

   public final var count: Int
      internal set

   @JvmField
   @NotNull
   public final var state: Any = TestSynchronizedBreakExit.Finishing(null, false)
      private set

   public fun close(code: Int): Boolean {
      var var5: Boolean
      run label53@{
         synchronized (this) {
            if (this.error != 0) {
               var5 = false
               return@label53
            }

            if (this.a && this.b) {
               return false
            }

            if (code < 0) {
               throw TestSynchronizedBreakExit.Oops()
            }

            this.error = code
         }

         val var7: Int = this.count++
         return true
      }

      // $VF: monitorexit
      return var5
   }

   public fun take(x: Int): Int {
      var var6: Byte
      run label67@{
         var var5: Byte
         run label66@{
            synchronized (this) {
               if (this.error != 0) {
                  var6 = -1
                  return@label67
               }

               if (x > 100) {
                  throw TestSynchronizedBreakExit.Oops()
               }

               if (this.a) {
                  if (this.b) {
                     var5 = 3
                     return@label66
                  }

                  return 2
               }

               this.count -= x
            }

            return 1
         }

         // $VF: monitorexit
         return var5
      }

      // $VF: monitorexit
      return var6
   }

   public fun add(state: Any, node: Any): Boolean {
      return state.hashCode() != node.hashCode()
   }

   public inline fun loopOnState(block: (Any) -> Unit): Nothing {
      while (true) {
         block(this.state)
      }
   }

   public fun register(onCancelling: Boolean, handler: Any, node: Any): Any? {
      val `this_$iv`: TestSynchronizedBreakExit = this

      val var10: Any
      val var15: Any
      while (true) {
         val state: Any = `this_$iv`.state
         var rootCause: Any = null
         var handle: Any = null
         if (onCancelling && `this_$iv`.state is TestSynchronizedBreakExit.Finishing) {
            var10 = `this_$iv`.state
            synchronized (`this_$iv`.state) {
               rootCause = (state as TestSynchronizedBreakExit.Finishing).rootCause
               if ((state as TestSynchronizedBreakExit.Finishing).rootCause == null
                  || handler is TestSynchronizedBreakExit.Handler && !(state as TestSynchronizedBreakExit.Finishing).isCompleting) {
                  if (!this.add(state, node)) {
                     continue
                  }

                  if (rootCause == null) {
                     var15 = node
                     break
                  }

                  handle = node
               }
            }
         }

         if (rootCause != null) {
            return handle
         }

         if (this.add(`this_$iv`.state, node)) {
            return node
         }
      }

      // $VF: monitorexit
      return var15
   }

   public class Finishing(rootCause: Any?, isCompleting: Boolean) {
      @JvmField
      @Nullable
      public final var rootCause: Any?
         private set

      @JvmField
      public final var isCompleting: Boolean
         private set

      init {
         this.rootCause = rootCause
         this.isCompleting = isCompleting
      }
   }

   public interface Handler

   public class Oops : RuntimeException
}
