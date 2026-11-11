package pkg

public class TestKtInlineClassValue {
   public final var total: pkg.TestKtInlineClassValue.Meters
      internal set

   public fun add(a: pkg.TestKtInlineClassValue.Meters, b: pkg.TestKtInlineClassValue.Meters): pkg.TestKtInlineClassValue.Meters {
      return TestKtInlineClassValue.Meters.plus_HWDY4ac/* $VF was: plus-HWDY4ac */(a, b)
   }

   public fun zero(a: pkg.TestKtInlineClassValue.Meters): Boolean {
      return TestKtInlineClassValue.Meters.isZero_impl/* $VF was: isZero-impl */(a)
   }

   public fun accumulate(xs: DoubleArray): pkg.TestKtInlineClassValue.Meters {
      var sum: Double = TestKtInlineClassValue.Meters.constructor_impl/* $VF was: constructor-impl */(0.0)

      for (x in xs) {
         sum = TestKtInlineClassValue.Meters.plus_HWDY4ac/* $VF was: plus-HWDY4ac */(
            sum, TestKtInlineClassValue.Meters.constructor_impl/* $VF was: constructor-impl */(x)
         )
      }

      return sum
   }

   public fun store(m: pkg.TestKtInlineClassValue.Meters) {
      this.total = TestKtInlineClassValue.Meters.plus_HWDY4ac/* $VF was: plus-HWDY4ac */(this.total, half)
   }

   public fun boxed(m: pkg.TestKtInlineClassValue.Meters): Any {
      return TestKtInlineClassValue.Meters.box_impl/* $VF was: box-impl */(m)
   }

   public fun same(a: pkg.TestKtInlineClassValue.Meters, b: pkg.TestKtInlineClassValue.Meters): Boolean {
      return TestKtInlineClassValue.Meters.equals_impl0/* $VF was: equals-impl0 */(a, b)
   }

   public fun twice(m: pkg.TestKtInlineClassValue.Meters): Double {
      return m * 2
   }

   public fun show(m: pkg.TestKtInlineClassValue.Meters) {
      System.out.println(m)
   }

   public fun text(m: pkg.TestKtInlineClassValue.Meters): String {
      return "len $m"
   }

   public fun describe(m: pkg.TestKtInlineClassValue.Meters): String {
      return "${TestKtInlineClassValue.Meters.toString_impl/* $VF was: toString-impl */(m)}${TestKtInlineClassValue.Meters.hashCode_impl/* $VF was: hashCode-impl */(
         m
      )}"
   }

   public fun greetBoth(n: pkg.TestKtInlineClassValue.Name, o: pkg.TestKtInlineClassValue.Name?): String {
      val var10000: java.lang.String = TestKtInlineClassValue.Name.greet_impl/* $VF was: greet-impl */(n)
      if (o != null) {
         val var10001: java.lang.String = TestKtInlineClassValue.Name.greet_impl/* $VF was: greet-impl */(o)
         if (var10001 != null) {
            return "$var10000$var10001"
         }
      }

      return "$var10000"
   }

   public fun local(a: Double): Double {
      val m: Double = TestKtInlineClassValue.Meters.constructor_impl/* $VF was: constructor-impl */(a)
      return TestKtInlineClassValue.Meters.scaled_6rxm0mU/* $VF was: scaled-6rxm0mU */(m, 3.0) + m
   }

   public fun pick(c: Boolean, a: pkg.TestKtInlineClassValue.Meters, b: pkg.TestKtInlineClassValue.Meters): Double {
      return if (c) a else b
   }

   @JvmInline
   public value class Meters {
      public final val v: Double

      @JvmStatic
      public operator fun plus(other: pkg.TestKtInlineClassValue.Meters): pkg.TestKtInlineClassValue.Meters {
         return constructor_impl/* $VF was: constructor-impl */(arg0 + other)
      }

      @JvmStatic
      public fun isZero(): Boolean {
         return arg0 == 0.0
      }

      @JvmStatic
      public fun scaled(k: Double): pkg.TestKtInlineClassValue.Meters {
         return constructor_impl/* $VF was: constructor-impl */(arg0 * k)
      }

      public final val half: pkg.TestKtInlineClassValue.Meters
         public final get() {
            return constructor_impl/* $VF was: constructor-impl */(arg0 / (double)2)
         }


      @JvmStatic
      public open fun toString(): String {
         return "Meters(v=$arg0)"
      }

      fun toString(): java.lang.String {
         toString_impl/* $VF was: toString-impl */(this.v)
      }

      @JvmStatic
      public open fun hashCode(): Int {
         return java.lang.Double.hashCode(arg0)
      }

      fun hashCode(): Int {
         hashCode_impl/* $VF was: hashCode-impl */(this.v)
      }

      @JvmStatic
      public open operator fun equals(other: Any?): Boolean {
         return other is TestKtInlineClassValue.Meters
            && java.lang.Double.compare(arg0, (other as TestKtInlineClassValue.Meters).unbox_impl/* $VF was: unbox-impl */()) == 0
         }

      fun equals(other: Any): Boolean {
         equals_impl/* $VF was: equals-impl */(this.v, other)
      }

      @JvmStatic
      fun `constructor-impl`(v: Double): Double {
         v
      }

      @JvmStatic
      fun `equals-impl0`(p1: Double, p2: Double): Boolean {
         java.lang.Double.compare(p1, p2) == 0
      }
   }

   @JvmInline
   public value class Name {
      public final val s: String

      @JvmStatic
      public fun greet(): String {
         return "hi $arg0"
      }

      @JvmStatic
      public open fun toString(): String {
         return "Name(s=$arg0)"
      }

      fun toString(): java.lang.String {
         toString_impl/* $VF was: toString-impl */(this.s)
      }

      @JvmStatic
      public open fun hashCode(): Int {
         return arg0.hashCode()
      }

      fun hashCode(): Int {
         hashCode_impl/* $VF was: hashCode-impl */(this.s)
      }

      @JvmStatic
      public open operator fun equals(other: Any?): Boolean {
         return other is TestKtInlineClassValue.Name && arg0 == (other as TestKtInlineClassValue.Name).unbox_impl/* $VF was: unbox-impl */()
      }

      fun equals(other: Any): Boolean {
         equals_impl/* $VF was: equals-impl */(this.s, other)
      }

      @JvmStatic
      fun `constructor-impl`(s: java.lang.String): java.lang.String {
         s
      }

      @JvmStatic
      fun `equals-impl0`(p1: java.lang.String, p2: java.lang.String): Boolean {
         p1 == p2
      }
   }
}
