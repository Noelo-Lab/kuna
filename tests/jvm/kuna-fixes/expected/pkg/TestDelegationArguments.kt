package pkg

public class TestDelegationArguments(text: String, size: Int) : DelegationBase(DelegationJava.label(text), size) {
   public constructor(raw: String) : this(DelegationJava.label(raw), 0)

   public constructor(size: Int, raw: String) : this(DelegationJava.label(raw), size)

   public constructor(raw: String, tag: String) : this("${DelegationJava.label(raw)}${DelegationJava.label(tag)}", 2)

   public constructor(raw: String, tag: String, size: Int) : this(DelegationJava.count(), DelegationJava.label(raw).length() + tag.length() + size)

   public constructor(raw: String, size: Int, tag: String) : this(DelegationJava.label(raw), DelegationJava.label(tag).length() + size)
}
