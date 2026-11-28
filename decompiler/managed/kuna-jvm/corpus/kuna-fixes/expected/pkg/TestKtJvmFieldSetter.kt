package pkg

import org.jetbrains.annotations.NotNull
import org.jetbrains.annotations.Nullable

public open class TestKtJvmFieldSetter {
   @JvmField
   public final var count: Int = 0

   @JvmField
   @NotNull
   internal final var name: String = ""

   @JvmField
   @Nullable
   protected final var tag: Any? = null

   @JvmField
   public final val fixed: Int = 1

   public final var hidden: Int = 0
      private set

   internal final var total: Long = 0L
      private set

   public final lateinit var label: String
      private set

   private final var own: Int = 0

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
      public final var shared: String? = null

      public final var limit: Int = 0
         private set
   }
}
