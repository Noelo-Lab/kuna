package pkg

class TestKtInlineClassValue {
   @JvmInline
   value class Meters(val v: Double) {
      operator fun plus(other: Meters): Meters = Meters(v + other.v)

      fun isZero(): Boolean = v == 0.0

      fun scaled(k: Double): Meters = Meters(v * k)

      val half: Meters
         get() = Meters(v / 2)
   }

   @JvmInline
   value class Name(val s: String) {
      fun greet(): String = "hi " + s
   }

   var total: Meters = Meters(0.0)

   fun add(a: Meters, b: Meters): Meters = a + b

   fun zero(a: Meters): Boolean = a.isZero()

   fun accumulate(xs: DoubleArray): Meters {
      var sum = Meters(0.0)
      for (x in xs) {
         sum = sum + Meters(x)
      }
      return sum
   }

   fun store(m: Meters) {
      total = total + m.half
   }

   fun boxed(m: Meters): Any = m

   fun same(a: Meters, b: Meters): Boolean = a == b

   fun twice(m: Meters): Double = m.v * 2

   fun show(m: Meters) = println(m.v)

   fun text(m: Meters): String = "len " + m.v

   fun describe(m: Meters): String = m.toString() + m.hashCode()

   fun greetBoth(n: Name, o: Name?): String = n.greet() + (o?.greet() ?: "")

   fun local(a: Double): Double {
      val m = Meters(a)
      val n = m.scaled(3.0)
      return n.v + m.v
   }

   fun pick(c: Boolean, a: Meters, b: Meters): Double = (if (c) a else b).v
}
