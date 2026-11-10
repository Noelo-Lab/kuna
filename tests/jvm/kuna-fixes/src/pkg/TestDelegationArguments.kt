package pkg

open class DelegationBase(val text: String, val size: Int)

class TestDelegationArguments(text: String, size: Int) : DelegationBase(DelegationJava.label(text), size) {
    constructor(raw: String) : this(DelegationJava.label(raw), 0)

    constructor(size: Int, raw: String) : this(DelegationJava.label(raw), size)

    constructor(raw: String, tag: String) : this(DelegationJava.label(raw) + DelegationJava.label(tag), 2)

    constructor(raw: String, tag: String, size: Int) : this(DelegationJava.count(), DelegationJava.label(raw).length + tag.length + size)

    constructor(raw: String, size: Int, tag: String) : this(DelegationJava.label(raw), DelegationJava.label(tag).length + size)
}

class TestDelegationOrder(val a: String, val b: String, val n: Int) {
    constructor(raw: String, first: String, size: Long) : this(DelegationJava.label(first), DelegationJava.label(raw), size.toInt())

    constructor(n: Int, raw: String) : this(raw, DelegationJava.label(raw), n + 1)
}
