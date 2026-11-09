package pkg

class TestVarargFromMetadata(val names: Array<String>, val counts: IntArray) {
    constructor(vararg parts: String) : this(parts as Array<String>, IntArray(parts.size))

    constructor(first: Int, vararg rest: Int) : this(arrayOfNulls<String>(0) as Array<String>, rest)

    fun sum(vararg values: Int): Int {
        var total = 0
        for (v in values) total += v
        return total
    }

    fun count(sep: String, vararg items: Any): Int = items.size + sep.length

    fun first(items: Array<String>): String = items[0]
}
