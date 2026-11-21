package pkg

public class TestMemberExtensionProperty(scale: Int) {
   private final val scale: Int

   init {
      this.scale = scale
   }

   private final val Int.scaled: Int
      private get() {
         return this * this@TestMemberExtensionProperty.scale
      }


   public fun total(values: List<Int>): Int {
      var sum: Int = 0
      val var3: Iterator<Int> = values.iterator()

      while (var3.hasNext()) {
         sum += (var3.next() as Number).toInt().scaled
      }

      return sum
   }
}
