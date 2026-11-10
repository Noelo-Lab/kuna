package pkg

public open class TestDefaultAccessorFlags {
   public final var plain: Int

   public final var privateSet: Int
      private set

   public final var syntheticSet: Int
   protected final var prot: Int
   public open var openVar: Int
   public final lateinit var late: String

   public object Counter {
      @JvmStatic
      public TestDefaultAccessorFlags.Counter INSTANCE = TestDefaultAccessorFlags.Counter();
      public final var count: Long
   }
}
