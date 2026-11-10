package pkg

public class TestSupertypeTypeArguments<E>(seed: E) {
   private final val seed: E

   init {
      this.seed = seed as E
   }

   public fun make(): pkg.TestSupertypeTypeArguments.Seeded {
      return TestSupertypeTypeArguments.Seeded()
   }

   public open class Box<T>(value: T, tag: String) {
      public final val value: T
      public final val tag: String

      init {
         this.value = value as T
         this.tag = tag
      }
   }

   public open inner class Holder<X>(held: X) {
      public final val held: X

      init {
         this.held = held as X
      }
   }

   public class IntBox(v: Int) : TestSupertypeTypeArguments.Box<Int>(v, "int")

   public class Keyed<K, V>(key: K, other: V) : TestSupertypeTypeArguments.Box<K>(key as K, "keyed") {
      public final val other: V

      init {
         this.other = other as V
      }
   }

   public inner class Seeded : TestSupertypeTypeArguments.Holder<E>(this@TestSupertypeTypeArguments.seed)
}
