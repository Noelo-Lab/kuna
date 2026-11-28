package pkg

public class TestKtInlineClassValue {
   public final var total: pkg.TestKtInlineClassValue.Meters = TestKtInlineClassValue.Meters(0.0)

   public fun add(a: pkg.TestKtInlineClassValue.Meters, b: pkg.TestKtInlineClassValue.Meters): pkg.TestKtInlineClassValue.Meters {
      return a.plus(b)
   }

   public fun zero(a: pkg.TestKtInlineClassValue.Meters): Boolean {
      return a.isZero()
   }

   public fun accumulate(xs: DoubleArray): pkg.TestKtInlineClassValue.Meters {
      var sum: TestKtInlineClassValue.Meters = TestKtInlineClassValue.Meters(0.0)

      for (x in xs) {
         sum = sum.plus(TestKtInlineClassValue.Meters(x))
      }

      return sum
   }

   public fun store(m: pkg.TestKtInlineClassValue.Meters) {
      this.total = this.total.plus(m.half)
   }

   public fun boxed(m: pkg.TestKtInlineClassValue.Meters): Any {
      return m
   }

   public fun same(a: pkg.TestKtInlineClassValue.Meters, b: pkg.TestKtInlineClassValue.Meters): Boolean {
      return a == b
   }

   public fun twice(m: pkg.TestKtInlineClassValue.Meters): Double {
      return m.v * 2
   }

   public fun show(m: pkg.TestKtInlineClassValue.Meters) {
      System.out.println(m.v)
   }

   public fun text(m: pkg.TestKtInlineClassValue.Meters): String {
      return "len ${m.v}"
   }

   public fun describe(m: pkg.TestKtInlineClassValue.Meters): String {
      return "${m.toString()}${m.hashCode()}"
   }

   public fun greetBoth(n: pkg.TestKtInlineClassValue.Name, o: pkg.TestKtInlineClassValue.Name?): String {
      val var10000: String = n.greet()
      if (o != null) {
         val var10001: String = o.greet()
         if (var10001 != null) {
            return "$var10000$var10001"
         }
      }

      return "$var10000"
   }

   public fun local(a: Double): Double {
      val m: TestKtInlineClassValue.Meters = TestKtInlineClassValue.Meters(a)
      return m.scaled(3.0).v + m.v
   }

   public fun pick(c: Boolean, a: pkg.TestKtInlineClassValue.Meters, b: pkg.TestKtInlineClassValue.Meters): Double {
      return (if (c) a else b).v
   }

   @JvmInline
   public value class Meters {
      public final val v: Double

      @JvmStatic
      public operator fun plus(other: pkg.TestKtInlineClassValue.Meters): pkg.TestKtInlineClassValue.Meters {
         return TestKtInlineClassValue.Meters(arg0.v + other.v)
      }

      @JvmStatic
      public fun isZero(): Boolean {
         return arg0.v == 0.0
      }

      @JvmStatic
      public fun scaled(k: Double): pkg.TestKtInlineClassValue.Meters {
         return TestKtInlineClassValue.Meters(arg0.v * k)
      }

      public final val half: pkg.TestKtInlineClassValue.Meters
         public get() {
            return TestKtInlineClassValue.Meters(arg0.v / 2.toDouble())
         }


      @JvmStatic
      public open fun toString(): String {
         return "Meters(v=${arg0.v})"
      }

      fun toString(): String {
         return toString_impl/* $VF was: toString-impl */(this.v)
      }

      @JvmStatic
      public open fun hashCode(): Int {
         return java.lang.Double.hashCode(arg0.v)
      }

      fun hashCode(): Int {
         return hashCode_impl/* $VF was: hashCode-impl */(this.v)
      }

      @JvmStatic
      public open operator fun equals(other: Any?): Boolean {
         return other is TestKtInlineClassValue.Meters && java.lang.Double.compare(arg0.v, (other as TestKtInlineClassValue.Meters).v) == 0
      }

      fun equals(other: Any): Boolean {
         return equals_impl/* $VF was: equals-impl */(this.v, other)
      }

      @JvmStatic
      fun `constructor-impl`(v: Double): Double {
         return v
      }

      @JvmStatic
      fun `equals-impl0`(p1: Double, p2: Double): Boolean {
         return java.lang.Double.compare(p1, p2) == 0
      }
   }

   @JvmInline
   public value class Name {
      public final val s: String

      @JvmStatic
      public fun greet(): String {
         return "hi ${arg0.s}"
      }

      @JvmStatic
      public open fun toString(): String {
         return "Name(s=${arg0.s})"
      }

      fun toString(): String {
         return toString_impl/* $VF was: toString-impl */(this.s)
      }

      @JvmStatic
      public open fun hashCode(): Int {
         return arg0.s.hashCode()
      }

      fun hashCode(): Int {
         return hashCode_impl/* $VF was: hashCode-impl */(this.s)
      }

      @JvmStatic
      public open operator fun equals(other: Any?): Boolean {
         return other is TestKtInlineClassValue.Name && arg0.s == (other as TestKtInlineClassValue.Name).s
      }

      fun equals(other: Any): Boolean {
         return equals_impl/* $VF was: equals-impl */(this.s, other)
      }

      @JvmStatic
      fun `constructor-impl`(s: String): String {
         return s
      }

      @JvmStatic
      fun `equals-impl0`(p1: String, p2: String): Boolean {
         return p1 == p2
      }
   }
}
