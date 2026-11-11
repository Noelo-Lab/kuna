package pkg

import org.jetbrains.annotations.NotNull

public class TestCompanionPropertyInitializer(id: Int) {
   public final val id: Int

   init {
      this.id = id
   }

   public companion object {
      public final val INSTANCES: MutableMap<String, TestCompanionPropertyInitializer>
      public final val FIRST: TestCompanionPropertyInitializer

      @JvmField
      @NotNull
      public final val SECOND: TestCompanionPropertyInitializer

      public final var created: Int
         internal set

      private final val names: Array<String>
      public const val LIMIT: Int = 64

      public fun name(i: Int): String {
         return TestCompanionPropertyInitializer.names[i]
      }
   }
}
