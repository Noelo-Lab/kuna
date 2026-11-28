package pkg

interface NoMetaFeed {
    suspend fun next(): Int
}

interface NoMetaReader {
    fun read(): Int
}

abstract class NoMetaCell(val name: String, val width: Int) {
    abstract fun get(): Int
}

inline fun cell(name: String, crossinline f: () -> Int): NoMetaCell = object : NoMetaCell(name, 4) {
    override fun get(): Int = f() + width
}

inline fun feed(crossinline f: suspend () -> Int): NoMetaFeed = object : NoMetaFeed {
    override suspend fun next(): Int = f() + 1
}

fun named(n: Int, positive: String, negative: String): NoMetaCell =
    cell(if (n > 0) positive else negative) { n * 2 }

fun counting(start: Int, step: Long, src: NoMetaFeed): NoMetaFeed = feed {
    var total = step
    var scale = 0.5
    var ratio = 1.5f
    var seen = false
    val a = src.next()
    total += a
    scale += a
    ratio += a
    seen = a > start
    val b = src.next()
    if (seen) b + total.toInt() + scale.toInt() + ratio.toInt() else b
}

class NoMetaBag(val src: NoMetaReader, val size: Int) {
    fun view(): NoMetaReader = object : NoMetaReader {
        val first: Int = src.read() + size
        override fun read(): Int = first
    }
}
