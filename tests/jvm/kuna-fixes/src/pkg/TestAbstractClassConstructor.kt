package pkg

class TestAbstractClassConstructor {
   abstract class Shape(val name: String) {
      abstract fun area(): Int
   }

   abstract class Counter protected constructor() {
      var count = 0

      abstract fun step(): Int
   }

   abstract class Tagged(val arity: Int, val tag: String?) {
      constructor(arity: Int) : this(arity, null)
   }

   abstract class Checked(val limit: Int) : Shape("checked") {
      init {
         if (limit < 0) {
            throw IllegalArgumentException("negative")
         }
      }
   }

   class Square(val side: Int) : Shape("square") {
      override fun area(): Int = side * side
   }
}
