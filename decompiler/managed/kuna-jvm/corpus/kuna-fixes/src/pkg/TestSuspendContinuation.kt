package pkg

interface SuspendSource {
    suspend fun next(): Int
}

interface SuspendSink {
    suspend fun put(v: Int)
}

suspend fun sumOf(src: SuspendSource, n: Int): Int {
    var total = 0
    for (i in 0 until n) {
        total += src.next()
    }
    return total
}

suspend fun firstPositive(src: SuspendSource, tries: Int): Int? {
    var left = tries
    while (left > 0) {
        val v = src.next()
        if (v > 0) {
            return v
        }
        left--
    }
    return null
}

fun summing(src: SuspendSource): SuspendSink = object : SuspendSink {
    var sum = 0L

    override suspend fun put(v: Int) {
        sum += v + src.next()
    }
}

suspend fun SequenceScope<Int>.yieldTwice(x: Int) {
    yield(x)
    yield(x + 1)
}
