package pkg

public class TestDefaultValueNames(length: Int) {
   public final val length: Int

   init {
      this.length = length
   }

   public fun String.pad(width: Int = this@TestDefaultValueNames.length): String {
      return "$this$width"
   }

   public fun String.cut(n: Int = this.length - 1, keep: Boolean = n > 0): String {
      return if (keep) "$this$n" else this
   }

   public fun twice(a: Int, b: Int = a * 2): Int {
      return a + b
   }

   public interface Shape {
      public abstract fun scaled(base: Int, by: Int = base * 2): Int

      // $VF: Class flags could not be determined
      internal class DefaultImpls
   }
}
