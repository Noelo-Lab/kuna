package pkg

public class TestDefaultArgsBridgeLookup(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   @JvmName(name = "scaled")
   public fun scale(x: Int, factor: Int = this.base): Int {
      return x * factor
   }

   internal fun shift(x: Int, by: Int = x + 1): Int {
      return x + by
   }

   public fun walk(d: pkg.TestDefaultArgsBridgeLookup.Meters, steps: Int = 2): Int {
      return d + steps + this.base
   }

   public suspend fun load(key: String = "k", times: Int = key.length): String {
      return "$key$times"
   }

   @JvmName(name = "pickInts")
   public fun pick(xs: List<Int>, n: Int = 1): Int {
      return xs.size + n
   }

   public fun pick(xs: List<String>, n: Int = 2): Int {
      return xs.size * n
   }

   public fun <A : pkg.TestDefaultArgsBridgeLookup.Sink> A.emit(text: String, sep: String = ", "): A {
      this.put(text)
      this.put(sep)
      return this as A
   }

   public companion object {
      @JvmName(name = "of")
      public fun make(size: Int = 4, label: String = "box"): TestDefaultArgsBridgeLookup {
         return TestDefaultArgsBridgeLookup(size + label.length)
      }
   }

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
         return other is TestDefaultArgsBridgeLookup.Meters && arg0 == (other as TestDefaultArgsBridgeLookup.Meters).unbox_impl/* $VF was: unbox-impl */()
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

   public interface Shape {
      public abstract fun area(scale: Int = 2): Int

      public open fun describe(name: String = "shape", width: Int = 3): String {
      }

      // $VF: Class flags could not be determined
      internal class DefaultImpls {
         @JvmStatic
         fun describe(`$this`: TestDefaultArgsBridgeLookup.Shape, name: String, width: Int): String {
            return "$name$width"
         }
      }
   }

   public interface Sink {
      public abstract fun put(s: String)
   }
}
