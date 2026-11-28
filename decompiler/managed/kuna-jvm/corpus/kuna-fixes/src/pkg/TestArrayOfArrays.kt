package pkg

class TestArrayOfArrays {
   class Cell(val v: Int)

   fun rows(n: Int): Array<ByteArray?> {
      return arrayOfNulls<ByteArray>(n)
   }

   fun grid(n: Int): Array<Array<Cell>?> {
      return arrayOfNulls<Array<Cell>>(n)
   }

   fun table(n: Int): Array<IntArray> {
      return Array(n) { IntArray(it) }
   }
}
