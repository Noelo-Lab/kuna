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

   public constructor(raw: String, first: String, size: Long) : this(DelegationJava.label(first), DelegationJava.label(raw), size.toInt())

   public constructor(n: Int, raw: String) : this(raw, DelegationJava.label(raw), n + 1)
}
