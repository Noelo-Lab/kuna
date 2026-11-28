package pkg

class TestSecondaryConstructorLine(val n: Int, val s: String) {
    constructor(n: Int) : this(n, "n")

    open class Base(val text: String) {
        constructor(code: Int) : this(code.toString())

        fun size(): Int = text.length

        constructor() : this(0)
    }
}
