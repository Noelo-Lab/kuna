package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestExtensionReceiver.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestExtensionReceiver.kt\npkg/TestExtensionReceiver\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n*L\n1#1,28:1\n1#2:29\n*E\n"])
public class TestExtensionReceiver(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun Int.scaled(): Int {
      return `$this$scaled` * this.base + this.base
   }

   public fun run(): Int {
      return this.scaled(3) + TestExtensionReceiver.Inner(2).next(5)
   }

   public companion object {
      public fun String.twice(): String {
         return "$`$this$twice`$`$this$twice`"
      }
   }

   public inner class Inner(step: Int) {
      public final val step: Int

      init {
         this.step = step
      }

      public fun Int.next(): Int {
         return `$this$next` + this.step + TestExtensionReceiver.this.base
      }
   }
}
