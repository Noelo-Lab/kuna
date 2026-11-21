package pkg

public open class TestDefaultAccessorFlags {
   public final var plain: Int = 0

   public final var privateSet: Int = 0
      private set

   public final var syntheticSet: Int = 0
   protected final var prot: Int = 0
   public open var openVar: Int = 0
   public final lateinit var late: String

   public object Counter {
      @JvmStatic
      public TestDefaultAccessorFlags.Counter INSTANCE = TestDefaultAccessorFlags.Counter();
      public final var count: Long = 0L
   }
}
