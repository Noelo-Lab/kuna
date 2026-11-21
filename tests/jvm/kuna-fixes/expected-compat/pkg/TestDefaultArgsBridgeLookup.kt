package pkg

public class TestDefaultArgsBridgeLookup(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   @JvmName(name = "scaled")
   public fun scale(x: Int, factor: Int = ...): Int {
      return x * factor
   }

   internal fun shift(x: Int, by: Int = ...): Int {
      return x + by
   }

   public fun walk(d: pkg.TestDefaultArgsBridgeLookup.Meters, steps: Int = ...): Int {
      return d + steps + this.base
   }

   public suspend fun load(key: String = ..., times: Int = ...): String {
      return "$key$times"
   }

   @JvmName(name = "pickInts")
   public fun pick(xs: List<Int>, n: Int = 2): Int {
      return xs.size() + n
   }

   public fun pick(xs: List<String>, n: Int = 2): Int {
      return xs.size() * n
   }

   public fun <A : pkg.TestDefaultArgsBridgeLookup.Sink> A.emit(text: String, sep: String = ...): A {
      `$this$emit`.put(text)
      `$this$emit`.put(sep)
      return (A)`$this$emit`
   }

   public companion object {
      @JvmName(name = "of")
      public fun make(size: Int = ..., label: String = ...): TestDefaultArgsBridgeLookup {
         return TestDefaultArgsBridgeLookup(size + label.length())
      }
   }

   @JvmInline
   public value class Meters {
      public final val v: Int

      @JvmStatic
      public open fun toString(): String {
         return "Meters(v=$arg0)"
      }

      fun toString(): java.lang.String {
         toString_impl/* $VF was: toString-impl */(this.v)
      }

      @JvmStatic
      public open fun hashCode(): Int {
         return Integer.hashCode(arg0)
      }

      fun hashCode(): Int {
         hashCode_impl/* $VF was: hashCode-impl */(this.v)
      }

      @JvmStatic
      public open operator fun equals(other: Any?): Boolean {
         return other is TestDefaultArgsBridgeLookup.Meters && arg0 == (other as TestDefaultArgsBridgeLookup.Meters).unbox_impl/* $VF was: unbox-impl */()
      }

      fun equals(other: Any): Boolean {
         equals_impl/* $VF was: equals-impl */(this.v, other)
      }

      @JvmStatic
      fun `constructor-impl`(v: Int): Int {
         v
      }

      @JvmStatic
      fun `equals-impl0`(p1: Int, p2: Int): Boolean {
         p1 == p2
      }
   }

   public interface Shape {
      public abstract fun area(scale: Int = ...): Int {
      }

      public open fun describe(name: String = ..., width: Int = ...): String {
      }

      // $VF: Class flags could not be determined
      internal class DefaultImpls {
         @JvmStatic
         fun describe(`$this`: TestDefaultArgsBridgeLookup.Shape, name: java.lang.String, width: Int): java.lang.String {
            "$name$width"
         }
      }
   }

   public interface Sink {
      public abstract fun put(s: String) {
      }
   }
}
