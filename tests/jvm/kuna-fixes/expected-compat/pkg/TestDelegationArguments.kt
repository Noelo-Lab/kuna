package pkg

public class TestDelegationArguments(text: String, size: Int) : DelegationBase {
   init {
      val var10001: java.lang.String = DelegationJava.label(text)
      super(var10001, size)
   }

   public constructor(raw: String)  {
      val var10001: java.lang.String = DelegationJava.label(raw)
      this(var10001, 0)
   }

   public constructor(size: Int, raw: String)  {
      val var10001: java.lang.String = DelegationJava.label(raw)
      this(var10001, size)
   }

   public constructor(raw: String, tag: String) : this("${DelegationJava.label(raw)}${DelegationJava.label(tag)}", 2)
   public constructor(raw: String, tag: String, size: Int)  {
      val var10001: java.lang.String = DelegationJava.count()
      this(var10001, DelegationJava.label(raw).length() + tag.length() + size)
   }

   public constructor(raw: String, size: Int, tag: String)  {
      val var10001: java.lang.String = DelegationJava.label(raw)
      this(var10001, DelegationJava.label(tag).length() + size)
   }
}
