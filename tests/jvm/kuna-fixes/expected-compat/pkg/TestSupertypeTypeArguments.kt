package pkg

public class TestSupertypeTypeArguments<E>(seed: Any) {
   private final val seed: Any

   init {
      this.seed = (E)seed
   }

   public fun make(): pkg.TestSupertypeTypeArguments.Seeded {
      return TestSupertypeTypeArguments.Seeded()
   }

   public open class Box<T>(value: Any, tag: String) {
      public final val value: Any
      public final val tag: String

      init {
         this.value = (T)value
         this.tag = tag
      }
   }

   public open inner class Holder<X>(held: Any) {
      public final val held: Any

      init {
         this.held = (X)held
      }
   }

   public class IntBox(v: Int) : TestSupertypeTypeArguments.Box(v, "int")

   public class Keyed<K, V>(key: Any, other: Any) : TestSupertypeTypeArguments.Box((K)key, "keyed") {
      public final val other: Any

      init {
         this.other = (V)other
      }
   }

   public inner class Seeded : TestSupertypeTypeArguments.Holder(TestSupertypeTypeArguments.this.seed)
}
