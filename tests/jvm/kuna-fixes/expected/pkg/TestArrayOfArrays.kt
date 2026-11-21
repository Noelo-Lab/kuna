package pkg

public class TestArrayOfArrays {
   public fun rows(n: Int): Array<ByteArray?> {
      return arrayOfNulls(n)
   }

   public fun grid(n: Int): Array<Array<pkg.TestArrayOfArrays.Cell>?> {
      return arrayOfNulls(n)
   }

   public fun table(n: Int): Array<IntArray> {
      var var2: Int = 0
      val var3: Array<IntArray> = arrayOfNulls(n)

      while (var2 < n) {
         var3[var2] = IntArray(var2)
         var2++
      }

      return var3
   }

   public class Cell(v: Int) {
      public final val v: Int

      init {
         this.v = v
      }
   }
}
