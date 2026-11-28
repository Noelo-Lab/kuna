package pkg

import kotlin.jvm.internal.Intrinsics

public class TestKtLegacyFunctionChecks(prefix: String) {
   private final val prefix: String
   public final var count: Int = 0

   init {
      this.prefix = prefix
   }

   public fun join(first: String, second: String?, rest: List<String>): String {
      return "${this.prefix}$first$second${rest.size}"
   }

   public fun pick(a: String, b: String): String {
      return if ((a as CharSequence).length == 0) b else a
   }

   public inline fun <T> measure(label: String, block: () -> T): T {
      val result: Any = block()
      this.record(label)
      return result as T
   }

   public fun record(label: String) {
      this.count = this.count + label.length
   }

   public fun String.shout(times: Int): String {
      var res: String = this

      repeat(times) { i ->
         res = "$res!"
      }

      return res
   }

   public fun late(first: String, second: Any?): Int {
      val n: Int = first.length
      Intrinsics.checkParameterIsNotNull(second, "second")
      return n
   }

   private fun hidden(text: String): Int {
      return text.length
   }

   public fun total(texts: Array<String>): Int {
      var sum: Int = 0

      for (t in texts) {
         sum += this.hidden(t)
      }

      return sum
   }

   public interface Formatter {
      public abstract fun format(value: String, width: Int): String
   }

   public class Padded : TestKtLegacyFunctionChecks.Formatter {
      public override fun format(value: String, width: Int): String {
         var text: String = value

         while (text.length < width) {
            text = " $text"
         }

         return text
      }
   }
}
