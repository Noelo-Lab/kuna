package pkg

import java.io.InputStream

class TestUnescapedNames {
   private var count: Int = 0

   fun IntArray.view(): AbstractList<Int> = object : AbstractList<Int>() {
      override val size: Int
         get() = this@view.size

      override fun get(index: Int): Int = this@view[index]
   }

   val plusOne: (Int) -> Int = @JvmSerializableLambda { `in` -> `in` + 1 }

   fun shift(xs: List<Int>): List<Int> = xs.map { `is` -> `is` * 2 }

   fun firsts(pairs: Map<Int, String>): List<Int> = pairs.map { (k, _) -> k }

   fun pickFirst(f: (Int, Int) -> Int): Int = f(3, 4)

   fun first(): Int = pickFirst { a, _ -> a }

   val second: (Int, Int) -> Int = @JvmSerializableLambda { _, b -> b }

   fun stdin(): InputStream = System.`in`

   internal fun bump(): Int {
      count++
      return count
   }

   class Other {
      fun use(t: TestUnescapedNames): Int = t.bump()
   }
}
