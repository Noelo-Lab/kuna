package pkg

import kotlin.jvm.internal.Intrinsics

public class TestKtLegacyFunctionChecks(prefix: String) {
   private final val prefix: String

   public final var count: Int
      internal set

   init {
      Intrinsics.checkParameterIsNotNull(prefix, "prefix")
      super()
      this.prefix = prefix
   }

   public fun join(first: String, second: String?, rest: List<String>): String {
      Intrinsics.checkParameterIsNotNull(first, "first")
      Intrinsics.checkParameterIsNotNull(rest, "rest")
      return "${this.prefix}$first$second${rest.size()}"
   }

   public fun pick(a: String, b: String): String {
      Intrinsics.checkParameterIsNotNull(a, "a")
      Intrinsics.checkParameterIsNotNull(b, "b")
      return if ((a as java.lang.CharSequence).length() == 0) b else a
   }

   public inline fun <T> measure(label: String, block: () -> T): T {
      Intrinsics.checkParameterIsNotNull(label, "label")
      Intrinsics.checkParameterIsNotNull(block, "block")
      val result: Any = block()
      this.record(label)
      return (T)result
   }

   public fun record(label: String) {
      Intrinsics.checkParameterIsNotNull(label, "label")
      this.count = this.count + label.length()
   }

   public fun String.shout(times: Int): String {
      Intrinsics.checkParameterIsNotNull(`$this$shout`, "$this$shout")
      var res: java.lang.String = `$this$shout`

      repeat(times) { i ->
         res = "$res!"
      }

      return res
   }

   public fun late(first: String, second: Any?): Int {
      Intrinsics.checkParameterIsNotNull(first, "first")
      val n: Int = first.length()
      Intrinsics.checkParameterIsNotNull(second, "second")
      return n
   }

   private fun hidden(text: String): Int {
      return text.length()
   }

   public fun total(texts: Array<String>): Int {
      Intrinsics.checkParameterIsNotNull(texts, "texts")
      var sum: Int = 0

      for (t in texts) {
         sum += this.hidden(t)
      }

      return sum
   }

   public interface Formatter {
      public abstract fun format(value: String, width: Int): String {
      }
   }

   public class Padded : TestKtLegacyFunctionChecks.Formatter {
      public override fun format(value: String, width: Int): String {
         Intrinsics.checkParameterIsNotNull(value, "value")
         var text: java.lang.String = value

         while (text.length() < width) {
            text = " $text"
         }

         return text
      }
   }
}
