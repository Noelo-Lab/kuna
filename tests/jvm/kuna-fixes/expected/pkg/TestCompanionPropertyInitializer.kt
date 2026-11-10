package pkg

import java.util.LinkedHashMap
import org.jetbrains.annotations.NotNull

public class TestCompanionPropertyInitializer(id: Int) {
   public final val id: Int

   init {
      this.id = id
   }

   public companion object {
      public final val INSTANCES: MutableMap<String, TestCompanionPropertyInitializer> = LinkedHashMap() as java.util.Map
      public final val FIRST: TestCompanionPropertyInitializer = TestCompanionPropertyInitializer(1)

      @JvmField
      @NotNull
      public final val SECOND: TestCompanionPropertyInitializer = TestCompanionPropertyInitializer(2)

      public final var created: Int = TestCompanionPropertyInitializer.INSTANCES.size + 7
      private final val names: Array<String> = arrayOf("a", "b")
      public const val LIMIT: Int = 64

      public fun name(i: Int): String {
         return TestCompanionPropertyInitializer.names[i]
      }
   }
}
