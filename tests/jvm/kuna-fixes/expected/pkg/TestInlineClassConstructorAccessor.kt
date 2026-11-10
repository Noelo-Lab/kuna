package pkg

public class TestInlineClassConstructorAccessor {
   public class Closed(start: pkg.TestInlineClassConstructorAccessor.Meters, end: pkg.TestInlineClassConstructorAccessor.Meters) : TestInlineClassConstructorAccessor.Range(
         start, end, 1
      )

   @JvmInline
   public value class Meters {
      public final val v: Int

      @JvmStatic
      public open fun toString(): String {
         return "Meters(v=$arg0)"
      }

      fun toString(): String {
         return toString_impl/* $VF was: toString-impl */(this.v)
      }

      @JvmStatic
      public open fun hashCode(): Int {
         return Integer.hashCode(arg0)
      }

      fun hashCode(): Int {
         return hashCode_impl/* $VF was: hashCode-impl */(this.v)
      }

      @JvmStatic
      public open operator fun equals(other: Any?): Boolean {
         return other is TestInlineClassConstructorAccessor.Meters
            && arg0 == (other as TestInlineClassConstructorAccessor.Meters).unbox_impl/* $VF was: unbox-impl */()
         }

      fun equals(other: Any): Boolean {
         return equals_impl/* $VF was: equals-impl */(this.v, other)
      }

      @JvmStatic
      fun `constructor-impl`(v: Int): Int {
         return v
      }

      @JvmStatic
      fun `equals-impl0`(p1: Int, p2: Int): Boolean {
         return p1 == p2
      }
   }

   public open class Range(start: pkg.TestInlineClassConstructorAccessor.Meters, end: pkg.TestInlineClassConstructorAccessor.Meters, step: Int) {
      public final val start: pkg.TestInlineClassConstructorAccessor.Meters
      public final val end: pkg.TestInlineClassConstructorAccessor.Meters
      public final val step: Int

      init {
         this.start = start
         this.end = end
         this.step = step
         if (this.step == 0) {
            throw IllegalArgumentException("step")
         }
      }
   }

   public class Timed<T>(value: T, length: pkg.TestInlineClassConstructorAccessor.Meters) {
      public final val value: T
      public final val length: pkg.TestInlineClassConstructorAccessor.Meters

      init {
         this.value = value as T
         this.length = length
      }
   }
}
