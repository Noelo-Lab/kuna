package pkg

class TestJumpToNonLoopLabel(val items: Array<Item?>) {
   interface Item {
      fun rank(): Int
   }

   var size = 0
   var log = 0

   fun removeAt(index: Int): Item {
      size--
      if (index < size) {
         swap(index, size)
         val j = (index - 1) / 2
         if (index > 0 && items[index]!!.rank() < items[j]!!.rank()) {
            swap(index, j)
            up(j)
         } else {
            down(index)
         }
      }
      return items[size]!!
   }

   fun matches(a: IntArray, b: IntArray): Int {
      var hits = 0
      val n = a.size
      outer@ for (i in 0 until n) {
         val x = a[i]
         val m = b.size
         for (j in 0 until m) {
            if (x == b[j]) {
               hits += i
               continue@outer
            }
         }
      }
      return hits
   }

   fun swap(i: Int, j: Int) {
      log += i * 31 + j
   }

   fun up(i: Int) {
      log += i
   }

   fun down(i: Int) {
      log -= i
   }
}
