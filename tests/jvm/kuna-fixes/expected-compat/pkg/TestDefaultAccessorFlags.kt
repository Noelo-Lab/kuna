package pkg

public open class TestDefaultAccessorFlags {
   public final var plain: Int
      internal set

   public final var privateSet: Int
      private set

   public final var syntheticSet: Int
      internal set

   protected final var prot: Int
      internal set

   public open var openVar: Int
      internal final set

   public final lateinit var late: String
      internal set

   public object Counter {
      @JvmStatic
      public TestDefaultAccessorFlags.Counter INSTANCE = TestDefaultAccessorFlags.Counter();

      public final var count: Long
         internal set
   }
}
