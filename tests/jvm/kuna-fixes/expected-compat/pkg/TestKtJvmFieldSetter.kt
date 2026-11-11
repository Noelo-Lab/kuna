package pkg

import org.jetbrains.annotations.NotNull
import org.jetbrains.annotations.Nullable

public open class TestKtJvmFieldSetter {
   @JvmField
   public final var count: Int
      private set

   @JvmField
   @NotNull
   internal final var name: String = ""
      private set

   @JvmField
   @Nullable
   protected final var tag: Any?
      private set

   @JvmField
   public final val fixed: Int = 1

   public final var hidden: Int
      private set

   internal final var total: Long
      private set

   public final lateinit var label: String
      private set

   private final var own: Int

   public fun touch(): Long {
      this.hidden++
      this.total = this.total + this.hidden
      this.own++
      this.label = "x"
      return this.total + this.own
   }

   public companion object {
      @JvmField
      @Nullable
      public final var shared: String?
         private set

      public final var limit: Int
         private set
   }
}
