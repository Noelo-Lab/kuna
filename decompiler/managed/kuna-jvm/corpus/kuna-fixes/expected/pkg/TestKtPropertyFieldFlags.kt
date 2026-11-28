package pkg

import org.jetbrains.annotations.Nullable

public class TestKtPropertyFieldFlags {
   @Volatile
   public final var state: Any? = null

   @Volatile
   private final var counter: Int = 0

   @Transient
   public final var cache: String? = null

   @JvmField
   @Nullable
   @Volatile
   public final var shared: Any? = null

   @Transient
   private final val memo: IntArray = IntArray(4)

   @delegate:Transient
   public final val label: String by LazyKt.lazy({ 
      "label"
   })
      public get() {
         return this.`label$delegate`.getValue() as String
      }


   @JvmSynthetic
   public final var synthetic: Int = 0

   public final var plain: Int = 0

   public fun bump(): Int {
      this.counter++
      this.memo[0] = this.counter
      return this.counter
   }

   public companion object {
      @Volatile
      private final var instance: TestKtPropertyFieldFlags? = null

      public fun get(): TestKtPropertyFieldFlags? {
         return TestKtPropertyFieldFlags.instance
      }
   }

   public object Registry {
      @Volatile
      public final var current: String? = null
   }
}
