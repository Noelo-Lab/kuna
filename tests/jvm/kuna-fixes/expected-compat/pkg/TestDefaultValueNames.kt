package pkg

public class TestDefaultValueNames(length: Int) {
   public final val length: Int

   init {
      this.length = length
   }

   public fun String.pad(width: Int = this.length): String {
      return "$`$this$pad`$width"
   }

   public fun String.cut(n: Int = `$this$cut`.length() - 1, keep: Boolean = n > 0): String {
      return if (keep) "$`$this$cut`$n" else `$this$cut`
   }

   public fun twice(a: Int, b: Int = a * 2): Int {
      return a + b
   }

   public interface Shape {
      public abstract fun scaled(base: Int, by: Int = ...): Int {
      }

      // $VF: Class flags could not be determined
      internal class DefaultImpls
   }
}
