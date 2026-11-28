package pkg

class TestConstructorBodyBeyondFirstBlock {
   open class Base(val n: Int)

   class Checked(n: Int) : Base(n) {
      init {
         if (n > 5) {
            throw IllegalStateException("big")
         }
      }
   }

   class Pair(val a: Int, val b: Int) {
      constructor(a: Int) : this(a, a) {
         if (a < 0) {
            throw IllegalArgumentException("negative")
         }
      }
   }

   class Plain(n: Int) : Base(n)
}
