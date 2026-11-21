package pkg

public class TestMemberExtensionProperty(scale: Int) {
   private final val scale: Int

   init {
      this.scale = scale
   }

   private final val scaled: Int
      private final get() {
         return `$this$scaled` * this.scale
      }


   public fun total(values: List<Int>): Int {
      var sum: Int = 0
      val var3: java.util.Iterator = values.iterator()

      while (var3.hasNext()) {
         sum += this.scaled
      }

      return sum
   }
}
