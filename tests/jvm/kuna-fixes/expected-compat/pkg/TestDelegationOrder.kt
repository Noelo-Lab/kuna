package pkg

public class TestDelegationOrder(a: String, b: String, n: Int) {
   public final val a: String
   public final val b: String
   public final val n: Int

   init {
      this.a = a
      this.b = b
      this.n = n
   }

   public constructor(raw: String, first: String, size: Long)  {
      val var10001: java.lang.String = DelegationJava.label(first)
      val var10002: java.lang.String = DelegationJava.label(raw)
      this(var10001, var10002, (int)size)
   }

   public constructor(n: Int, raw: String)  {
      val var10002: java.lang.String = DelegationJava.label(raw)
      this(raw, var10002, n + 1)
   }
}
