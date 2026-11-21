package pkg

import org.jetbrains.annotations.Nullable

public class TestKtPropertyFieldFlags {
   public final var state: Any?
      internal set

   private final var counter: Int

   public final var cache: String?
      internal set

   @JvmField
   @Nullable
   public final var shared: Any?
      private set

   private final val memo: IntArray = IntArray(4)

   public final val label: String by LazyKt.lazy({ 
      "label"
   })
      public final get() {
         return this.label$delegate.getValue() as java.lang.String
      }


   public final var synthetic: Int
      internal set

   public final var plain: Int
      internal set

   public fun bump(): Int {
      this.counter++
      this.memo[0] = this.counter
      return this.counter
   }

   public companion object {
      private final var instance: TestKtPropertyFieldFlags?

      public fun get(): TestKtPropertyFieldFlags? {
         return TestKtPropertyFieldFlags.instance
      }
   }

   public object Registry {
      public final var current: String?
         internal set
   }
}
