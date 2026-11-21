package pkg

class TestInlineClassConstructorAccessor {
   @JvmInline
   value class Meters(val v: Int)

   open class Range(val start: Meters, val end: Meters, val step: Int) {
      init {
         if (step == 0) {
            throw IllegalArgumentException("step")
         }
      }
   }

   class Closed(start: Meters, end: Meters) : Range(start, end, 1)

   class Timed<T>(val value: T, val length: Meters)
}
