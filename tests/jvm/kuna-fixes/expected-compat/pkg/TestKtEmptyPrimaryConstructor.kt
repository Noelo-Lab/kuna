package pkg

public class TestKtEmptyPrimaryConstructor {
   private final var executor: Any?

   public final var calls: Int
      internal set

   public constructor(executor: Any) : this() {
      this.executor = executor
   }

   public class Builder {
      public final var name: String = "x"
         internal set

      internal constructor(other: TestKtEmptyPrimaryConstructor) : this() {
         this.name = java.lang.String.valueOf(other.calls)
      }
   }
}
