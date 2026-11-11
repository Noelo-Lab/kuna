package pkg

import kotlin.jvm.internal.SourceDebugExtension
import org.jetbrains.annotations.NotNull
import org.jetbrains.annotations.Nullable

@SourceDebugExtension(["SMAP\nTestSynchronizedBreakExit.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestSynchronizedBreakExit.kt\npkg/TestSynchronizedBreakExit\n*L\n1#1,67:1\n43#1:68\n46#1:69\n*S KotlinDebug\n*F\n+ 1 TestSynchronizedBreakExit.kt\npkg/TestSynchronizedBreakExit\n*L\n49#1:68\n55#1:69\n*E\n"])
public class TestSynchronizedBreakExit {
   public final var error: Int = 0
   public final var a: Boolean = false
   public final var b: Boolean = false
   public final var count: Int = 0

   @JvmField
   @NotNull
   public final var state: Any = TestSynchronizedBreakExit.Finishing(null, false)

   public fun close(code: Int): Boolean {
      synchronized (this) {
         if (this.error != 0) {
            return false
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

   public fun take(x: Int): Int {
      synchronized (this) {
         if (this.error != 0) {
            return -1
         }

         if (x > 100) {
            throw TestSynchronizedBreakExit.Oops()
         }

         if (this.a) {
            if (this.b) {
               return 3
            }

            return 2
         }

         this.count -= x
      }

      return 1
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

      while (true) {
         val state: Any = `this_$iv`.state
         var rootCause: Any = null
         var handle: Any = null
         if (onCancelling && state is TestSynchronizedBreakExit.Finishing) {
            run label60@{
               synchronized (state) {
                  rootCause = (state as TestSynchronizedBreakExit.Finishing).rootCause
                  if ((state as TestSynchronizedBreakExit.Finishing).rootCause == null
                     || handler is TestSynchronizedBreakExit.Handler && !(state as TestSynchronizedBreakExit.Finishing).isCompleting) {
                     if (!this.add(state, node)) {
                        continue
                     }

                     if (rootCause == null) {
                        return node
                     }

                     handle = node
                  }
                  return@label60
               }
            }
         }

         if (rootCause != null) {
            return handle
         }

         if (this.add(state, node)) {
            return node
         }
      }
   }

   public class Finishing(rootCause: Any?, isCompleting: Boolean) {
      @JvmField
      @Nullable
      public final var rootCause: Any?

      @JvmField
      public final var isCompleting: Boolean

      init {
         this.rootCause = rootCause
         this.isCompleting = isCompleting
      }
   }

   public interface Handler

   public class Oops : RuntimeException()
}
