package pkg

public class TestKtEmptyPrimaryConstructor() {
   private final var executor: Any? = null
   public final var calls: Int = 0

   public constructor(executor: Any) : this() {
      this.executor = executor
   }

   public class Builder() {
      public final var name: String = "x"

      internal constructor(other: TestKtEmptyPrimaryConstructor) : this() {
         this.name = java.lang.String.valueOf(other.calls)
      }
   }
}
