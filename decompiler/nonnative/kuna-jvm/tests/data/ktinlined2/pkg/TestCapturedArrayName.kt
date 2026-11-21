package pkg

interface ArraySource {
    fun total(): Int
}

@PublishedApi
internal inline fun arraySource(values: Array<Int>, crossinline f: (Int) -> Int): ArraySource = object : ArraySource {
    override fun total(): Int {
        var s = 0
        for (v in values) s += f(v)
        return s
    }
}

fun spread(a: Int, b: Int, c: Int, k: Int): ArraySource = arraySource(arrayOf(a, b, c)) { it * k }
