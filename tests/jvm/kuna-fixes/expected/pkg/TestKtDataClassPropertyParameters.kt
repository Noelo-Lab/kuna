package pkg

public class TestKtDataClassPropertyParameters {
   public fun use(h: pkg.TestKtDataClassPropertyParameters.Header): Int {
      return h.size + h.value
   }

   public data class Defaults(public final val a: Int = 1, public final val b: String? = null)

   public data class Entry<A>(private final val first: A, internal final val rest: List<A>) {
      public fun head(): A {
         return this.first
      }
   }

   public data class Header(public final val name: String, public final var value: Int) {
      public final val size: Int

      init {
         this.size = this.name.length + this.value
      }
   }
}
